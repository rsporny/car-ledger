FROM rust:1.82 AS builder
WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html ./

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown
ARG BACKEND_URL=http://127.0.0.1:8081
RUN trunk build

FROM nginx:alpine
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
