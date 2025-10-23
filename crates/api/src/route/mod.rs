use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use middleware;
use service::AppState;
pub mod health;
pub use health as other_health;

use crate::user;

pub fn api_route(state: AppState) -> Router {
    let none_auth_route = none_auth_route();
    let auth_route = middleware::apply(auth_route(state));
    Router::merge(none_auth_route, auth_route)
}

pub fn none_auth_route() -> Router {
    Router::new()
        .route("/health", get(other_health::health))
        .route("/example/token", get(other_health::example_user))
        .route("/example/user", get(other_health::example_user_info))
}

pub fn auth_route(state: AppState) -> Router {
    Router::new()
        .route("/users", post(user::create_user).get(user::list_users).put(user::update_user))
        .route("/users/:id", get(user::get_user).delete(user::del_user))
        .with_state(state)
        .fallback(fallback_handler)
}

/// Fallback handler for unmatched routes
async fn fallback_handler(req: Request<Body>) -> Response {
    let not_found = format!("No route for {}", req.uri());
    (StatusCode::NOT_FOUND, not_found).into_response()
}
