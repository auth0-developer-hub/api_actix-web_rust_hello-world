use crate::api::types::Response;
use actix_web::{web, Responder};

pub async fn admin() -> impl Responder {
    web::Json(Response {
        message: "The API successfully recognized you as an admin.",
    })
}

pub async fn protected() -> impl Responder {
    web::Json(Response {
        message: "The API successfully validated your access token.",
    })
}

pub async fn public() -> impl Responder {
    web::Json(Response {
        message: "The API doesn't require an access token to share this message.",
    })
}
