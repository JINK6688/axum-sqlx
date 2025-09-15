use anyhow::Result;
use repositroy::{
    entity::user::{create_user, del_user, get_user, list_users, update_user, User},
    PgPool,
};
use tracing::instrument;
use uuid::Uuid;

use super::*;

impl Services {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    #[instrument(skip(self))]
    pub async fn create_user(&self, email: &str, name: &str) -> Result<User> {
        create_user(&self.pool, email, name).await
    }

    #[instrument(skip(self))]
    pub async fn update_user(&self, user: &User) -> Result<User> {
        update_user(&self.pool, user).await
    }

    #[instrument(skip(self))]
    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>> {
        get_user(&self.pool, id).await
    }

    #[instrument(skip(self))]
    pub async fn list_users(&self, limit: i64, offset: i64) -> Result<Vec<User>> {
        list_users(&self.pool, limit, offset).await
    }

    #[instrument(skip(self))]
    pub async fn del_user(&self, id: Uuid) -> Result<Option<User>> {
        del_user(&self.pool, id).await
    }
}
