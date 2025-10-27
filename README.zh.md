# Axum + SQLx + Postgres — 本地开发中文说明 (README.zh.md)

这是一个基于 Rust 的示例工程，使用 `axum` 构建 HTTP API，使用 `sqlx` 与 Postgres 进行数据库交互，采用 multi-crate workspace（多 crate）结构。该文档为中文版本，覆盖本地启动、迁移、配置、API 端点以及常见问题排查等内容，帮助你快速上手。

目录
- 简介
- 前置条件
- 快速开始（本地）
  - 创建数据库并启用扩展
  - 配置环境变量
  - 运行迁移（两种方式）
  - 运行服务
- API 端点示例
- 配置说明（.env）
- 迁移文件位置与示例
- 项目结构概览
- Docker / 容器化（可选）
- 开发建议
- 常见问题与排查
- 贡献指南

---

## 简介
该仓库提供了一个可运行的模板，适合作为构建 Rust + Axum + SQLx + Postgres 服务的起点。项目将不同职责拆分为独立的 crate（例如 `api`、`repositroy`、`configure`、`migrations`），便于模块化开发与测试。

---

## 前置条件
- Rust（建议 stable）
- Cargo（随 Rust 一起）
- Postgres（版本 >= 12 推荐）
- 可选：`sqlx-cli`（用于方便运行迁移与预编译查询校验）
- 本地需要可用的 `createdb` / `psql` 命令行工具（通常随 Postgres 一起安装）

---

## 快速开始（本地）

下面是一个最小的本地启动流程示例。你可以按顺序执行以下步骤来启动服务并创建初始数据表。

1) 创建数据库并启用 `pgcrypto`（用于 UUID 生成）
```axum-sqlx/README.zh.md#L1-10
# Postgres
createdb axum_app
psql axum_app -c 'CREATE EXTENSION IF NOT EXISTS pgcrypto;'
```

2) 复制环境变量模板
```axum-sqlx/README.zh.md#L11-20
cp .env.example .env
# 然后编辑 .env，确保 DATABASE_URL 等变量设置正确
```

3) 运行数据库迁移（两种常见方式）

- 方式 A：使用 `sqlx-cli`（推荐用于开发）
  - 安装（如果尚未安装）：
  ```axum-sqlx/README.zh.md#L21-30
  cargo install sqlx-cli --no-default-features --features postgres,rustls
  ```
  - 运行迁移：
  ```axum-sqlx/README.zh.md#L31-40
  export DATABASE_URL="postgres://user:password@localhost/axum_app"
  sqlx migrate run --source ./crates/migrations
  ```

- 方式 B：手动执行 SQL 文件（直接用 psql）
  ```axum-sqlx/README.zh.md#L41-50
  psql "$DATABASE_URL" -f crates/migrations/20250101000000_create_users.sql
  ```

> 说明：本仓库的迁移文件位于 `crates/migrations`。如果你使用 `sqlx migrate run`，请确认当前目录或 `--source` 参数指向该目录。

4) 运行服务
```axum-sqlx/README.zh.md#L51-60
# 在 workspace 根运行 api crate
cargo run -p api

# 或者切换到 crates/api 并运行
cd crates/api
cargo run
```

5) 格式化代码
```axum-sqlx/README.zh.md#L61-70
cargo fmt
```

---

## API 端点示例（示意）
仓库中示例实现提供下列基础端点（具体以 `crates/api` 中路由为准）：

- GET /health
  - 用途：健康检查，返回服务状态
  - 示例：
  ```axum-sqlx/README.zh.md#L71-80
  curl -v http://localhost:3000/health
  ```

- POST /users
  - 用途：创建用户
  - 请求示例（JSON body）：
    - `email` (string)
    - `name` (string)
  - 示例：
  ```axum-sqlx/README.zh.md#L81-90
  curl -X POST http://localhost:3000/users \
    -H "Content-Type: application/json" \
    -d '{"email":"a@b.com","name":"Alice"}'
  ```

- GET /users?limit=50&offset=0
  - 用途：分页查询用户
  - 示例：
  ```axum-sqlx/README.zh.md#L91-100
  curl "http://localhost:3000/users?limit=50&offset=0"
  ```

