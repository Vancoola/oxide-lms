use oxide_shared_types::auth::JwtToken;
use oxide_shared_types::auth::AuthRequest;
use utoipa::{Modify, OpenApi};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use crate::user::auth::{__path_login};
use crate::user::{__path_me};

#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "/", description = "Localhost"),
    ),
    paths(
        login,
        me
    ),
    components(
        schemas(
            AuthRequest,
            JwtToken,
        ),
    ),
    tags(
        (name="auth", description="Access control"),
        (name="user", description="Account Management"),
        (name="admin", description="Administrative functionality"),
    ),
    info(
        title="Oxide-LMS API",
        version="0.1.0",
        description="LMS API Documentation",
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
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "api_jwt_token",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}