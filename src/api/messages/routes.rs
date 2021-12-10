use super::handlers;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/messages")
        .service(handlers::admin)
        .service(handlers::protected)
        .service(handlers::public)
}
