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

## 配置说明（.env）与 Settings 目录

本项目既支持使用 `.env`（`.env.example` 参照）也支持基于 `setting/` 目录的 TOML 配置。常见做法是：把不含 secrets 的默认配置放入 `setting/*.toml`，把敏感或部署环境相关的值通过环境变量注入（或通过 CI/secret manager）。

复制 `.env.example` 为 `.env` 并填写必要项，常见变量包括（以仓库中的 `.env.example` 为准）：
- `DATABASE_URL` — PostgreSQL 连接字符串，例如：`postgres://user:password@localhost/axum_app`
- `RUST_LOG` — 日志级别（例如 `info`, `debug`）
- `PORT` — 服务监听端口（如果 `api` crate 支持配置）

注意：不要将含有真实凭据的 `.env` 或包含 secrets 的 `setting/*.toml` 提交到版本控制。生产环境请使用 secrets 管理方案或环境变量。

### Settings 目录说明（`setting/`）
仓库中提供了 `setting/` 目录用于将配置管理为 TOML 文件（示例：`default.toml`、`development.toml`、`production.toml`、`test.toml`）。`crates/configure` 会按如下顺序合并配置（后者覆盖前者）：

1. `setting/default.toml` — 基础默认配置  
2. `setting/{profile}.toml` — profile 特定的覆盖（例如 `development.toml`, `production.toml`, `test.toml`）  
3. 环境变量（以 `APP` 为前缀） — 环境变量优先级最高，用于覆盖文件中的配置

配置加载器的核心在 `crates/configure`，实现读取仓库根目录下的 `setting` 文件夹（它通过向上查找 `Cargo.lock` 确定仓库根），然后按上面的顺序合并。例如，核心读取逻辑（已简化）如下：

```axum-sqlx/crates/configure/src/lib.rs#L1-120
pub fn read() -> Result<AppConfig, ConfigError> {
    let config_dir = get_root_dir()?.join("setting");

    let env_source = get_env_source("APP");
    let profile = get_profile()?;
    let profile_filename = format!("{profile}.toml");

    let config = config::Config::builder()
        .add_source(config::File::with_name(&format!("{}/default.toml", config_dir.to_string_lossy())))
        .add_source(config::File::with_name(&format!("{}/{}", config_dir.to_string_lossy(), profile_filename)))
        .add_source(env_source)
        .build()?;
    config.try_deserialize()
}
```

### Profile 与环境变量覆盖规则
- Profile 由环境变量 `ENVIRONMENT` 控制（例如 `development`、`production`、`test`）。若未设置，默认使用 `development`。实现位于 `crates/configure/src/env.rs`：
```axum-sqlx/crates/configure/src/env.rs#L1-40
pub fn get_profile() -> Result<Profile, config::ConfigError> {
    dotenvy::dotenv().ok();
    std::env::var("ENVIRONMENT")
        .map(|env| Profile::from_str(&env).map_err(|e| config::ConfigError::Message(e.to_string())))
        .unwrap_or_else(|_e| Ok(Profile::Development))
}
```

- 环境变量覆盖使用 `APP` 前缀，`__`（双下划线）表示嵌套配置键。例如：
  - `APP__DATABASE__HOST=127.0.0.1` 会覆盖 `database.host`
  - `APP__SERVER__PORT=8080` 会覆盖 `server.port`
  - `APP__JWT__SECRET=supersecret` 会覆盖 `jwt.secret`

示例（POSIX shell）：
```axum-sqlx/README.zh.md#L1-20
export APP__DATABASE__HOST=127.0.0.1
export APP__SERVER__PORT=8080
export ENVIRONMENT=production
```

### 仓库中 `setting/` 的示例文件
仓库自带示例配置文件（请根据实际部署需求修改并避免提交 secrets）：
- `setting/default.toml` — 基础默认配置
- `setting/development.toml` — 开发环境覆盖（示例包含 server、database、jwt 等字段）
- `setting/production.toml` — 生产环境覆盖（示例）
- `setting/test.toml` — 测试环境覆盖（示例）

示例片段（节选自 `setting/development.toml`）：
```axum-sqlx/setting/development.toml#L1-40
debug = true
profile = "development"
[tracing]
log_level = "info"

[server]
host = "0.0.0.0"
port = 3000

[database]
username = "postgres"
password = "password"
host = "127.0.0.1"
port = 5_432
database_name = "xxxx"

[jwt]
secret= "thisismysecret"
expired = 6
```

### 安全建议
- 示例 TOML 文件仅用于本地开发。不要在仓库中保存真实凭据或生产密钥。
- 将敏感信息通过环境变量注入 CI 或运行环境，或使用专门的密钥/凭据管理服务。

如果你愿意，我可以：
- 在 README 中加入几个常用的 `APP__...` 覆盖示例（中英文两版）；
- 生成一个不包含密钥的 `setting/production.toml` 模板；
- 在 `.env.example` 中增加关于 `setting/` 与环境变量如何协作的简短说明。

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
