use super::types::Message;
use crate::{extractors::Claims, types::ErrorMessage};
use actix_web::{get, web, Responder, HttpResponse};
use std::collections::HashSet;

#[get("/admin")]
pub async fn admin(claims: Claims) -> impl Responder {
    if claims.validate_permissions(&HashSet::from(["read:admin-messages".to_string()])) {
        Ok(web::Json(Message {
            text: "The API successfully recognized you as an admin.".to_string(),
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
        text: "The API successfully validated your access token.".to_string(),
    })
}

#[get("/public")]
pub async fn public() -> impl Responder {
    web::Json(Message {
        text: "The API doesn't require an access token to share this message.".to_string(),
    })
}
