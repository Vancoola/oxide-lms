mod handler;
mod openapi;
mod dto;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::PgPool;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use oxide_data::manager::UserManager;
use crate::handler::auth::login;
use crate::openapi::ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    log_startup_banner();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");

    let app_state = web::Data::new(AppState{
        user_manager: UserManager::new(PgPool::connect(database_url.as_str()).await?),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://axion-tech.ru")
            .allowed_origin("http://localhost:8080")
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(
                web::scope("/api/v1/auth")
                    .service(login)
            )
            .service(SwaggerUi::new("/docs/v1/{_:.*}").url("/api-docs/v1/openapi.json", ApiDoc::openapi()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}


struct AppState {
    pub user_manager: UserManager
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