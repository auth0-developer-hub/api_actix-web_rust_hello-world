use crate::api::types::Response;
use actix_web::{dev, http, middleware::errhandlers, Result};
use serde_json::json;

pub fn errhandlers<B: 'static>() -> errhandlers::ErrorHandlers<B> {
    errhandlers::ErrorHandlers::new()
        .handler(http::StatusCode::INTERNAL_SERVER_ERROR, internal_error)
        .handler(http::StatusCode::NOT_FOUND, not_found)
}

fn internal_error<B>(
    mut res: dev::ServiceResponse<B>,
) -> Result<errhandlers::ErrorHandlerResponse<B>> {
    res.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    // TODO: Extract the error message from the original response (separate PR)
    let msg = json!(Response {
        message: "Internal server error"
    });
    Ok(errhandlers::ErrorHandlerResponse::Response(res.map_body(
        |_, _| dev::ResponseBody::Body(dev::Body::from(msg)).into_body(),
    )))
}

fn not_found<B>(mut res: dev::ServiceResponse<B>) -> Result<errhandlers::ErrorHandlerResponse<B>> {
    res.headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    let msg = json!(Response {
        message: "Not found"
    });
    Ok(errhandlers::ErrorHandlerResponse::Response(res.map_body(
        |_, _| dev::ResponseBody::Body(dev::Body::from(msg)).into_body(),
    )))
}
