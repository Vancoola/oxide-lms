LABEL authors="suzdaltsev"
LABEL version="1.0.0"
LABEL description="API for Oxide project"

FROM rust:1.83-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --package oxide-api --bin oxide-api

FROM debian:bookworm-slim as runtime
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/oxide-api /usr/local/bin/oxide-api
COPY --from=builder /app/.env .env

ENV RUST_LOG=info
EXPOSE 8080

CMD ["oxide-api"]