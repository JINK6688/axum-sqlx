use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use axum::{
    http::Request,
    response::{IntoResponse, Response},
};
use chrono::Utc;
use configure::{error::AppError, CONFIG};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tower::{Layer, Service};
use tracing::error;

use crate::ctx::LoginUser;

/// JWT Claims结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub user_id: String,
    pub exp: i64,
}

impl Claims {
    /// 构建Claims
    pub fn build(sub: &str, user_id: &str, username: &str) -> Self {
        let token_exp = Utc::now().timestamp() + (CONFIG.jwt.expired as i64) * 3600;
        Claims {
            sub: sub.to_string(),
            username: username.to_string(),
            user_id: user_id.to_string(),
            exp: token_exp,
        }
    }

    /// 生成JWT token
    pub fn to_token(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let jwt_secret = &CONFIG.jwt.secret;
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(jwt_secret.as_bytes());
        let mut header = jsonwebtoken::Header::default();
        header.alg = jsonwebtoken::Algorithm::HS256;
        jsonwebtoken::encode(&header, self, &encoding_key)
    }

    pub fn to_login_user(&self) -> LoginUser {
        LoginUser { user_id: self.user_id.clone(), username: self.username.clone(), exp: self.exp }
    }
}

/// JWT中间件层
#[derive(Clone)]
pub struct JwtLayer;

impl JwtLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for JwtLayer {
    type Service = JwtMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtMiddleware { inner }
    }
}

/// JWT 中间件实现
#[derive(Clone)]
pub struct JwtMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for JwtMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let token = req
                .headers()
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .and_then(|h| h.strip_prefix("Bearer ").or_else(|| Some(h)));

            if token.is_none() {
                let resp = AppError::Unauthorized("Missing token".into()).into_response();
                return Ok(resp);
            }

            let decoding_key = DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes());
            let validation = Validation::new(Algorithm::HS256);

            match decode::<Claims>(token.unwrap(), &decoding_key, &validation) {
                Ok(data) => {
                    let user = data.claims.to_login_user();
                    req.extensions_mut().insert(user);
                    inner.call(req).await
                }
                Err(e) => {
                    let resp =
                        AppError::Unauthorized(format!("Invalid token: {e}")).into_response();
                    error!("JWT Error: {}", e);
                    Ok(resp)
                }
            }
        })
    }
}
