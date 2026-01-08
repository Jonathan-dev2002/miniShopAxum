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

use crate::services::user_service::UserService;
use crate::services::categories_service::CategoriesService;

#[tokio::main]
async fn main() {
    //Load Environment Variables
    dotenv().ok();

    // Init Database Connection Pool
    let pool = init_db().await;
    let user_service = UserService::new(pool.clone());
    let categories_service = CategoriesService::new(pool.clone());
    let state = AppState {
        db: pool,
        user_service: user_service,
        categories_service: categories_service,
    };

    let app = create_routes().with_state(state);

    //Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
