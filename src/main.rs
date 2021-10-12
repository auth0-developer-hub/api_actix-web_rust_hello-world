use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/messages/public")]
async fn public() -> impl Responder {
    HttpResponse::Ok().body("public")
}

#[get("/api/messages/protected")]
async fn protected() -> impl Responder {
    HttpResponse::Ok().body("protected")
}

#[get("/api/messages/admin")]
async fn admin() -> impl Responder {
    HttpResponse::Ok().body("admin")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(public)
            .service(protected)
            .service(admin)
    })
    .bind("127.0.0.1:6060")?
    .run()
    .await
}
