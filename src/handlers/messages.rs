use crate::types;
use actix_web::{get, web, Responder};

#[get("/api/messages/public")]
async fn public() -> impl Responder {
    web::Json(types::Response {
        message: "The API doesn't require an access token to share this message.",
    })
}

#[get("/api/messages/protected")]
async fn protected() -> impl Responder {
    web::Json(types::Response {
        message: "The API successfully validated your access token.",
    })
}

#[get("/api/messages/admin")]
async fn admin() -> impl Responder {
    web::Json(types::Response {
        message: "The API successfully recognized you as an admin.",
    })
}
