FROM rust:1.57-slim-bullseye as build
WORKDIR /app
COPY Cargo.* .
RUN mkdir src && \
    touch src/lib.rs && \
    cargo build --release
COPY src src
RUN cargo build --release

FROM gcr.io/distroless/cc
ENV HOST "0.0.0.0"
COPY --from=build /app/target/release/api_actix-web_rust_hello-world /app/
CMD ["/app/api_actix-web_rust_hello-world"]
