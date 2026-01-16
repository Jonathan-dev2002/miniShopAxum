use crate::models::{
    dto::{FilterOptions, ProductRequest, UpdateProductRequest},
    entity::{ProductEntity, ProductWithCategory},
};
use sqlx::{Pool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct ProductsRepository {
    pool: Pool<Postgres>,
}

impl ProductsRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_product(&self, req: ProductRequest) -> Result<ProductEntity, sqlx::Error> {
        sqlx::query_as!(
            ProductEntity,
            r#"
            INSERT INTO products 
            (category_id, name, description, price, stock, is_active, average_rating, review_count) 
            VALUES ($1, $2, $3, $4, $5, $6, 0.0, 0) -- default rating=0
            RETURNING *
            "#,
            req.category_id,
            req.name,
            req.description,
            req.price,
            req.stock,
            req.is_active.unwrap_or(true)
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn list_all(
        &self,
        opts: FilterOptions,
    ) -> Result<(Vec<ProductWithCategory>, i64), sqlx::Error> {
        let page = opts.page.unwrap_or(1);
        let limit = opts.limit.unwrap_or(10);
        let offset = (page - 1) * limit;
        let active_filter = opts.is_active.unwrap_or(true);

        // JOIN categories
        let base_sql = "
            SELECT p.*, c.name as category_name 
            FROM products p
            JOIN categories c ON p.category_id = c.id
            WHERE 1 = 1
        ";

        let mut qb = QueryBuilder::new(base_sql);

        // Filter: Search Product Name
        if let Some(search) = &opts.search {
            qb.push(" AND p.name ILIKE ");
            qb.push_bind(format!("%{}%", search));
        }

        // Filter: Is Active
        qb.push(" AND p.is_active = ");
        qb.push_bind(active_filter);

        // Sort
        if let Some(sort_by) = &opts.sort_by {
            let order_col = match sort_by.as_str() {
                "name" => "p.name",
                "price" => "p.price",
                "stock" => "p.stock",
                "created_at" => "p.created_at",
                _ => "p.created_at",
            };
            let dir = if opts.sort_dir.as_deref() == Some("asc") {
                "ASC"
            } else {
                "DESC"
            };
            qb.push(format!(" ORDER BY {} {}", order_col, dir));
        } else {
            qb.push(" ORDER BY p.created_at DESC");
        }

        // Pagination
        qb.push(" LIMIT ");
        qb.push_bind(limit as i64);
        qb.push(" OFFSET ");
        qb.push_bind(offset as i64);

        let products = qb
            .build_query_as::<ProductWithCategory>() // ✅ ใช้ Struct ที่มี flatten
            .fetch_all(&self.pool)
            .await?;

        // Count Total
        let mut count_qb = QueryBuilder::new("SELECT COUNT(*) FROM products p WHERE 1 = 1");
        if let Some(search) = &opts.search {
            count_qb.push(" AND p.name ILIKE ");
            count_qb.push_bind(format!("%{}%", search));
        }
        count_qb.push(" AND p.is_active = ");
        count_qb.push_bind(active_filter);

        let count_row: (i64,) = count_qb.build_query_as().fetch_one(&self.pool).await?;

        Ok((products, count_row.0))
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<ProductWithCategory>, sqlx::Error> {
        sqlx::query_as::<_, ProductWithCategory>(
            r#"
            SELECT p.*, c.name as category_name 
            FROM products p
            JOIN categories c ON p.category_id = c.id
            WHERE p.id = $1
            "#
        ) 
        .bind(id) 
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_product(
        &self,
        id: Uuid,
        req: UpdateProductRequest,
    ) -> Result<ProductEntity, sqlx::Error> {
        // Update ใช้ COALESCE (Patch)
        sqlx::query_as!(
            ProductEntity,
            r#"
            UPDATE products
            SET 
                category_id = COALESCE($1, category_id),
                name = COALESCE($2, name),
                description = COALESCE($3, description),
                price = COALESCE($4, price),
                stock = COALESCE($5, stock),
                is_active = COALESCE($6, is_active),
                updated_at = NOW()
            WHERE id = $7
            RETURNING *
            "#,
            req.category_id,
            req.name,
            req.description,
            req.price,
            req.stock,
            req.is_active,
            id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn soft_delete(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE products SET is_active = false, updated_at = NOW() WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}
