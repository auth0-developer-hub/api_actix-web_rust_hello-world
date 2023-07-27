use actix_web::{http::header, middleware::DefaultHeaders};

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add((header::X_XSS_PROTECTION, "0"))
        .add((
            header::STRICT_TRANSPORT_SECURITY,
            "max-age=31536000; includeSubDomains",
        ))
        .add((header::X_FRAME_OPTIONS, "deny"))
        .add((header::X_CONTENT_TYPE_OPTIONS, "nosniff"))
        .add(
            (header::CONTENT_SECURITY_POLICY,
            "default-src 'self'; frame-ancestors 'none';",)
        )
        .add((
            header::CACHE_CONTROL,
            "no-cache, no-store, max-age=0, must-revalidate",
        ))
        .add((header::PRAGMA, "no-cache"))
        .add((header::EXPIRES, "0"))
}