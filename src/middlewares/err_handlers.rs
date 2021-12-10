use crate::types::ErrorMessage;
use actix_web::{
    dev::ServiceResponse,
    http::StatusCode,
    middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers},
    web::HttpResponse,
    Result,
};

pub fn err_handlers<B: 'static>() -> ErrorHandlers<B> {
    ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, internal_error)
        .handler(StatusCode::NOT_FOUND, not_found)
}

fn internal_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let http_res = HttpResponse::InternalServerError().json(ErrorMessage {
        error: None,
        error_description: None,
        message: "Internal server error".to_string(),
    });
    Ok(ErrorHandlerResponse::Response(
        res.into_response(http_res.into_body()),
    ))
}

fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let http_res = HttpResponse::NotFound().json(ErrorMessage {
        error: None,
        error_description: None,
        message: "Not Found".to_string(),
    });
    Ok(ErrorHandlerResponse::Response(
        res.into_response(http_res.into_body()),
    ))
}
