FROM rust:1.51 AS builder

WORKDIR /usr/src/texide-docs

COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo fetch

COPY src src
RUN cargo doc --no-deps --lib

FROM nginx
COPY --from=builder /usr/src/texide-docs/target/doc /usr/share/nginx/html/rustdoc
