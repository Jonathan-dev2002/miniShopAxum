use crate::config::AppState;
use crate::models::{
    dto::{FilterOptions, ProductRequest, UpdateProductRequest},
    error::AppError,
    response::ApiResponse,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn list_products_handler(
    State(state): State<AppState>,
    Query(opts): Query<FilterOptions>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.products_service.list_products(opts).await?;
    Ok(ApiResponse::success(
        response,
        "1000",
        "List products successfully.",
    ))
}

pub async fn create_product_handler(
    State(state): State<AppState>,
    Json(payload): Json<ProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    let product = state.products_service.create_product(payload).await?;
    Ok(ApiResponse::success(
        product,
        "1000",
        "Create product successfully.",
    ))
}

pub async fn get_product_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let product = state.products_service.get_product_by_id(id).await?;
    Ok(ApiResponse::success(
        product,
        "1000",
        "Get product successfully.",
    ))
}

pub async fn update_product_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateProductRequest>,
) -> Result<impl IntoResponse, AppError> {
    let product = state.products_service.update_product(id, payload).await?;
    Ok(ApiResponse::success(
        product,
        "1000",
        "Update product successfully.",
    ))
}

pub async fn delete_product_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    state.products_service.delete_product(id).await?;
    Ok(ApiResponse::<()>::success_no_data(
        "1000",
        "Delete product successfully.",
    ))
}
