mod config;
mod constants;
mod controllers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use axum::{
    Router, middleware as axum_middleware,
    routing::{get, post},
};
use config::{AppState, init_db};
use controllers::auth_controller::{login_handler, register_handler};
use dotenvy::dotenv;
use middleware::auth::auth_middleware;
use routes::create_routes;
use std::net::SocketAddr;

use meilisearch_sdk::client::Client;

use crate::services::{ cart_service, search_service::{self, SearchService}, user_service::UserService};
use crate::services::categories_service::CategoriesService;
use crate::services::products_service::ProductsService;
use crate::services::cart_service::CartService;

#[tokio::main]
async fn main() {
    //Load Environment Variables
    dotenv().ok();

    // Init Database Connection Pool
    let pool = init_db().await;

    // เพิ่มส่วนการเชื่อมต่อ Meilisearch
    let meili_url = std::env::var("MEILISEARCH_URL").expect("MEILISEARCH_URL must be set");
    let meili_key = std::env::var("MEILISEARCH_API_KEY").expect("MEILISEARCH_API_KEY must be set");

    let meili_client = Client::new(meili_url, Some(meili_key))
        .expect("Failed to create Meilisearch client: Invalid URL");

    let user_service = UserService::new(pool.clone());
    let categories_service = CategoriesService::new(pool.clone());
    let product_service = ProductsService::new(pool.clone());
    let cart_service = CartService::new(pool.clone());
    let search_service = SearchService::new(meili_client.clone());

    let state = AppState {
        db: pool,
        meilisearch: meili_client,
        user_service: user_service,
        categories_service: categories_service,
        products_service: product_service,
        cart_service: cart_service,
        search_service:search_service,
    };

    let app = create_routes().with_state(state);

    //Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
