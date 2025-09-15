pub mod user_service;
use repositroy::PgPool;

#[derive(Clone)]
pub struct Services {
    pub pool: PgPool,
}

#[derive(Clone)]
pub struct AppState {
    pub services: Services,
}
