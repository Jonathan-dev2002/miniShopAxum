use crate::models::entity::{CartItemDetail, CartsEntity};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct CartRepository {
    pool: Pool<Postgres>,
}

impl CartRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn get_or_create_cart_id(&self, user_id: Uuid) -> Result<Uuid, sqlx::Error> {
        let cart = sqlx::query_as!(
            CartsEntity,
            "SELECT * FROM carts WHERE user_id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(c) = cart {
            Ok(c.id)
        } else {
            let new_cart = sqlx::query!(
                "INSERT INTO carts (user_id) VALUES ($1) RETURNING id",
                user_id
            )
            .fetch_one(&self.pool)
            .await?;
            Ok(new_cart.id)
        }
    }

    pub async fn find_cart_items(&self, cart_id: Uuid) -> Result<Vec<CartItemDetail>, sqlx::Error> {
        sqlx::query_as!(
            CartItemDetail,
            r#"
            SELECT 
                ci.id as item_id,
                ci.product_id,
                p.name as product_name,
                p.price as "price: rust_decimal::Decimal",
                ci.quantity
            FROM cart_items ci
            JOIN products p ON ci.product_id = p.id
            WHERE ci.cart_id = $1
            ORDER BY ci.created_at ASC
            "#,
            cart_id
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn upsert_item(
        &self,
        cart_id: Uuid,
        product_id: Uuid,
        quantity: i32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO cart_items (cart_id, product_id, quantity)
            VALUES ($1, $2, $3)
            ON CONFLICT (cart_id, product_id) 
            DO UPDATE SET 
                quantity = cart_items.quantity + $3,
                updated_at = NOW()
            "#,
            cart_id,
            product_id,
            quantity
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_item_quantity(
        &self,
        item_id: Uuid,
        quantity: i32,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE cart_items SET quantity = $1, updated_at = NOW() WHERE id = $2",
            quantity,
            item_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn delete_item(&self, item_id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM cart_items WHERE id = $1", item_id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}
