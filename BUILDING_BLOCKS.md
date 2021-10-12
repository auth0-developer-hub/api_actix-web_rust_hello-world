# Building Blocks

## Steps

Create a new project

```bash
cargo new api_actix-web_rust_hello-world
```

Add `actix-web` dependency to `Cargo.toml`

```diff
- [dependencies]
+ [dependencies]
+ actix-web = "^3.3.2"
```

Import `actix_web` inside the `src/main.rs` file:

```diff
+ use actix_web::{get, web, App, HttpServer, Responder};

fn main() {
    println!("Hello, world!");
}
```

Replace the original `main` function with the following code:

```diff
- fn main() {
-     println!("Hello, world!");
- }
+ #[actix_web::main]
+ async fn main() -> std::io::Result<()> {
+     HttpServer::new(|| {
+         App::new()
+     })
+     .bind("127.0.0.1:6060")?
+     .run()
+     .await
+ }
```

Add the 3 endpoints:

```diff
use actix_web::{get, web, App, HttpServer, Responder};

+ #[get("/api/messages/public")]
+ async fn public() -> impl Responder {
+     web::Json("The API doesn't require an access token to share this message.")
+ }

+ #[get("/api/messages/protected")]
+ async fn protected() -> impl Responder {
+     web::Json("The API successfully validated your access token.")
+ }

+ #[get("/api/messages/admin")]
+ async fn admin() -> impl Responder {
+     web::Json("The API successfully recognized you as an admin.")
+ }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
    })
    .bind("127.0.0.1:6060")?
    .run()
    .await
}
```

Register all 3 endpoints:

```diff
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/api/messages/public")]
async fn public() -> impl Responder {
    web::Json("The API doesn't require an access token to share this message.")
}

#[get("/api/messages/protected")]
async fn protected() -> impl Responder {
    web::Json("The API successfully validated your access token.")
}

#[get("/api/messages/admin")]
async fn admin() -> impl Responder {
    web::Json("The API successfully recognized you as an admin.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
+             .service(public)
+             .service(protected)
+             .service(admin)
    })
    .bind("127.0.0.1:6060")?
    .run()
    .await
}
```

## References

- https://actix.rs/#responders
- https://actix.rs/docs/errors/
