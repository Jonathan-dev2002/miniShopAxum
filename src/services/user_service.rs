use crate::models::dto::{FilterOptions, UpdateUserRequest, PagedResponse, UserResponse};
use crate::models::error::AppError;
use crate::repositories::user_repository::UserRepository;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let repo = UserRepository::new(pool);
        Self { repo }
    }

    // Get Current User Profile
    pub async fn get_current_user(&self, user_id: Uuid) -> Result<UserResponse, AppError> {
        let user = self
            .repo
            .find_by_id(user_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or(AppError::NotFound("User not found".into()))?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    // List Users
    pub async fn list_users(&self) -> Result<Vec<UserResponse>, AppError> {
        let users = self
            .repo
            .list_users()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let user_responses = users
            .into_iter()
            .map(|user| UserResponse {
                id: user.id,
                username: user.username,
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
            .collect();

        Ok(user_responses)
    }

    // Update User
    pub async fn update_user(
        &self,
        user_id: Uuid,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        let new_username = req
            .username
            .ok_or(AppError::ValidationError("Username is required".into()))?;

        let updated_user = self
            .repo
            .update_user(user_id, Some(&new_username))
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(UserResponse {
            id: updated_user.id,
            username: updated_user.username,
            created_at: updated_user.created_at,
            updated_at: updated_user.updated_at,
        })
    }

    // Delete User
    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), AppError> {
        self.repo
            .delete_user(user_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    // Get Users with Pagination and Filtering
    pub async fn get_users(
        &self,
        opts: FilterOptions,
    ) -> Result<PagedResponse<UserResponse>, AppError> {
        let limit = opts.limit.unwrap_or(10);
        let page = opts.page.unwrap_or(1);

        let (users, total) = self
            .repo
            .find_all(opts)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // แปลง Entity -> Response DTO
        let user_responses: Vec<UserResponse> = users
            .into_iter()
            .map(|u| UserResponse {
                id: u.id,
                username: u.username,
                created_at: u.created_at,
                updated_at: u.updated_at,
            })
            .collect();

        // คำนวณจำนวนหน้าทั้งหมด
        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        Ok(PagedResponse {
            data: user_responses,
            total,
            page,
            limit,
            total_pages,
        })
    }
}
