use crate::models::{
    dto::{CategoryRequest, FilterOptions, UpdateCategoryRequest},
    entity::CategoryEntity,
};
use axum::extract::Query;
use sqlx::{Pool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct CategoriesRepository {
    pool: Pool<Postgres>,
}

impl CategoriesRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_category(
        &self,
        req: CategoryRequest,
    ) -> Result<CategoryEntity, sqlx::Error> {
        sqlx::query_as!(
            CategoryEntity,
            r#"
            INSERT INTO categories (name, description, is_active) 
            VALUES ($1, $2, $3) 
            RETURNING *
            "#,
            req.name,
            req.description,
            req.is_active.unwrap_or(true)
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_all(
        &self,
        opts: FilterOptions,
    ) -> Result<(Vec<CategoryEntity>, i64), sqlx::Error> {
        let page = opts.page.unwrap_or(1);
        let limit = opts.limit.unwrap_or(10);
        let offset = (page - 1) * limit;

        let active_filter = opts.is_active.unwrap_or(true);

        let mut qb = QueryBuilder::new("SELECT * FROM categories WHERE 1 = 1");

        if let Some(search) = &opts.search {
            qb.push(" AND name ILIKE ");
            qb.push_bind(format!("%{}%", search));
        }

        qb.push(" AND is_active = ");
        qb.push_bind(active_filter);

        // Sorting
        if let Some(sort_by) = &opts.sort_by {
            let order_col = match sort_by.as_str() {
                "name" => "name",
                "created_at" => "created_at",
                _ => "created_at",
            };

            let dir = if opts.sort_dir.as_deref() == Some("asc") {
                "ASC"
            } else {
                "DESC"
            };

            qb.push(format!(" ORDER BY {} {}", order_col, dir));
        } else {
            qb.push(" ORDER BY created_at DESC");
        }

        // Pagination
        qb.push(" LIMIT ");
        qb.push_bind(limit as i64);
        qb.push(" OFFSET ");
        qb.push_bind(offset as i64);

        // Execute Main Query
        let categories = qb
            .build_query_as::<CategoryEntity>()
            .fetch_all(&self.pool)
            .await?;

        let mut count_qb = QueryBuilder::new("SELECT COUNT(*) FROM categories WHERE 1 = 1");

        if let Some(search) = &opts.search {
            count_qb.push(" AND name ILIKE ");
            count_qb.push_bind(format!("%{}%", search));
        }

        count_qb.push(" AND is_active = ");
        count_qb.push_bind(active_filter);

        let count_row: (i64,) = count_qb.build_query_as().fetch_one(&self.pool).await?;

        let total = count_row.0;

        Ok((categories, total))
    }

    pub async fn find_by_id(
        &self,
        categories_id: Uuid,
    ) -> Result<Option<CategoryEntity>, sqlx::Error> {
        sqlx::query_as!(
            CategoryEntity,
            "SELECT * FROM categories WHERE id = $1",
            categories_id
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_categories(
        &self,
        categories_id: Uuid,
        data: UpdateCategoryRequest,
    ) -> Result<CategoryEntity, sqlx::Error> {
        sqlx::query_as!(
            CategoryEntity,
            r#"
            UPDATE categories 
            SET
                name = COALESCE($1, name),
                description = COALESCE($2, description),
                is_active = COALESCE($3, is_active),
                updated_at = NOW()
            WHERE id = $4
            RETURNING *
            "#,
            data.name,
            data.description,
            data.is_active,
            categories_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn soft_delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE categories 
            SET is_active = false, 
                updated_at = NOW() 
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }
}
