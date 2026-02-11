use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use crate::user::auth::{__path_login};
use crate::user::{__path_me};
use crate::dto::auth::{LoginRequest};

#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "/", description = "Локальный сервер"),
        (url = "https://api.axion-tech.ru", description = "Продакшен сервер"),
        (url = "https://staging.axion-tech.ru", description = "Стейджнг сервер"),
    ),
    paths(
        login,
        me
    ),
    components(
        schemas(LoginRequest),
    ),
    tags(
        (name="auth", description="Управление получением доступа"),
        (name="user", description="Управление аккаунтом"),
        (name="admin", description="Административный функционал"),
    ),
    info(
        title="Oxide-LMS API",
        version="0.2.0",
        description="Документация по LMS",
        license(
            name="Apache-2.0",
            url="https://www.apache.org/licenses/",
        ),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        openapi.components = Some(
            utoipa::openapi::ComponentsBuilder::new()
                .security_scheme(
                    "api_jwt_token",
                    SecurityScheme::Http(
                        HttpBuilder::new()
                            .scheme(HttpAuthScheme::Bearer)
                            .bearer_format("JWT")
                            .build(),
                    ),
                )
                .build(),
        )
    }
}