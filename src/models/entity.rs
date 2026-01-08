use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::types::Decimal;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CategoryEntity {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct ProductEntity {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    pub average_rating: f64,
    pub review_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
pub struct ProductWithCategory {
    #[sqlx(flatten)] // ProductEntity ให้เทรวมมาอยู่ชั้นนี้เลย"
    pub product: ProductEntity,
    
    pub category_name: String,
}
