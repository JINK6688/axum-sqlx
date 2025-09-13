# Axum + SQLx + Postgres — Multi-crate Workspace Scaffold

## Run locally

```bash
# Postgres
createdb axum_app
psql axum_app -c 'CREATE EXTENSION IF NOT EXISTS pgcrypto;'

# Env
cp .env.example .env

# Run
cargo run -p api

# Format code using rustfmt
cargo fmt
```

## Endpoints
- GET /health
- POST /users  {"email":"a@b.com", "name":"Alice"}
- GET /users?limit=50&offset=0
- GET /users/:id
