use actix_web::{dev, get, http, middleware, web, App, HttpServer, Responder, Result};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Deserialize)]
struct Config {
    #[serde(default="default_port")]
    port: u16,
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

fn default_port() -> u16 {
    6060
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

#[get("/500")]
async fn error_500() -> impl Responder {
    web::HttpResponse::InternalServerError()
}

fn internal_error<B>(mut res: dev::ServiceResponse<B>) -> Result<middleware::errhandlers::ErrorHandlerResponse<B>> {
    res.headers_mut().insert(http::header::CONTENT_TYPE, http::HeaderValue::from_static("application/json"));
    // TODO: serialize Response in the response body
    Ok(middleware::errhandlers::ErrorHandlerResponse::Response(res))
}

async fn not_found() -> impl Responder {
    web::HttpResponse::NotFound().json(Response { message: "Not found" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = envy::from_env::<Config>().expect("Provide missing environment variables");
    HttpServer::new(|| {
        App::new()
            .wrap(
                middleware::errhandlers::ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, internal_error)
            )
            .wrap(middleware::Logger::default())
            .service(public)
            .service(protected)
            .service(admin)
            .service(error_500)
            .default_service(web::to(not_found))
    })
    .bind(SocketAddr::from(([127, 0, 0, 1], config.port)))?
    .run()
    .await
}
