FROM rust:latest AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release

COPY . .
RUN cargo build --release --bin websocket-basic

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/websocket-basic /app/app
COPY --from=builder /app/migrations /app/migrations

ENV DATABASE_URL=postgres://postgres:postgres@postgres:5432/chat
ENV REDIS_URL=redis://redis:6379

EXPOSE 8080
CMD ["./app"]
