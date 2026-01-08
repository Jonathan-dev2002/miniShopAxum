use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::entity::CategoryEntity;

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
// Response
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
#[derive(Debug, Serialize)]
pub struct PagedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: usize,
    pub limit: usize,
    pub total_pages: i64,
}
