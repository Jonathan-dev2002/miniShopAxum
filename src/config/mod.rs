use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use meilisearch_sdk::client::Client;

use crate::services::categories_service::CategoriesService;
use crate::services::products_service::ProductsService;
use crate::services::search_service::SearchService;
use crate::services::user_service::UserService;
use crate::services::cart_service::CartService;
#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>, // นี่คือ Connection Pool
    pub meilisearch: Client,
    pub user_service: UserService,
    pub categories_service: CategoriesService,
    pub products_service: ProductsService,
    pub cart_service: CartService,
    pub search_service: SearchService
}

pub async fn init_db() -> Pool<Postgres> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
