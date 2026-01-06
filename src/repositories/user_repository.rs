use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::models::entity::UserEntity;

// สไตล์ Rust จะนิยมสร้าง Struct แล้วแปะ Method ใส่เลย
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, username: &str, password_hash: &str) -> Result<UserEntity, sqlx::Error> {
        sqlx::query_as!(
            UserEntity,
            "INSERT INTO users (id, username, password_hash, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
            Uuid::new_v4(),
            username,
            password_hash,
            chrono::Utc::now()
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<UserEntity>, sqlx::Error> {
        sqlx::query_as!(
            UserEntity,
            "SELECT * FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await
    }
}