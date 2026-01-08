use crate::models::dto::{
    CategoryRequest, CategoryResponse, FilterOptions, PagedResponse, UpdateCategoryRequest,
};
use crate::models::error::AppError;
use crate::repositories::categories_repository::CategoriesRepository;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct CategoriesService {
    repo: CategoriesRepository,
}

impl CategoriesService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let repo = CategoriesRepository::new(pool);
        Self { repo }
    }

    // Create Category
    pub async fn create_category(
        &self,
        req: CategoryRequest,
    ) -> Result<CategoryResponse, AppError> {
        let category = self
            .repo
            .create_category(req)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(category.into())
    }

    // List Categories with Filtering and Pagination
    pub async fn list_categories(
        &self,
        opts: FilterOptions,
    ) -> Result<PagedResponse<CategoryResponse>, AppError> {
        let limit = opts.limit.unwrap_or(10);
        let page = opts.page.unwrap_or(1);

        let (categories, total) = self
            .repo
            .list_all(opts)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        // แปลง Entity -> Response
        let category_responses: Vec<CategoryResponse> = categories
            .into_iter()
            .map(|category| CategoryResponse {
                id: category.id,
                name: category.name,
                description: category.description,
                is_active: category.is_active,
                created_at: category.created_at,
                updated_at: category.updated_at,
            })
            .collect();

        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        Ok(PagedResponse {
            data: category_responses,
            total,
            page,
            limit,
            total_pages,
        })
    }

    pub async fn get_categories_by_id(
        &self,
        categories_id: Uuid,
    ) -> Result<CategoryResponse, AppError> {
        let categories = self
            .repo
            .find_by_id(categories_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?
            .ok_or(AppError::NotFound("Categories not found".into()))?;

        Ok(categories.into())
    }

    pub async fn delete_categories(&self, categories_id: Uuid) -> Result<(), AppError> {
        let delete = self
            .repo
            .soft_delete(categories_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        if !delete {
            return Err(AppError::NotFound("Category not found".into()));
        }
        Ok(())
    }

    pub async fn update_categories(
        &self,
        categories_id: Uuid,
        req: UpdateCategoryRequest,
    ) -> Result<CategoryResponse, AppError> {
        let update = self
            .repo
            .update_categories(categories_id, req)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound("Category not found".into()),
                _ => AppError::DatabaseError(e.to_string()),
            })?;

        Ok(update.into())
    }
}
