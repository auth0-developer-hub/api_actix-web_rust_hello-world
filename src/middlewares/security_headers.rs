use actix_web::{http::header, middleware::DefaultHeaders};

pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .header(header::X_XSS_PROTECTION, "0")
        .header(
            header::STRICT_TRANSPORT_SECURITY,
            "max-age=31536000; includeSubDomains",
        )
        .header(header::X_FRAME_OPTIONS, "deny")
        .header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
        .header(
            header::CONTENT_SECURITY_POLICY,
            "default-src 'self'; frame-ancestors 'none';",
        )
        .header(
            header::CACHE_CONTROL,
            "no-cache, no-store, max-age=0, must-revalidate",
        )
        .header(header::PRAGMA, "no-cache")
        .header(header::EXPIRES, "0")
}
