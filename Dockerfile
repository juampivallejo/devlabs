# syntax=docker/dockerfile:1

# ---- Build Stage ----
FROM rust:1.86-slim as builder

WORKDIR /app

# Install required build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch
run cargo install sqlx-cli --no-default-features --features sqlite

COPY src ./src
COPY migrations ./migrations
# SQLX prepare needs to be run beforehand with a valid database connection
COPY .sqlx ./.sqlx

# Build the application in release mode
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Set DATABASE_URL and Server port
ARG DATABASE_URL
ARG SERVER_PORT=3000
ENV DATABASE_URL=${DATABASE_URL}
ENV SERVER_PORT=${SERVER_PORT}

WORKDIR /app

COPY --from=builder /app/target/release/api /app/app

EXPOSE $SERVER_PORT

CMD ["./app"]
