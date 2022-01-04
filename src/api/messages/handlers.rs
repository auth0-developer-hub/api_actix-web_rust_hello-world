use super::types::Message;
use actix_web::{get, web, Responder};

#[get("/admin")]
pub async fn admin() -> impl Responder {
    web::Json(Message {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "starter".to_string(),
        text: "The starter API doesn't require an access token to share this admin message.".to_string(),
    })
}

#[get("/protected")]
pub async fn protected() -> impl Responder {
    web::Json(Message {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "starter".to_string(),
        text: "The starter API doesn't require an access token to share this protected message.".to_string(),
    })
}

#[get("/public")]
pub async fn public() -> impl Responder {
    web::Json(Message {
        api: "api_actix-web_rust_hello-world".to_string(),
        branch: "starter".to_string(),
        text: "The starter API doesn't require an access token to share this public message.".to_string(),
    })
}
