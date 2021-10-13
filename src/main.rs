use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct Response {
    message: &'static str
}

#[get("/api/messages/public")]
async fn public() -> impl Responder {
    web::Json(Response { message: "The API doesn't require an access token to share this message." })
}

#[get("/api/messages/protected")]
async fn protected() -> impl Responder {
    web::Json(Response { message: "The API successfully validated your access token." })
}

#[get("/api/messages/admin")]
async fn admin() -> impl Responder {
    web::Json(Response { message: "The API successfully recognized you as an admin." })
}

async fn not_found() -> impl Responder {
    web::HttpResponse::NotFound().json(Response { message: "Not found" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(public)
            .service(protected)
            .service(admin)
            .default_service(web::to(not_found))
    })
    .bind("127.0.0.1:6060")?
    .run()
    .await
}
