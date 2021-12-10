use actix_web::middleware::Logger;

pub fn logger() -> Logger {
    Logger::default()
}
