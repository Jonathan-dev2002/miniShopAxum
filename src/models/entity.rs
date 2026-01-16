use chrono::{Date, DateTime, Utc};
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
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(sqlx::FromRow)]
pub struct ProductWithCategory {
    #[sqlx(flatten)] // ProductEntity ให้เทรวมมาอยู่ชั้นนี้เลย"
    pub product: ProductEntity,

    pub category_name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CartsEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CartItemsEntity {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CartWithItems {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<CartItemDetail>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CartItemDetail {
    pub item_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub price: Decimal,
    pub quantity: i32,
}
