mod handler;
mod dto;
mod openapi;

use std::sync::Arc;
use std::time::Duration;
use axum::http::Method;
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, Any, CorsLayer};
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use oxide_data::manager::UserManager;
use crate::handler::auth::login;
use crate::handler::user::me;
use crate::openapi::ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    log_startup_banner();

    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            "http://localhost:8080".parse().unwrap(),
            "http://127.0.0.1:8080".parse().unwrap(),
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
            "content-type".parse().unwrap(),
            "authorization".parse().unwrap(),
            "accept".parse().unwrap(),
            "x-requested-with".parse().unwrap(),
            "access-control-allow-origin".parse().unwrap(),
        ]))
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    let auth_route = Router::new()
        .route("/login", post(login));


    let user_route = Router::new()
        .route("/me", get(me));

    let app = Router::new()
        .nest("/api/v1/auth", auth_route)
        .nest("/api/v1/users", user_route)
        .route("/", get(|| async { "Hello, World!" }))
        .merge(SwaggerUi::new("/swagger-ui" ).url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//#[derive(Clone)]
struct AppState {
    user_manager: UserManager
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