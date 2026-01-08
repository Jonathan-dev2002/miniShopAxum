use crate::models::dto::{LoginRequest, LoginResponse, RegisterRequest};
use crate::models::error::AppError;
use crate::repositories::user_repository::UserRepository;
use crate::utils::jwt;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let repo = UserRepository::new(pool);
        Self { repo }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<(), AppError> {
        //Hash Password
        let hash = bcrypt::hash(req.password, 4)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        //Save to DB
        self.repo
            .create_user(&req.username, &hash)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub async fn login(&self, req: LoginRequest) -> Result<LoginResponse, AppError> {
        //Find User
        let user = self
            .repo
            .find_by_username(&req.username)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or(AppError::AuthError("User not found".into()))?;

        //Verify Password
        let valid = bcrypt::verify(req.password, &user.password_hash).unwrap_or(false);
        if !valid {
            return Err(AppError::AuthError("Invalid password".into()));
        }

        // 3. Generate JWT
        let token =
            jwt::encode_jwt(user.id).map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(LoginResponse { token })
    }
}
