mod dto;
mod openapi;
mod user;
mod state;
mod boot;
mod error;

use crate::openapi::ApiDoc;
use crate::user::auth::{auth_router, login};
use crate::user::{me, user_router};
use axum::Router;
use axum::http::Method;
use axum::routing::{get, post};
use dotenvy::dotenv;
use oxide_business::event::{EventDispatcher, TokyoEventBus};
use oxide_business::user::service::register_admin;
use oxide_data::PostgresContext;
use oxide_domain::crypto::PasswordHasher;
use oxide_domain::user::object::{Email, RawPassword};
use oxide_domain::user::plugin::UserExtensionRegistry;
use oxide_domain::user::repository::UserRepository;
use sqlx::PgPool;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};
use tracing::{error, info, warn};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::boot::create_app;
use crate::error::AppError;
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();
    log_startup_banner();
    dotenv().ok();
    info!("Environment value have been set");

    let (tokio_event_bus, rec) = TokyoEventBus::new();
    let tokio_event_bus = Arc::new(tokio_event_bus);

    let app_state = Arc::new(AppState::new().await?);

    let event_bus = EventDispatcher::new(Vec::new());
    tokio::spawn(async move {
        event_bus.run(rec).await;
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
