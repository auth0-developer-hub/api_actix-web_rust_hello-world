use actix_web::web;

fn routes() -> String {
    web::scope("/api/messages");
}
