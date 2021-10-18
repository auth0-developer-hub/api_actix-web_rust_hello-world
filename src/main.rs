use actix_cors::Cors;
use actix_web::{dev, get, http, middleware, web, App, HttpServer, Responder, Result};
use env_logger::Env;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Deserialize)]
struct Config {
    #[serde(default = "default_port")]
    port: u16,
    #[serde(default = "default_client_origin_url")]
    client_origin_url: String,
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

fn default_port() -> u16 {
    6060
}

fn default_client_origin_url() -> String {
    "http://localhost:4040".to_string()
}

#[get("/api/messages/public")]
async fn public() -> impl Responder {
    web::Json(Response {
        message: "The API doesn't require an access token to share this message.",
    })
}

#[get("/api/messages/protected")]
async fn protected() -> impl Responder {
    web::Json(Response {
        message: "The API successfully validated your access token.",
    })
}

#[get("/api/messages/admin")]
async fn admin() -> impl Responder {
    web::Json(Response {
        message: "The API successfully recognized you as an admin.",
    })
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
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
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
            .service(public)
            .service(protected)
            .service(admin)
            .default_service(web::to(not_found))
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
