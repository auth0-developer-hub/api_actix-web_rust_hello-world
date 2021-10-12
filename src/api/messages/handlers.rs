use super::types::Message;
use crate::extractors::Claims;
use actix_web::{get, web, Responder};

#[get("/admin")]
pub async fn admin(_claims: Claims) -> impl Responder {
    web::Json(Message {
        text: "The API successfully recognized you as an admin.".to_string(),
    })
}

#[get("/protected")]
pub async fn protected(_claims: Claims) -> impl Responder {
    web::Json(Message {
        text: "The API successfully validated your access token.".to_string(),
    })
}

#[get("/public")]
pub async fn public() -> impl Responder {
    web::Json(Message {
        text: "The API doesn't require an access token to share this message.".to_string(),
    })
}
