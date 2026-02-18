use std::sync::Arc;
use std::time::Duration;
use axum::http::Method;
use axum::Router;
use axum::routing::get;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use oxide_business::event::{EventDispatcher, TokyoEventBus};
use oxide_business::profile::handler::ProfileHandler;
use oxide_config::AppConfig;
use oxide_domain::event::EventHandler;
use oxide_infrastructure::outbox::OutboxWatcher;
use oxide_infrastructure::outbox::postgres::PostgresOutboxWatcher;
use crate::openapi::ApiDoc;
use crate::state::AppState;
use crate::user::auth::auth_router;
use crate::user::user_router;

pub async fn create_app(app_state: Arc<AppState>, app_config: Arc<AppConfig>) -> anyhow::Result<()> {
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
        .route("/", get(|| async { "Hello, World!" }))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(app_state);
    let addr = format!("{}:{}", app_config.server.host, app_config.server.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}


pub async fn run_outbox(app_state: Arc<AppState>, app_config: Arc<AppConfig>) -> anyhow::Result<()> {

    let (tokio_event_bus, rec) = TokyoEventBus::new();
    let tokio_event_bus = Arc::new(tokio_event_bus);

    let postgres_outbox = PostgresOutboxWatcher::new(app_state.pool.clone(), tokio_event_bus.clone());

    let mut event_handlers = Vec::<Arc<dyn EventHandler>>::new();
    event_handlers.push(Arc::new(ProfileHandler::new(app_state.clone().profile_repo.clone())));

    let event_dispatcher = EventDispatcher::new(event_handlers);
    tokio::spawn(async move {
        event_dispatcher.run(rec).await;
    });
    tokio::spawn(async move {
        postgres_outbox.watch().await;
    });
    
    Ok(())
}