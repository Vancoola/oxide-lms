use utoipa::OpenApi;
use crate::handler::auth::{__path_login};
use crate::dto::auth::{LoginRequest};

#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "/", description = "Локальный сервер"),
        (url = "https://api.axion-tech.ru", description = "Продакшен сервер"),
        (url = "https://staging.axion-tech.ru", description = "Стейджнг сервер"),
    ),
    paths(
        login
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
        title="Oxide",
        version="0.2.0",
        description="Документация по LMS",
        license(
            name="EULA лицензия",
            url="https://axion-tech.ru/privacy-policy/",
        ),
        contact(
            name="Поддержка",
            email="support@axion-tech.ru"
        )
    )
)]
pub struct ApiDoc;