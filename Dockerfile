FROM rust:1.58-slim-buster@sha256:9c8f5415ae79aad6337ab366293270846769d4b5f06a08c619a983c4c65d53de AS builder
RUN groupadd auth0 && useradd -m developer -g auth0
USER developer
RUN mkdir /home/developer/app
WORKDIR /home/developer/app
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src && \
    touch src/lib.rs && \
    cargo build --release
COPY src src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10@sha256:73da85e4f095290b0e3636b331ff5fa874cc890e1bc0977aeb22f21b432cfdec
USER 1000
ENV HOST "0.0.0.0"
COPY --from=builder /home/developer/app/target/release/api_actix-web_rust_hello-world .
EXPOSE 6060
CMD ["/api_actix-web_rust_hello-world"]
