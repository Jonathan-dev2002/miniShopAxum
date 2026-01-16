use crate::config::AppState;
use crate::models::dto::{AddToCartRequest, UpdateCartItemRequest};
use crate::models::error::AppError;
use crate::models::response::ApiResponse;
use axum::{
    extract::{Path, State},
    Extension, Json,
    response::IntoResponse,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Claims {
    pub user_id: Uuid,
}

pub async fn get_cart_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, AppError> {
    let response = state.cart_service.get_cart(claims.user_id).await?;

    Ok(ApiResponse::success(
        response,
        "1000",
        "Get cart successfully.",
    ))
}

pub async fn add_to_cart_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<AddToCartRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .cart_service
        .add_to_cart(claims.user_id, payload)
        .await?;

    Ok(ApiResponse::success(
        response,
        "1000",
        "Add item to cart successfully.",
    ))
}

pub async fn update_cart_item_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCartItemRequest>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .cart_service
        .update_item(claims.user_id, id, payload)
        .await?;

    Ok(ApiResponse::success(
        response,
        "1000",
        "Update cart item successfully.",
    ))
}

pub async fn remove_cart_item_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let response = state
        .cart_service
        .remove_item(claims.user_id, id)
        .await?;

    Ok(ApiResponse::success(
        response,
        "1000",
        "Remove cart item successfully.",
    ))
}