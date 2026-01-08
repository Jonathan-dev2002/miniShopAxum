use crate::{config::AppState, models::dto::FilterOptions};
use crate::models::dto::UpdateUserRequest;
use crate::models::error::AppError;
use crate::models::response::ApiResponse;
use crate::utils::jwt::Claims;
use axum::{
    Extension, //ใช้ดึงข้อมูลจาก Middleware
    Json,
    extract::{Query, State},
    response::IntoResponse,
};

//GET /users/me
pub async fn get_me_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    // let service = UserService::new(state.db);

    // แปลง String ID จาก Token กลับเป็น Uuid
    // let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::AuthError("Invalid User ID in token".into()))?;
    let user_id = claims.get_user_id()?;

    let user = state.user_service.get_current_user(user_id).await?;

    Ok(ApiResponse::success(
        user,
        "1000",
        "Get profile successfully.",
    ))
}

//GET /users
pub async fn list_users_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = state.user_service.list_users().await?;

    Ok(ApiResponse::success(
        users,
        "1000",
        "List users successfully.",
    ))
}

//PUT /users/me
pub async fn update_me_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.get_user_id()?;
    let updated_user = state.user_service.update_user(user_id, payload).await?;

    Ok(ApiResponse::success(
        updated_user,
        "1000",
        "Update profile successfully.",
    ))
}

//DELETE /users/me
pub async fn delete_me_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = claims.get_user_id()?;
    state.user_service.delete_user(user_id).await?;

    Ok(ApiResponse::<()>::success_no_data(
        "1000",
        "User deleted successfully.",
    ))
}

pub async fn get_users_handler(
    State(state): State<AppState>,
    Query(opts): Query<FilterOptions>, //Query extractor
) -> Result<impl IntoResponse, AppError> {
    let response = state.user_service.get_users(opts).await?;

    Ok(ApiResponse::success(
        response,
        "1000",
        "Get users list successfully.",
    ))
}
