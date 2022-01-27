use super::types::{Message, Metadata};
use actix_web::{get, web, Responder};

#[get("/admin")]
pub async fn admin() -> impl Responder {
    web::Json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "starter".to_string(),
        },
        text: "This is an admin message.".to_string(),
    })
}

#[get("/protected")]
pub async fn protected() -> impl Responder {
    web::Json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "starter".to_string(),
        },
        text: "This is a protected message.".to_string(),
    })
}

#[get("/public")]
pub async fn public() -> impl Responder {
    web::Json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "starter".to_string(),
        },
        text: "This is a public message.".to_string(),
    })
}
