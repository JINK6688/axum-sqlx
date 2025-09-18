mod user;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use configure::{log_tracing, AppConfig, CONFIG};
use middleware::{ctx::LoginUser, jwt::Claims};
use repositroy::{get_db_pool, init_database};
use service::{AppState, Services};
use tokio::signal;
use tracing::info;
use user::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    let _guard = log_tracing::init();

    // Load configuration
    let app_config: AppConfig = CONFIG.clone();
    //初始化数据库连接池
    init_database().await;
    let pool = get_db_pool().clone();
    let services = Services::new(pool);
    let state = AppState { services };

    // Routes
    let app = Router::new()
        .route("/health", get(health))
        .route("/health2", get(health2))
        .route("/users", post(create_user).get(list_users).put(update_user))
        .route("/users/:id", get(get_user).delete(del_user))
        .fallback(fallback_handler)
        .with_state(state);
    // Apply middleware
    let app = middleware::apply(app.clone());

    let server = app_config.server;
    let addr = server.get_socket_addr()?;
    info!("listening {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await?, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

/// Signal handler for graceful shutdown
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

/// Fallback handler for unmatched routes
async fn fallback_handler(req: Request<Body>) -> Response {
    let not_found = format!("No route for {}", req.uri());
    (StatusCode::NOT_FOUND, not_found).into_response()
}

async fn health() -> String {
    let claims = Claims::build("sub", "1", "test");
    let token = claims.to_token().unwrap();
    token
}

async fn health2(user: LoginUser) -> String {
    format!("Hello, {}! Your user_id is {}. Token exp: {}", user.username, user.user_id, user.exp)
}
