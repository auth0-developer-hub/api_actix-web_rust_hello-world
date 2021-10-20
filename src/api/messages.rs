pub mod handlers;

use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/messages")
        .route("/admin", web::get().to(handlers::admin))
        .route("/protected", web::get().to(handlers::protected))
        .route("/public", web::get().to(handlers::public))
}
