mod route;
mod user;
use configure::{error::AppError, log_tracing, AppConfig, CONFIG};
use repositroy::{get_db_pool, init_database};
use service::{AppState, Services};
use tokio::signal;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize tracing
    let _guard = log_tracing::init();

    // Load configuration
    let app_config: AppConfig = CONFIG.clone();
    //init database connect
    init_database().await;
    let pool = get_db_pool().clone();
    let services = Services::new(pool);
    let state = AppState { services };

    // Routes
    let app = route::api_route(state);

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
