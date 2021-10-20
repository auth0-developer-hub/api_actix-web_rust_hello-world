mod api;
mod app;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct Config {
    port: u16,
    client_origin_url: String,
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
            .wrap(app::middlewares::errhandlers())
            .wrap(cors)
            .service(api::routes())
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
