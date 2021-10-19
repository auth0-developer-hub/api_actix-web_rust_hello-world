mod api;

use crate::api::types::Response;
use actix_cors::Cors;
use actix_web::{dev, http, middleware, web, App, HttpServer, Responder, Result};
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::json;

#[derive(Clone, Deserialize)]
struct Config {
    port: u16,
    client_origin_url: String,
}

fn internal_error<B>(
    mut res: dev::ServiceResponse<B>,
) -> Result<middleware::errhandlers::ErrorHandlerResponse<B>> {
    res.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    let msg = json!(Response {
        message: "Internal server error"
    });
    Ok(middleware::errhandlers::ErrorHandlerResponse::Response(
        res.map_body(|_, _| dev::ResponseBody::Body(dev::Body::from(msg)).into_body()),
    ))
}

async fn not_found() -> impl Responder {
    web::HttpResponse::NotFound().json(Response {
        message: "Not found",
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = envy::from_env::<Config>().expect("Provide missing environment variables");
    let client_origin_url = config.client_origin_url;
    HttpServer::new(move || {
        let cors = Cors::default().allowed_origin(client_origin_url.as_str());
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                middleware::errhandlers::ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, internal_error),
            )
            .wrap(cors)
            .service(api::routes::routes())
            .default_service(web::to(not_found))
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
