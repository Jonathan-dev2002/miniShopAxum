use crate::models::dto::{
    AddToCartRequest, CartItemResponse, CartResponse, UpdateCartItemRequest,
};
use crate::models::error::AppError;
use crate::repositories::cart_repository::CartRepository;
use rust_decimal::Decimal;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone)]
pub struct CartService {
    repo: CartRepository,
}

impl CartService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let repo = CartRepository::new(pool);
        Self { repo }
    }

    async fn get_cart_response(&self, user_id: Uuid) -> Result<CartResponse, AppError> {
        let cart_id = self
            .repo
            .get_or_create_cart_id(user_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let items = self
            .repo
            .find_cart_items(cart_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut total_price = Decimal::ZERO;
        let mut total_items = 0;

        let item_responses: Vec<CartItemResponse> = items
            .into_iter()
            .map(|item| {
                let quantity_dec = Decimal::from(item.quantity);
                let subtotal = item.price * quantity_dec;

                total_price += subtotal;
                total_items += item.quantity;

                CartItemResponse {
                    item_id: item.item_id,
                    product_id: item.product_id,
                    product_name: item.product_name,
                    price: item.price,
                    quantity: item.quantity,
                    subtotal,
                }
            })
            .collect();

        Ok(CartResponse {
            id: cart_id,
            user_id,
            items: item_responses,
            total_price,
            total_items,
        })
    }

    pub async fn get_cart(&self, user_id: Uuid) -> Result<CartResponse, AppError> {
        self.get_cart_response(user_id).await
    }

    pub async fn add_to_cart(
        &self,
        user_id: Uuid,
        req: AddToCartRequest,
    ) -> Result<CartResponse, AppError> {
        let cart_id = self
            .repo
            .get_or_create_cart_id(user_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.repo
            .upsert_item(cart_id, req.product_id, req.quantity)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        self.get_cart_response(user_id).await
    }

    pub async fn update_item(
        &self,
        user_id: Uuid,
        item_id: Uuid,
        req: UpdateCartItemRequest,
    ) -> Result<CartResponse, AppError> {
        let updated = self
            .repo
            .update_item_quantity(item_id, req.quantity)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !updated {
            return Err(AppError::NotFound("Cart item not found".into()));
        }

        self.get_cart_response(user_id).await
    }

    pub async fn remove_item(
        &self, 
        user_id: Uuid, 
        item_id: Uuid
    ) -> Result<CartResponse, AppError> {
        let deleted = self
            .repo
            .delete_item(item_id)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        if !deleted {
            return Err(AppError::NotFound("Cart item not found".into()));
        }

        self.get_cart_response(user_id).await
    }
}