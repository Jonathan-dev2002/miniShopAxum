use crate::models::{
    dto::{
        FilterOptions, PagedResponse, ProductRequest, ProductResponse, ProductSearchDocument,
        UpdateProductRequest,
    },
    error::AppError,
};
use crate::repositories::products_repository::ProductsRepository;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct ProductsService {
    repo: ProductsRepository,
}

impl ProductsService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let repo = ProductsRepository::new(pool);
        Self { repo }
    }

    pub async fn create_product(&self, req: ProductRequest) -> Result<ProductResponse, AppError> {
        let created = self
            .repo
            .create_product(req)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.get_product_by_id(created.id).await // เรียกใช้ฟังก์ชัน get เพื่อเอา data สวยๆ
    }

    pub async fn list_products(
        &self,
        opts: FilterOptions,
    ) -> Result<PagedResponse<ProductResponse>, AppError> {
        let limit = opts.limit.unwrap_or(10);
        let page = opts.page.unwrap_or(1);

        let (products, total) = self
            .repo
            .list_all(opts)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let data: Vec<ProductResponse> = products.into_iter().map(ProductResponse::from).collect();

        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        Ok(PagedResponse {
            data,
            total,
            page,
            limit,
            total_pages,
        })
    }

    pub async fn get_product_by_id(&self, id: Uuid) -> Result<ProductResponse, AppError> {
        let product = self
            .repo
            .find_by_id(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or(AppError::NotFound("Product not found".into()))?;

        Ok(product.into())
    }

    pub async fn update_product(
        &self,
        id: Uuid,
        req: UpdateProductRequest,
    ) -> Result<ProductResponse, AppError> {
        let _updated = self
            .repo
            .update_product(id, req)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound("Product not found".into()),
                _ => AppError::DatabaseError(e.to_string()),
            })?;

        self.get_product_by_id(id).await
    }

    pub async fn delete_product(&self, id: Uuid) -> Result<(), AppError> {
        let deleted = self
            .repo
            .soft_delete(id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !deleted {
            return Err(AppError::NotFound("Product not found".into()));
        }
        Ok(())
    }
}
