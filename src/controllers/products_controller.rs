use crate::config::AppState;
use crate::models::{
    dto::{FilterOptions, ProductRequest, ProductSearchDocument, UpdateProductRequest},
    error::AppError,
    response::ApiResponse,
};
use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use meilisearch_sdk::search::SearchResults;
use uuid::Uuid;
#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub filter: Option<String>,
    pub sort: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
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

    // เรียกแบบ Fire-and-forget
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

    let search_doc = ProductSearchDocument {
        id: product.id,
        name: product.name.clone(),
        description: product.description.clone().unwrap_or_default(),
        price: product.price,
        category_id: product.category_id,
        image_url: None,
    };
    let _ = state.search_service.add_product(search_doc).await;

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
    let _ = state.search_service.delete_product(id).await;
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
    let results = state.search_service.search_products(params).await?;

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

pub async fn reindex_handler(State(state): State<AppState>) -> Result<impl IntoResponse, AppError> {
    let batch_size = 1000;
    let mut page = 1;

    state.search_service.delete_all_documents().await?;

    loop {
        let filter = FilterOptions {
            page: Some(page),
            limit: Some(batch_size),
            search: None,
            sort_by: Some("created_at".to_string()),
            sort_dir: Some("asc".to_string()),
            is_active: None,
        };

        let products_response = state.products_service.list_products(filter).await?;
        let products = products_response.data;

        if products.is_empty() {
            break;
        }

        // แปลงข้อมูล
        let docs: Vec<ProductSearchDocument> = products
            .into_iter()
            .map(|p| ProductSearchDocument {
                id: p.id,
                name: p.name,
                description: p.description.unwrap_or_default(),
                price: p.price,
                category_id: p.category_id,
                image_url: None,
            })
            .collect();

        // ถ้าใช้ blocking ในลูปนี้ API อาจจะ Timeout ได้ถ้าข้อมูลเยอะจัด
        // แนะนำให้ใช้ add_documents ธรรมดา แล้วปล่อยให้ Meilisearch จัดคิวเอง
        state.search_service.add_documents(&docs).await?;

        // ขยับไปหน้าถัดไป
        page += 1;
    }

    Ok(ApiResponse::<()>::success_no_data(
        "1000",
        "Sync Completed! Index is now up-to-date.",
    ))
}

pub async fn create_products_bulk_handler(
    State(state): State<AppState>,
    Json(payload): Json<Vec<ProductRequest>>,
) -> Result<impl IntoResponse, AppError> {
    let products = state.products_service.create_products_bulk(payload).await?;

    let search_docs: Vec<ProductSearchDocument> = products
        .iter()
        .map(|p| ProductSearchDocument {
            id: p.id,
            name: p.name.clone(),
            description: p.description.clone().unwrap_or_default(),
            price: p.price,
            category_id: p.category_id,
            image_url: None,
        })
        .collect();

    state.search_service.add_documents(&search_docs).await?;

    Ok(ApiResponse::success(
        products,
        "1000",
        "Bulk create products successfully.",
    ))
}
