use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::entity::{
    CartItemDetail, CartWithItems, CategoryEntity, ProductEntity, ProductWithCategory,
};

// Request
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    // pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct CategoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct ProductRequest {
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub price: Decimal,
    pub stock: i32,
}

#[derive(Deserialize)]
pub struct UpdateProductRequest {
    pub category_id: Option<Uuid>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
    pub price: Option<Decimal>,
    pub stock: Option<i32>,
}
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct CategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<CategoryEntity> for CategoryResponse {
    fn from(entity: CategoryEntity) -> Self {
        Self {
            id: entity.id,
            name: entity.name,
            description: entity.description,
            is_active: entity.is_active,
            created_at: entity.created_at,
            updated_at: entity.updated_at,
        }
    }
}

#[derive(Serialize)]
pub struct ProductResponse {
    pub id: Uuid,
    pub category_id: Uuid,
    pub category_name: String,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub price: Decimal,
    pub stock: i32,
    pub average_rating: f64,
    pub review_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<ProductWithCategory> for ProductResponse {
    fn from(data: ProductWithCategory) -> Self {
        Self {
            id: data.product.id,
            category_id: data.product.category_id,
            category_name: data.category_name,
            name: data.product.name,
            description: data.product.description,
            is_active: data.product.is_active,
            price: data.product.price,
            stock: data.product.stock,
            average_rating: data.product.average_rating,
            review_count: data.product.review_count,
            created_at: data.product.created_at,
            updated_at: data.product.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: usize,
    pub limit: usize,
    pub total_pages: i64,
}

// Cart
#[derive(Deserialize)]
pub struct AddToCartRequest {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Deserialize)]
pub struct UpdateCartItemRequest {
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct CartItemResponse {
    pub item_id: Uuid,
    pub product_id: Uuid,
    pub product_name: String,
    pub price: Decimal,
    pub quantity: i32,
    pub subtotal: Decimal,
}

#[derive(Serialize)]
pub struct CartResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub items: Vec<CartItemResponse>,
    pub total_price: Decimal, 
    pub total_items: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProductSearchDocument {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub price: Decimal,
    pub category_id: Uuid,
    pub image_url: Option<String>,
}

// Implement trait เพื่อระบุว่า field ไหนคือ ID (Primary Key ใน Meilisearch)
impl ProductSearchDocument {
    pub const INDEX_NAME: &'static str = "products";
}