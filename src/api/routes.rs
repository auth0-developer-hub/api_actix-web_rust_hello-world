use crate::api::messages;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/api").service(messages::routes::routes())
}
