# Building Blocks

## Steps

Create a new project

```bash
cargo new api_actix-web_rust_hello-world
```

Add `actix-web` dependency to `Cargo.toml`

```diff
[dependencies]
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

Add `serde` dependency to `Cargo.toml`

```diff
[dependencies]
actix-web = "^3.3.2"
+ serde = "^1.0.130"
```

Import `serde` inside the `src/main.rs` file, and create a `struct` for the
`Response`:

```diff
use actix_web::{get, web, App, HttpServer, Responder};
+ use serde::{Serialize};

+ #[derive(Serialize)]
+ struct Response {
+     message: &'static str
+ }

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
            .service(public)
            .service(protected)
            .service(admin)
    })
    .bind("127.0.0.1:6060")?
    .run()
    .await
}
```

Refactor the code using the new `Response` struct:

```diff
use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct Response {
    message: &'static str
}

#[get("/api/messages/public")]
async fn public() -> impl Responder {
-     web::Json("The API doesn't require an access token to share this message.")
+     web::Json(Response { message: "The API doesn't require an access token to share this message." })
}

#[get("/api/messages/protected")]
async fn protected() -> impl Responder {
-     web::Json("The API successfully validated your access token.")
+     web::Json(Response { message: "The API successfully validated your access token." })
}

#[get("/api/messages/admin")]
async fn admin() -> impl Responder {
-     web::Json("The API successfully recognized you as an admin.")
+     web::Json(Response { message: "The API successfully recognized you as an admin." })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(public)
            .service(protected)
            .service(admin)
    })
    .bind("127.0.0.1:6060")?
    .run()
    .await
}
```

## References

- https://actix.rs/#responders
- https://actix.rs/docs/errors/
