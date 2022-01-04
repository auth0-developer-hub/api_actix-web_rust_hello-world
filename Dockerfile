FROM rust:1.57-slim-bullseye

EXPOSE 6060

RUN useradd developer && \
    mkdir /usr/src/api_actix-web_rust_hello-world && \
    chown developer:developer -R /usr/src/api_actix-web_rust_hello-world

WORKDIR /usr/src/api_actix-web_rust_hello-world
USER developer

COPY Cargo.* .

RUN mkdir src && \
    echo -n > src/lib.rs && \
    cargo build --release && \
    rm -r src

COPY src src

RUN cargo build --release

ENTRYPOINT ["cargo", "run", "--release"]
