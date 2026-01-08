use crate::models::{dto::FilterOptions, entity::UserEntity};
use sqlx::{Pool, Postgres, QueryBuilder};
use uuid::Uuid;

#[derive(Clone)]
pub struct UserRepository {
    pool: Pool<Postgres>,
}

impl UserRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_user(
        &self,
        username: &str,
        password_hash: &str,
    ) -> Result<UserEntity, sqlx::Error> {
        sqlx::query_as!(
            UserEntity,
            "INSERT INTO users (id, username, password_hash, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
            Uuid::new_v4(),
            username,
            password_hash,
            chrono::Utc::now()
        )
        .fetch_one(&self.pool) //ต้องเจอ 1 แถวเท่านั้น (ถ้าไม่เจอจะ Error)
        .await
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<UserEntity>, sqlx::Error> {
        sqlx::query_as!(
            UserEntity,
            "SELECT * FROM users WHERE username = $1",
            username
        )
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<UserEntity>, sqlx::Error> {
        sqlx::query_as!(UserEntity, "SELECT * FROM users WHERE id = $1", user_id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn list_users(&self) -> Result<Vec<UserEntity>, sqlx::Error> {
        sqlx::query_as!(UserEntity, "SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool) //ขอมาเป็น List (Vec)
            .await
    }

    pub async fn find_all(
        &self,
        opts: FilterOptions,
    ) -> Result<(Vec<UserEntity>, i64), sqlx::Error> {
        // กำหนดค่า Default
        let page = opts.page.unwrap_or(1);
        let limit = opts.limit.unwrap_or(10);
        let offset = (page - 1) * limit; 

        // สร้าง Base Query สำหรับดึงข้อมูล
        let mut qb = QueryBuilder::new("SELECT * FROM users WHERE 1 = 1");

        // ถ้ามี Search ให้เพิ่มเงื่อนไข
        if let Some(search) = &opts.search {
            qb.push(" AND username ILIKE "); // ILIKE ไม่สนตัวพิมพ์เล็กใหญ่
            qb.push_bind(format!("%{}%", search)); // bind ค่าเพื่อกัน SQL Injection
        }

        // การ Sort (ต้องระวัง SQL Injection ตรงชื่อ Column!)
        if let Some(sort_by) = &opts.sort_by {
            let order_col = match sort_by.as_str() {
                "username" => "username",
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

        // Execute Query เพื่อเอา Data
        let users = qb
            .build_query_as::<UserEntity>()
            .fetch_all(&self.pool)
            .await?;

        // Total Count
        let mut count_qb = QueryBuilder::new("SELECT COUNT(*) FROM users WHERE 1 = 1");

        if let Some(search) = &opts.search {
            count_qb.push(" AND username ILIKE ");
            count_qb.push_bind(format!("%{}%", search));
        }

        let count_row: (i64,) = count_qb.build_query_as().fetch_one(&self.pool).await?;

        let total = count_row.0;

        Ok((users, total))
    }

    pub async fn update_user(
        &self,
        user_id: Uuid,
        new_username: Option<&str>,
    ) -> Result<UserEntity, sqlx::Error> {
        let now = chrono::Utc::now();
        sqlx::query_as!(
            UserEntity,
            r#"
        UPDATE users 
        SET 
            username = COALESCE($1, username), 
            updated_at = $2 
        WHERE id = $3 
        RETURNING *
        "#,
            new_username,
            now,
            user_id
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_user(&self, user_id: Uuid) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
