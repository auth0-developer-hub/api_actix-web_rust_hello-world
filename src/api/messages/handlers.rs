use super::types::Message;
use crate::extractors::Claims;
use actix_web::{get, web, Responder};

#[get("/admin")]
pub async fn admin(_claims: Claims) -> impl Responder {
    web::Json(Message {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-authorization".to_string(),
        text: "The secured API requires a valid access token to share this admin message.".to_string(),
    })
}

#[get("/protected")]
pub async fn protected(_claims: Claims) -> impl Responder {
    web::Json(Message {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-authorization".to_string(),
        text: "The secured API requires a valid access token to share this protected message.".to_string(),
    })
}

#[get("/public")]
pub async fn public() -> impl Responder {
    web::Json(Message {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "basic-authorization".to_string(),
        text: "The secured API doesn't require an access token to share this public message.".to_string(),
    })
}
