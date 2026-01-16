use crate::config::AppState;
use crate::models::{
    dto::{FilterOptions, ProductRequest, UpdateProductRequest, ProductSearchDocument},
    error::AppError,
    response::ApiResponse,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub q: String,
}

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

    // ส่งข้อมูลใหม่ไป Meilisearch
    let search_doc = ProductSearchDocument {
        id: product.id,
        name: product.name.clone(),
        description: product.description.clone().unwrap_or_default(),
        price: product.price, 
        category_id: product.category_id,
        image_url: None, 
    };

    // เรียกแบบ Fire-and-forget (ไม่รอผลลัพธ์ก็ได้ ถ้าไม่อยากให้ API ช้า) 
    let _ = state.search_service.add_product(search_doc).await;

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

// GET /products/search?q=iphone
pub async fn search_products_handler(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<impl IntoResponse, AppError> {
    let results = state.search_service.search_products(params.q).await?;

    Ok(ApiResponse::success(
        results,
        "1000",
        "Search successfully.",
    ))
}

pub async fn sync_products_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {

    // ดึงสินค้าทั้งหมดจาก Database
    let filter = FilterOptions {
        page: Some(1),
        limit: Some(10000),
        search: None,   
        sort_by: None,   
        sort_dir: None, 
        is_active: None,
    };

    let products_response = state.products_service.list_products(filter).await?;
    let products = products_response.data;

    // แปลงข้อมูล ProductDto เป็น ProductSearchDocument
    let mut search_documents = Vec::new();

    for p in products {
        search_documents.push(ProductSearchDocument {
            id: p.id,
            name: p.name,
            description: p.description.unwrap_or_default(),
            price: p.price,
            category_id: p.category_id,
            image_url: None,
        });
    }

    // ส่งเข้า Meilisearch
    state
        .search_service
        .add_documents(&search_documents)
        .await?;

    Ok(ApiResponse::<()>::success_no_data(
        "1000",
        "Sync products to Meilisearch successfully.",
    ))
}
