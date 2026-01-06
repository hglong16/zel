FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

RUN touch src/main.rs
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /root

COPY --from=builder /app/target/release/zel /usr/local/bin/zel


CMD ["zel"]