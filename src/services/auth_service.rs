use crate::repositories::user_repository::UserRepository;
use crate::models::dto::{RegisterRequest, LoginRequest, LoginResponse};
use crate::models::error::AppError; // Custom Error ของเรา
use crate::utils::jwt;
use sqlx::{Pool, Postgres};

pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let repo = UserRepository::new(pool);
        Self { repo }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<(), AppError> {
        // 1. Hash Password
        let hash = bcrypt::hash(req.password, 4).map_err(|e| AppError::InternalServerError(e.to_string()))?;
        
        // 2. Save to DB
        self.repo.create_user(&req.username, &hash).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        Ok(())
    }

    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        // 1. Find User
        let user = self.repo.find_by_username(&req.username).await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or(AppError::AuthError("User not found".into()))?;

        // 2. Verify Password
        let valid = bcrypt::verify(req.password, &user.password_hash).unwrap_or(false);
        if !valid {
            return Err(AppError::AuthError("Invalid password".into()));
        }

        // 3. Generate JWT
        let token = jwt::encode_jwt(user.id).map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(LoginResponse { token })
    }
}