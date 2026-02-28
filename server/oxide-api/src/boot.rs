use outbox_core::prelude::*;
use std::sync::Arc;
use std::time::Duration;
use axum::http::Method;
use axum::Router;
use axum::routing::get;
use outbox_core::prelude::{OutboxConfig};
use outbox_postgres::PostgresOutbox;
use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tracing::{error, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use oxide_business::event::{EventDispatcher, TokyoEventBus};
use oxide_business::profile::handler::ProfileHandler;
use oxide_config::AppConfig;
use oxide_domain::event::{EventHandler, GlobalEvent};
use oxide_infrastructure::outbox::OutboxWatcher;
use oxide_infrastructure::outbox::postgres::PostgresOutboxWatcher;
use crate::openapi::ApiDoc;
use crate::state::AppState;
use crate::user::auth::auth_router;
use crate::user::{admin_router, user_router};

pub async fn create_app(app_state: Arc<AppState>, app_config: Arc<AppConfig>, shutdown_rx: Receiver<bool>) -> anyhow::Result<()> {

    run_outbox(app_state.clone(), app_config.clone(), shutdown_rx).await?;

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            "http://localhost:8080".parse()?,
            "http://127.0.0.1:8080".parse()?,
            "http://127.0.0.1:8085".parse()?,
        ]))
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
            Method::PATCH,
        ]))
        .allow_headers(AllowHeaders::list([
            "content-type".parse()?,
            "authorization".parse()?,
            "accept".parse()?,
            "x-requested-with".parse()?,
            "access-control-allow-origin".parse()?,
        ]))
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    let app = Router::new()
        .nest("/api/v1/auth", auth_router())
        .nest("/api/v1/users", user_router())
        .nest("/api/v1/admin", admin_router())
        .route("/", get(|| async { "Hello, World!" }))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(app_state);
    let addr = format!("{}:{}", app_config.server.host, app_config.server.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}


pub async fn run_outbox(app_state: Arc<AppState>, app_config: Arc<AppConfig>, shutdown_rx: Receiver<bool>) -> anyhow::Result<()> {

    let config = Arc::new(OutboxConfig::<GlobalEvent> {
        batch_size: 100,
        retention_days: 1,
        gc_interval_secs: 10,
        poll_interval_secs: 100,
        lock_timeout_mins: 1,
        idempotency_strategy: IdempotencyStrategy::None,
    });
    let storage = PostgresOutbox::<GlobalEvent>::new(app_state.pool.clone(), config.clone());

    let (sender, receiver) = tokio::sync::mpsc::unbounded_channel::<Event<GlobalEvent>>();
    let publisher = TokioEventPublisher(sender);
    let outbox = OutboxManager::new(
        Arc::new(storage),
        Arc::new(publisher),
        config.clone(),
        shutdown_rx,
    );

    tokio::spawn(async move {
        if let Err(e) = outbox.run().await {
            error!("Outbox critical error: {}", e);
        }
    });

    let mut event_handlers = Vec::<Arc<dyn EventHandler>>::new();
    event_handlers.push(Arc::new(ProfileHandler::new(app_state.clone().profile_repo.clone())));
    let event_dispatcher = EventDispatcher::new(event_handlers);
    tokio::spawn(async move {
        event_dispatcher.run(receiver).await;
    });
    Ok(())
}

#[derive(Clone)]
struct TokioEventPublisher(tokio::sync::mpsc::UnboundedSender<Event<GlobalEvent>>);

#[async_trait::async_trait]
impl Transport<GlobalEvent> for TokioEventPublisher {
    async fn publish(&self, event: Event<GlobalEvent>) -> Result<(), OutboxError> {
        self.0
            .send(event)
            .map_err(|e| OutboxError::InfrastructureError(e.to_string()))
    }
}