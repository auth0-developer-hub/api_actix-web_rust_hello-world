FROM rust:1.58-slim-buster@sha256:9c8f5415ae79aad6337ab366293270846769d4b5f06a08c619a983c4c65d53de AS builder
WORKDIR /app
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src && \
    touch src/lib.rs && \
    cargo build --release
COPY src src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
USER 1000
COPY --from=builder /app/target/release/api_actix-web_rust_hello-world .
ENV HOST "0.0.0.0"
EXPOSE 6060
CMD ["/api_actix-web_rust_hello-world"]
