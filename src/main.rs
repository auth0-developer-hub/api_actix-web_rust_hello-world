mod api;
mod app;

use actix_web::{middleware, App, HttpServer};
use dotenv::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    port: u16,
    client_origin_url: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let config = envy::from_env::<Config>().expect("Provide missing environment variables");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(app::middlewares::cors(&config.client_origin_url))
            .wrap(app::middlewares::errhandlers())
            .service(api::routes())
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
