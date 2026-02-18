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
use std::str::FromStr;
use std::sync::Arc;
use tracing::{info, warn, Level};
use oxide_business::profile::handler::ProfileHandler;
use oxide_config::{load_config, AppConfig};
use oxide_domain::event::EventHandler;
use oxide_infrastructure::outbox::OutboxWatcher;
use oxide_infrastructure::outbox::postgres::PostgresOutboxWatcher;
use crate::boot::create_app;
use crate::error::AppError;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), AppError> {

    dotenv().ok();
    info!("Environment value have been set");
    let app_config = Arc::new(load_config());


    tracing_subscriber::fmt()
        .with_max_level(Level::from_str(&app_config.server.tracing_level).unwrap_or(Level::INFO))
        .init();
    log_startup_banner();

    let app_state = Arc::new(AppState::new(app_config.clone()).await?);

    if let Err(e) = admin_register(app_state.clone(), app_config.clone()).await {
        warn!("Failed to register admin: {}", e);
    }
    create_app(app_state, app_config).await?;


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

async fn admin_register(app_state: Arc<AppState>, app_config: Arc<AppConfig>) -> anyhow::Result<()> {
    let need_reg = app_config.admin.register;

    if !need_reg {
        info!("The admin will not register");
        return Ok(());
    }
    let email = Email::new(app_config.admin.email.clone())?;
    let password = RawPassword::new(app_config.admin.password.clone())?;
    register_admin(app_state.user_repo.as_ref(), app_state.password_hasher.as_ref(), app_state.user_extension_registry.as_ref(), email, password).await?;
    info!("The admin has been registered");
    Ok(())
}
