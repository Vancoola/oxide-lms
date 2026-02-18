use std::path::PathBuf;
use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub tracing_level: String,
}

#[derive(Deserialize, Debug)]
pub struct AdminConfig {
    pub register: bool,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct JwtConfig {
    pub secret: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database_url: String,
    pub admin: AdminConfig,
    pub jwt: JwtConfig,
}

pub fn load_config() -> AppConfig {

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let app_toml = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .map(|p| p.join("App.toml"))
        .expect("Failed to construct config path");

    Figment::new()
        .merge(Toml::file(app_toml))
        .merge(Env::raw())
        .merge(Env::prefixed("LMS_").split("__"))
        .extract()
        .expect("Can't extract config")
}