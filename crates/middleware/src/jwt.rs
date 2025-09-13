use std::task::{Context, Poll};

use axum::{http::Request, response::Response};
use chrono::Utc;
/// 你的全局配置模块
use configure::CONFIG;
use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tower::{Layer, Service};

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
}

impl Claims {
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

/// JWT中间件实现
#[derive(Clone)]
pub struct JwtMiddleware<S> {
    inner: S,
}

impl<S, ReqBody> Service<Request<ReqBody>> for JwtMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response> + Clone + Send + Sync + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        // 提取token并校验
        let auth_header =
            req.headers().get(axum::http::header::AUTHORIZATION).and_then(|v| v.to_str().ok());

        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => &header[7..],
            _ => "",
        };

        let mut valid_user: Option<LoginUser> = None;

        if !token.is_empty() {
            let jwt_secret = &CONFIG.jwt.secret;
            let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
            let validation = Validation::new(Algorithm::HS256);
            let token_data: Result<TokenData<Claims>, _> =
                decode::<Claims>(token, &decoding_key, &validation);

            match token_data {
                Ok(data) => {
                    valid_user = Some(data.claims.to_login_user());
                }
                Err(e) => {
                    if let jsonwebtoken::errors::ErrorKind::ExpiredSignature = e.kind() {
                        tracing::warn!("JWT token expired");
                        // 这里可以考虑直接返回 401 响应，或者在 request
                        // 扩展中标记为过期
                    } else {
                        tracing::error!("JWT decode error: {:?}", e);
                    }
                }
            }
        }

        // 将LoginUser注入request扩展
        if let Some(user) = valid_user {
            req.extensions_mut().insert(user);
        }

        self.inner.call(req)
    }
}
