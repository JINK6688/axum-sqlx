use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use serde::{Deserialize, Serialize};

/// 登录用户信息（从JWT中提取）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub user_id: String,
    pub username: String,
    pub exp: i64,
}

#[async_trait]
impl<S> FromRequestParts<S> for LoginUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<LoginUser>()
            .cloned()
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Invalid or missing JWT".to_string()))
    }
}
