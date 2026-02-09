mod handler;
mod dto;
mod openapi;

use std::env;
use std::sync::Arc;
use std::time::Duration;
use axum::http::Method;
use axum::Router;
use axum::routing::{get, post};
use dotenvy::dotenv;
use sqlx::PgPool;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, Any, CorsLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use oxide_data::PostgresContext;
use crate::handler::auth::login;
use crate::handler::user::me;
use crate::openapi::ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    log_startup_banner();
    dotenv().ok();
    info!("Environment value have been set");

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            "http://localhost:8080".parse()?,
            "http://127.0.0.1:8080".parse()?,
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


    let auth_route = Router::new()
        .route("/login", post(login));

    let user_route = Router::new()
        .route("/me", get(me));


    let connection_string = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env or system env");


    let pg_pool = PgPool::connect(connection_string.as_str()).await?;

    let app_state = Arc::new(AppState{
        postgres_context: PostgresContext::new(pg_pool.clone()),
    });


    let app = Router::new()
        .nest("/api/v1/auth", auth_route)
        .nest("/api/v1/users", user_route)
        .route("/", get(|| async { "Hello, World!" }))
        .merge(SwaggerUi::new("/swagger-ui" ).url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors)
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

//#[derive(Clone)]
struct AppState {
    postgres_context: PostgresContext
}

fn log_startup_banner() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    info!("--------------------------------------------------");
    info!("Starting {} v{}", name, version);
    info!("Copyright (c) 2026 Axion Tech LLC");
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