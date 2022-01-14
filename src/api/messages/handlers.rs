use super::types::{Message, Metadata};
use crate::{extractors::Claims, types::ErrorMessage};
use actix_web::{get, web, HttpResponse, Responder};
use std::collections::HashSet;

#[get("/admin")]
pub async fn admin(claims: Claims) -> impl Responder {
    if claims.validate_permissions(&HashSet::from(["read:admin-messages".to_string()])) {
        Ok(web::Json(Message {
            metadata: Metadata {
                api: "api_actix-web_rust_hello-world".to_string(),
                branch: "main".to_string(),
            },
            text: "This is an admin message.".to_string(),
        }))
    } else {
        Err(HttpResponse::Forbidden().json(ErrorMessage {
            error: Some("insufficient_permissions".to_string()),
            error_description: Some("Requires read:admin-messages".to_string()),
            message: "Permission denied".to_string(),
        }))
    }
}

#[get("/protected")]
pub async fn protected(_claims: Claims) -> impl Responder {
    web::Json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "main".to_string(),
        },
        text: "This is a protected message.".to_string(),
    })
}

#[get("/public")]
pub async fn public() -> impl Responder {
    web::Json(Message {
        metadata: Metadata {
            api: "api_actix-web_rust_hello-world".to_string(),
            branch: "main".to_string(),
        },
        text: "This is a public message.".to_string(),
    })
}
