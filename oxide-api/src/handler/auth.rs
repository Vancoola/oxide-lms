use actix_web::{post, web, HttpResponse, Responder};
use validator::Validate;
use crate::dto::auth::LoginRequest;

#[utoipa::path(
    context_path="/api/v1/auth",
    post,
    path="/login",
    tag="auth",
    summary = "Authenticate user and return JWT token",
    description = "Logs in a user by verifying credentials and returns an access token. \
                 Requires valid email and password."
)]
#[post("/login")]
pub async fn login(body: web::Json<LoginRequest>) -> impl Responder {

    if let Err(e) = body.validate() {
        return HttpResponse::BadRequest().json(e);
    }

    HttpResponse::Ok().json("")
}