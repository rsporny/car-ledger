FROM rust:1.82 AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx

RUN cargo install sqlx-cli
RUN cargo build
RUN cargo sqlx prepare --check

EXPOSE 8081

COPY migrations ./migrations
COPY ./entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]