axum-sqlx-workspace/crates/api/Dockerfile
```
```Dockerfile
# ---- Build Stage ----
FROM rust:1.74 as builder

WORKDIR /app

# Cache dependencies
COPY ../../Cargo.toml ../../Cargo.lock ./
COPY ../.. .
RUN cargo fetch

# Build for release
RUN cargo build --release -p api

# ---- Runtime Stage ----
FROM debian:bullseye-slim

# Install necessary system dependencies
RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/api /app/api

# Copy any migration files or static assets if needed
# COPY --from=builder /app/migrations /app/migrations

ENV RUST_LOG=info

EXPOSE 3000

CMD ["./api"]
