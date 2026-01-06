use axum::{extract::State, Json, response::IntoResponse};
use crate::config::AppState;
use crate::models::dto::{RegisterRequest, LoginRequest};
use crate::services::auth_service::AuthService;
use crate::models::error::AppError;
use crate::models::response::ApiResponse;

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = AuthService::new(state.db);
    service.register(payload).await?;
    
    let response = ApiResponse::<()>::success_no_data("1000", "Register successfully.");
    Ok(response)
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = AuthService::new(state.db);
    
    let login_data = service.login(payload).await?; 
    
    let response = ApiResponse::success(login_data, "1000", "Login successfully.");
    
    Ok(response)
}