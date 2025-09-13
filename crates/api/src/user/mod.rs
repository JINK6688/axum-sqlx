use std::str::FromStr;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use db::User;
use serde::{Deserialize, Serialize};
use service::AppState;
use tracing::error;

#[derive(Deserialize)]
pub struct CreateUserReq {
    email: String,
    name: String,
}

#[derive(Serialize)]
pub struct UserRes {
    id: String,
    email: String,
    name: String,
    created_at: String,
}

impl From<db::User> for UserRes {
    fn from(u: db::User) -> Self {
        Self {
            id: u.id.to_string(),
            email: u.email,
            name: u.name,
            created_at: match u.created_at {
                Some(t) => t.to_rfc3339(),
                None => "".to_string(),
            },
        }
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<CreateUserReq>,
) -> impl IntoResponse {
    match state.services.create_user(&body.email, &body.name).await {
        Ok(user) => (StatusCode::CREATED, Json(UserRes::from(user))).into_response(),
        Err(e) => {
            error!(?e, "failed to create user");
            (StatusCode::INTERNAL_SERVER_ERROR, "failed").into_response()
        }
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Json(input): Json<User>,
) -> impl IntoResponse {
    match state.services.update_user(&input).await {
        Ok(user) => (StatusCode::OK, Json(UserRes::from(user))).into_response(),
        Err(e) => {
            error!(?e, "failed to update user");
            (StatusCode::INTERNAL_SERVER_ERROR, "failed").into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct ListQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> impl IntoResponse {
    let limit = q.limit.unwrap_or(50).min(200);
    let offset = q.offset.unwrap_or(0).max(0);
    match state.services.list_users(limit, offset).await {
        Ok(users) => {
            let list: Vec<UserRes> = users.into_iter().map(UserRes::from).collect();
            (StatusCode::OK, Json(list)).into_response()
        }
        Err(e) => {
            error!(?e, "failed to list users");
            (StatusCode::INTERNAL_SERVER_ERROR, "failed").into_response()
        }
    }
}

pub async fn get_user(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let uid = uuid::Uuid::from_str(&id).map_err(|_| ()).ok();
    if uid.is_none() {
        return (StatusCode::BAD_REQUEST, "invalid id").into_response();
    }
    match state.services.get_user(uid.unwrap()).await {
        Ok(Some(user)) => (StatusCode::OK, Json(UserRes::from(user))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => {
            error!(?e, "failed to get user");
            (StatusCode::INTERNAL_SERVER_ERROR, "failed").into_response()
        }
    }
}

pub async fn del_user(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let uid = uuid::Uuid::from_str(&id).map_err(|_| ()).ok();
    if uid.is_none() {
        return (StatusCode::BAD_REQUEST, "invalid id").into_response();
    }
    match state.services.del_user(uid.unwrap()).await {
        Ok(Some(user)) => (StatusCode::OK, Json(UserRes::from(user))).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "not found").into_response(),
        Err(e) => {
            error!(?e, "failed to del user");
            (StatusCode::INTERNAL_SERVER_ERROR, "failed").into_response()
        }
    }
}
