use middleware::{ctx::LoginUser, jwt::Claims};

pub async fn example_user() -> String {
    let claims = Claims::build("sub", "1", "test");
    let token = claims.to_token().unwrap();
    token
}

pub async fn example_user_info(user: LoginUser) -> String {
    format!("Hello, {}! Your user_id is {}. Token exp: {}", user.username, user.user_id, user.exp)
}

pub async fn health() -> String {
    "server is ok".into()
}
