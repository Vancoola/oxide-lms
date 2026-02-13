mod dto;
mod openapi;
mod user;
mod state;
mod boot;
mod error;

use dotenvy::dotenv;
use oxide_business::event::{EventDispatcher, TokyoEventBus};
use oxide_business::user::service::register_admin;
use oxide_domain::user::object::{Email, RawPassword};
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use tracing::{info, warn};
use oxide_business::profile::handler::ProfileHandler;
use oxide_domain::event::EventHandler;
use oxide_infrastructure::outbox::OutboxWatcher;
use oxide_infrastructure::outbox::postgres::PostgresOutboxWatcher;
use crate::boot::create_app;
use crate::error::AppError;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    log_startup_banner();
    dotenv().ok();
    info!("Environment value have been set");

    let (tokio_event_bus, rec) = TokyoEventBus::new();
    let tokio_event_bus = Arc::new(tokio_event_bus);
    let connection_string =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or system env");
    let pg_pool = PgPool::connect(connection_string.as_str()).await?;

    let postgres_outbox = PostgresOutboxWatcher::new(pg_pool.clone(), tokio_event_bus.clone());

    let app_state = Arc::new(AppState::new(pg_pool.clone()).await?);

    let mut event_handlers = Vec::<Arc<dyn EventHandler>>::new();
    event_handlers.push(Arc::new(ProfileHandler::new(app_state.clone().profile_repo.clone())));

    let event_dispatcher = EventDispatcher::new(event_handlers);
    tokio::spawn(async move {
        event_dispatcher.run(rec).await;
    });
    tokio::spawn(async move {
        postgres_outbox.watch().await;
    });

    if let Err(e) = admin_register(app_state.clone()).await {
        warn!("Failed to register admin: {}", e);
    }

    create_app(app_state).await?;
    Ok(())
}


fn log_startup_banner() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    info!("--------------------------------------------------");
    info!("Starting {} v{}", name, version);
    info!("Copyright (c) 2026 Oxide-LMS Teams");
    info!("Authors: {}", authors);
    info!(
        "Build Mode: {}",
        if cfg!(debug_assertions) {
            "Debug"
        } else {
            "Release"
        }
    );
    info!("--------------------------------------------------");
}

async fn admin_register(app_state: Arc<AppState>) -> anyhow::Result<()> {
    let need_reg = env::var("ADMIN_REGISTER").map(|_| true).unwrap_or(false);

    if !need_reg {
        info!("The admin will not register");
        return Ok(());
    }

    let env_email = env::var("ADMIN_EMAIL")
        .ok()
        .expect("ADMIN_EMAIL must be set in .env or system env");
    let env_password = env::var("ADMIN_PASSWORD")
        .ok()
        .expect("ADMIN_PASSWORD must be set in .env or system env");

    let email = Email::new(env_email)?;
    let password = RawPassword::new(env_password)?;
    register_admin(app_state.user_repo.as_ref(), app_state.password_hasher.as_ref(), app_state.user_extension_registry.as_ref(), email, password).await?;
    info!("The admin has been registered");
    Ok(())
}