- GET /users/:id
  - 用途：按 UUID 查询单个用户
  - 示例：
  ```axum-sqlx/README.zh.md#L101-110
  curl "http://localhost:3000/users/<uuid>"
  ```

---

## 配置说明（.env）
项目使用 `.env` 风格的环境变量（仓库根含 `.env.example`），请复制并修改为 `.env`。

常见变量（视项目实现而定）：
- `DATABASE_URL` — PostgreSQL 连接字符串，例如：`postgres://user:password@localhost/axum_app`
- `RUST_LOG` — 日志级别（例如 `info`, `debug`）
- `PORT` — 服务监听端口（如果 `api` crate 支持配置）
- 其他自定义配置请查看 `crates/configure` 及各 crate 的 README / 文档

注意：不要将含有真实凭据的 `.env` 提交到版本控制。

---

## 迁移文件位置与示例
所有 SQL 迁移文件位于：
- `crates/migrations`

示例迁移（创建 `users` 表）：
```axum-sqlx/crates/migrations/20250101000000_create_users.sql#L1-40
-- Create a basic users table
CREATE TABLE IF NOT EXISTS users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
```

新增迁移建议：
- 使用 `sqlx migrate add <name>`（如果使用 sqlx-cli），或手动按时间戳命名 SQL 文件并放入 `crates/migrations`。

---

## 项目结构概览
仓库采用 workspace，多 crate 组织（示意）：
```axum-sqlx/README.zh.md#L111-140
.
├── Cargo.toml           # workspace 根
├── .env.example
├── crates
│   ├── api              # HTTP 服务（axum）
│   ├── configure        # 配置读取/解析
│   ├── repositroy       # 数据访问层（注意仓库名称可能拼写为 repositroy）
│   └── migrations       # SQL migration 文件
└── README.md
```

说明：
- `crates/api`：负责 web 层、路由、启动逻辑。
- `crates/repositroy`：负责数据库访问（SQLx 查询、事务），可能包含 `users` CRUD。
- `crates/configure`：集中读取 `config`、`dotenv` 等配置。
- `crates/migrations`：存放 SQL 文件作为迁移。

---

## Docker / 容器化（可选）
你可以为项目创建 `Dockerfile` 与 `docker-compose.yml` 来快速在容器中运行 Postgres + API。基本思路：
- 使用 multi-stage build 构建 release 二进制
- 在容器运行时通过环境变量注入 `DATABASE_URL`
- 若需要，挂载 `crates/migrations` 到容器并在启动脚本中运行迁移

如果需要，我可以为你生成一个 `docker-compose.yml` 示例文件。

---

## 开发建议
- 代码格式化：`cargo fmt`
- 静态检查：`cargo clippy`
- 构建所有 crate：`cargo build --workspace`
- 运行单个 crate（开发调试）：`cargo run -p api`
- 当使用 `sqlx` 的编译时查询宏（例如 `sqlx::query!`）时，确保：
  - `DATABASE_URL` 在编译时可用，或者使用 `sqlx-data.json` 等替代策略
  - 在 CI 中提供一个可访问的测试数据库以便通过编译时检查（或禁用相关检查）

---

## 常见问题与排查
- 迁移找不到或失败：
  - 检查 `DATABASE_URL` 是否正确
  - 确认 `crates/migrations` 目录存在且路径被 `sqlx` 或手动命令正确引用
- UUID 生成失败：
  - 确保在数据库中执行 `CREATE EXTENSION IF NOT EXISTS pgcrypto;`
- 端点连接失败 / 404：
  - 检查 `api` crate 的路由是否注册，确认启动日志中监听地址和端口
- sqlx 的编译时查询失败：
  - 需要在编译时访问数据库；确保 CI/本地提供数据库或使用 `--no-default-features` 等策略

---

## 贡献与变更建议
- 新增迁移请遵循时间戳命名并放入 `crates/migrations`
- 若你要重命名 crate（例如把 `repositroy` 改为 `repository`），请同时更新 workspace 的 `Cargo.toml` 与所有依赖引用
- 提交 PR 前请运行 `cargo fmt` 并尽可能添加/更新测试
