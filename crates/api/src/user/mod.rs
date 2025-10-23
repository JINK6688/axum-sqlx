use std::str::FromStr;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use configure::error::AppError;
use repositroy::User;
use serde::{Deserialize, Serialize};
use service::AppState;

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

impl From<User> for UserRes {
    fn from(u: User) -> Self {
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
) -> Result<impl IntoResponse, AppError> {
    let user = state.services.create_user(&body.email, &body.name).await?;
    Ok((StatusCode::CREATED, Json(UserRes::from(user))))
}

pub async fn update_user(
    State(state): State<AppState>,
    Json(input): Json<User>,
) -> Result<impl IntoResponse, AppError> {
    let user = state.services.update_user(&input).await?;
    Ok((StatusCode::OK, Json(UserRes::from(user))))
}

#[derive(Deserialize)]
pub struct ListQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn list_users(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<impl IntoResponse, AppError> {
    let limit = q.limit.unwrap_or(50).min(200);
    let offset = q.offset.unwrap_or(0).max(0);
    let users = state.services.list_users(limit, offset).await?;
    let list: Vec<UserRes> = users.into_iter().map(UserRes::from).collect();
    Ok((StatusCode::OK, Json(list)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let uid =
        uuid::Uuid::from_str(&id).map_err(|_| AppError::BadRequest("invalid id".to_string()))?;
    match state.services.get_user(uid).await? {
        Some(user) => Ok((StatusCode::OK, Json(UserRes::from(user)))),
        None => Err(AppError::NotFound),
    }
}

pub async fn del_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let uid =
        uuid::Uuid::from_str(&id).map_err(|_| AppError::BadRequest("invalid id".to_string()))?;
    match state.services.del_user(uid).await? {
        Some(user) => Ok((StatusCode::OK, Json(UserRes::from(user)))),
        None => Err(AppError::NotFound),
    }
}
