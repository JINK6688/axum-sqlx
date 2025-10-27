# Axum + SQLx + Postgres — Multi-crate Workspace Scaffold (English)

This repository is a Rust example workspace that demonstrates a small HTTP API using `axum` with `sqlx` for Postgres access. The project is organized as multiple crates (workspace) so API, configuration, repository (DB layer), and migrations are separated for clarity.

This English README provides instructions to run the project locally, run migrations, configure environment variables, and the basic API endpoints available in the example.

Prerequisites
- Rust (stable) and Cargo
- Postgres (v12+ recommended)
- Command-line tools: `createdb`, `psql`
- Optional: `sqlx-cli` (useful for running migrations and compile-time checks)

Quick start (local)
1) Create the database and enable the `pgcrypto` extension (used for UUID generation):

```axum-sqlx/README.en.md#L1-12
# Postgres
createdb axum_app
psql axum_app -c 'CREATE EXTENSION IF NOT EXISTS pgcrypto;'
```

2) Copy the environment template and edit as needed:

```axum-sqlx/README.en.md#L13-20
cp .env.example .env
# Edit .env and set DATABASE_URL, etc.
```

3) Run database migrations (two common approaches)

- Option A — Use sqlx-cli (recommended for development):

```axum-sqlx/README.en.md#L21-36
# Install (if not installed)
cargo install sqlx-cli --no-default-features --features postgres,rustls

# Set DATABASE_URL and run migrations
export DATABASE_URL="postgres://user:password@localhost/axum_app"
sqlx migrate run --source ./crates/migrations
```

- Option B — Execute SQL migration files directly (e.g. with psql):

```axum-sqlx/README.en.md#L37-44
# Example: run the example migration file with psql
psql "$DATABASE_URL" -f crates/migrations/20250101000000_create_users.sql
```

Note: This repository keeps migrations under `crates/migrations`. When using `sqlx migrate run`, either ensure your current working directory and `--source` are set correctly or pass the appropriate flags so sqlx points to that migrations directory.

4) Run the service

```axum-sqlx/README.en.md#L45-56
# From the workspace root
cargo run -p api

# Or run inside the api crate
# cd crates/api && cargo run
```

5) Format the code

```axum-sqlx/README.en.md#L57-60
cargo fmt
```

Environment variables
- Copy `.env.example` to `.env` and provide appropriate values.
- Common variables (check `.env.example` in the repo for exact names):
  - `DATABASE_URL` — Postgres connection string, e.g. `postgres://user:password@localhost/axum_app`
  - `RUST_LOG` — logging level, e.g. `info` or `debug`
  - `PORT` — HTTP server port (if the api crate supports it)
- Do not commit secrets to version control.

Migration files
- Location: `crates/migrations`
- Example migration that is included in the repository (creates a `users` table):

```axum-sqlx/crates/migrations/20250101000000_create_users.sql#L1-40
-- Create a basic users table
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

API endpoints (example)
These are the basic endpoints described by the example project. For the exact behavior, check the `crates/api` crate source.

- GET /health
  - Purpose: service health check
  - Example:
```axum-sqlx/README.en.md#L61-66
curl -v http://localhost:3000/health
```

- POST /users
  - Purpose: create a user
  - Request body (JSON):
    - `email` (string)
    - `name` (string)
  - Example:
```axum-sqlx/README.en.md#L67-76
curl -X POST http://localhost:3000/users \
  -H "Content-Type: application/json" \
  -d '{"email":"a@b.com","name":"Alice"}'
```

- GET /users?limit=50&offset=0
  - Purpose: list users with pagination
  - Example:
```axum-sqlx/README.en.md#L77-80
curl "http://localhost:3000/users?limit=50&offset=0"
```

- GET /users/:id
  - Purpose: fetch a user by UUID
  - Example:
```axum-sqlx/README.en.md#L81-84
curl "http://localhost:3000/users/<uuid>"
```

Project layout (high level)
- `Cargo.toml` — workspace root
- `.env.example` — environment variable template
- `crates/`
  - `api` — axum-based HTTP service and routing
  - `configure` — configuration loading (dotenv, config, etc.)
  - `repositroy` — database access layer (note: repository crate name may be spelled `repositroy` in this repo)
  - `migrations` — SQL migration files

Development notes and best practices
- Format regularly with `cargo fmt`.
- Use `cargo clippy` for linting.
- Build the whole workspace with `cargo build --workspace`.
- When using `sqlx` compile-time macros (e.g. `sqlx::query!`), be aware those macros can require a reachable `DATABASE_URL` at compile time. In CI, either provide a test database or use strategies such as `sqlx-data.json` to avoid network dependency.
- If you rename crates (for example, `repositroy` -> `repository`), update workspace `Cargo.toml` and all internal references.

Docker & CI
- The repository includes a `Dockerfile` for building a release binary image. When running the container, provide a `DATABASE_URL` environment variable.
- If you want a one-command local setup, you can add a `docker-compose.yml` that brings up Postgres and the API (and runs migrations at startup). I can provide a `docker-compose.yml` template on request.

Troubleshooting
- "migrations not found" — ensure `sqlx migrate run`'s source points to `crates/migrations` or run from the proper working directory.
- UUID generation errors — confirm `pgcrypto` extension was created in the DB.
- Missing tables/columns — confirm migrations ran against the correct database (check `DATABASE_URL`).
- sqlx compile-time errors — ensure `DATABASE_URL` is available during build if using `query!` macros, or set up `sqlx-data.json`.

Contributing
- Open issues or PRs are welcome.
- Before submitting a PR, run `cargo fmt` and include tests where appropriate.
- When adding migrations, prefer `sqlx migrate add <name>` (if using sqlx-cli) or create a timestamped SQL file under `crates/migrations`.
