# Axum + SQLx + Postgres — Multi-crate Workspace (English-first)

This repository is an example Rust workspace demonstrating a small HTTP API built with `axum`, using `sqlx` for Postgres access. The project is split into multiple crates for clearer separation of concerns (API, repository, configuration, migrations).

If you prefer Chinese documentation, see: [README.zh.md](./README.zh.md)

Table of Contents
- Prerequisites
- Quick start (local)
- Database migrations
- Environment variables
- API endpoints (example)
- Project layout
- Docker / Local compose
- Development tips
- Troubleshooting
- Contributing

---

## Prerequisites

- Rust (stable) and Cargo
- Postgres (v12+ recommended)
- Command-line tools: `createdb`, `psql`
- Optional: `sqlx-cli` (useful for running migrations and compile-time checks)

---

## Quick start (local)

1) Create the database and enable `pgcrypto` (used for UUID generation)

```axum-sqlx/README.md#L1-8
# create a database and enable pgcrypto
createdb axum_app
psql axum_app -c 'CREATE EXTENSION IF NOT EXISTS pgcrypto;'
```

2) Copy environment template

```axum-sqlx/README.md#L9-12
cp .env.example .env
# edit .env and set DATABASE_URL, etc.
```

3) Run database migrations

Migrations live under `crates/migrations` (see the "Database migrations" section below). You can apply them in one of two ways:

- Option A — using `sqlx-cli` (recommended):

```axum-sqlx/README.md#L13-22
# Install (one time)
cargo install sqlx-cli --no-default-features --features postgres,rustls

# Set DATABASE_URL and run migrations
export DATABASE_URL="postgres://user:password@localhost/axum_app"
sqlx migrate run --source ./crates/migrations
```

- Option B — run SQL files directly with `psql`:

```axum-sqlx/README.md#L23-26
psql "$DATABASE_URL" -f crates/migrations/20250101000000_create_users.sql
```

4) Run the API service

```axum-sqlx/README.md#L27-32
# From the workspace root
cargo run -p api

# Or run the api crate directly:
# cd crates/api && cargo run
```

5) Format code

```axum-sqlx/README.md#L33-34
cargo fmt
```

---

## Database migrations

All SQL migration files are under `crates/migrations`. The repository includes an example migration that creates a `users` table:

```axum-sqlx/crates/migrations/20250101000000_create_users.sql#L1-40
-- Create a basic users table
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

Notes:
- If you use `sqlx migrate run`, ensure `--source` points to `crates/migrations` (or run from an appropriate working directory).
- Prefer `sqlx migrate add <name>` to generate new migrations if you use `sqlx-cli`; otherwise, add timestamped SQL files to `crates/migrations`.

---

## Environment variables

Copy `.env.example` to `.env` and update values before running the service.

Common variables (check `.env.example` for actual names used by the project):
- `DATABASE_URL` — Postgres connection string (e.g., `postgres://user:password@localhost/axum_app`)
- `RUST_LOG` — Logging level (e.g., `info` or `debug`)
- `PORT` — HTTP listen port (if supported by `api` crate)

Do not commit secrets or real credentials to source control.

---

## API endpoints (example)

These are the example endpoints described in this project. For exact behavior and request/response formats, refer to the `crates/api` implementation.

- GET /health
  - Purpose: health check
  - Example:

```axum-sqlx/README.md#L35-38
curl -v http://localhost:3000/health
```

- POST /users
  - Purpose: create a user
  - Example request JSON:
    - `email` (string)
    - `name` (string)
  - Example:

```axum-sqlx/README.md#L39-44
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"email":"a@b.com","name":"Alice"}'
```

- GET /users?limit=50&offset=0
  - Purpose: list users with pagination
  - Example:

```axum-sqlx/README.md#L45-46
curl "http://localhost:3000/users?limit=50&offset=0"
```

- GET /users/:id
  - Purpose: fetch a user by UUID
  - Example:

```axum-sqlx/README.md#L47-48
curl "http://localhost:3000/users/<uuid>"
```

---

## Project layout (high level)

The workspace contains multiple crates. Typical layout:

```axum-sqlx/README.md#L49-64
.
├── Cargo.toml           # workspace root
├── .env.example
├── crates
│   ├── api              # axum HTTP service and routes
│   ├── configure        # configuration loading (config, dotenv)
│   ├── repositroy       # database repository / sqlx queries (crate name may be spelled `repositroy`)
│   └── migrations       # SQL migration files
└── README.md
```

Notes:
- `crates/api` contains the `main` / server bootstrap and route registration.
- `crates/repositroy` implements DB access (users CRUD). Consider renaming if you prefer `repository`.

---

## Docker / Local compose

The repo contains a `Dockerfile` to build a release image. For local multi-container development you can add a `docker-compose.yml` to run Postgres + API together, e.g.:

- Postgres service with a volume for data
- API service that depends on Postgres and runs migrations at startup (or you run migrations manually)

If you'd like, I can generate a ready-to-use `docker-compose.yml` for this project.

---

## Development tips

- Format code: `cargo fmt`
- Lint: `cargo clippy`
- Build workspace: `cargo build --workspace`
- Run a single crate for development: `cargo run -p api`
- If you use `sqlx` compile-time macros (e.g. `sqlx::query!`), ensure `DATABASE_URL` is available at compile time, or adopt `sqlx-data.json` strategies for CI.

---

## Troubleshooting

- Migration errors:
  - Verify `DATABASE_URL` and DB connectivity.
  - Ensure migrations path is correct when running `sqlx migrate run`.
- UUID generation issues:
  - Make sure `pgcrypto` extension was created (see Quick start).
- Missing tables/columns at runtime:
  - Confirm migrations were executed against the correct database (check `DATABASE_URL`).

---

## Contributing

- Add migrations in `crates/migrations` with timestamped file names (or use `sqlx migrate add`).
- Add new API routes in `crates/api` and corresponding repository functions in `crates/repositroy`.
- Run `cargo fmt` and include tests where applicable before submitting PRs.
