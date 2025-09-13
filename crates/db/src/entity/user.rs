use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{self, PgPool};
// Data model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

// Queries
pub async fn create_user(pool: &PgPool, email: &str, name: &str) -> Result<User> {
    let rec = sqlx::query_as!(
        User,
        r#"INSERT INTO users (email, name)
           VALUES ($1, $2)
           RETURNING id, email, name, created_at"#,
        email,
        name
    )
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

pub async fn update_user(pool: &PgPool, user: &User) -> Result<User> {
    let rec = sqlx::query_as!(
        User,
        r#"UPDATE users
        SET email = $1, name = $2
           where id = $3
           RETURNING id, email, name, created_at"#,
        user.email,
        user.name,
        user.id
    )
    .fetch_one(pool)
    .await?;
    Ok(rec)
}

pub async fn get_user(pool: &PgPool, id: uuid::Uuid) -> Result<Option<User>> {
    let rec =
        sqlx::query_as!(User, r#"SELECT id, email, name, created_at FROM users WHERE id = $1"#, id)
            .fetch_optional(pool)
            .await?;
    Ok(rec)
}

pub async fn list_users(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<User>> {
    let rows = sqlx::query_as!(
        User,
        r#"SELECT id, email, name, created_at
           FROM users
           ORDER BY created_at DESC
           LIMIT $1 OFFSET $2"#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn del_user(pool: &PgPool, id: uuid::Uuid) -> Result<Option<User>> {
    let user = sqlx::query_as!(User, r#"DELETE FROM users WHERE id = $1 RETURNING *"#, id)
        .fetch_optional(pool)
        .await?;

    Ok(user)
}
