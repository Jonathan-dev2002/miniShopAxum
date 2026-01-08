use crate::models::dto::{CategoryRequest, UpdateCategoryRequest};
use crate::models::error::AppError;
use crate::models::response::ApiResponse;
use crate::{config::AppState, models::dto::FilterOptions};
use axum::extract::Path;
use axum::{
    Extension, Json,
    extract::{Query, State},
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn get_categories_handler(
    State(state): State<AppState>,
    Query(opts): Query<FilterOptions>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.categories_service.list_categories(opts).await?;

    Ok(ApiResponse::success(
        response,
        "1000",
        "List categories successfully.",
    ))
}

pub async fn create_categories_handler(
    State(state): State<AppState>,
    Json(payload): Json<CategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    state.categories_service.create_category(payload).await?;

    Ok(ApiResponse::<()>::success_no_data(
        "1000",
        "Create category successfully.",
    ))
}

pub async fn get_category_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let category = state.categories_service.get_categories_by_id(id).await?;

    Ok(ApiResponse::success(
        category,
        "1000",
        "Get category successfully.",
    ))
}

pub async fn delete_category_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let delete = state
        .categories_service
        .delete_categories(id)
        .await?;

    Ok(ApiResponse::<()>::success_no_data(
        "1000",
        "Delete category successfully.",
    ))
}

pub async fn update_categories_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let update_data = state.categories_service.update_categories(id, payload).await?;

    Ok(ApiResponse::success(
        update_data,
        "1000",
        "Update category successfully.",
    ))
}