// INFO: References for the building blocks
// https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee
// https://doc.rust-lang.org/edition-guide/rust-2018/path-changes.html
// https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html
pub mod messages;
pub mod types;

use actix_web::{web, Scope};

pub fn routes() -> Scope {
    web::scope("/api").service(messages::routes())
}
