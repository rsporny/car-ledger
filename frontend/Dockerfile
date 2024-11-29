FROM rust:1.82 AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock index.html ./
COPY src ./src

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

EXPOSE 8080
ENTRYPOINT ["trunk", "serve", "--address", "0.0.0.0"]