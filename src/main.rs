mod config;
mod controllers;
mod models;
mod repositories;
mod services;
mod utils;
mod middleware;
mod constants; 
// mod error;  <-- ไม่ต้องประกาศที่นี่ ถ้าเอาไฟล์ error.rs ไปไว้ใน folder 'models' ตามโครงสร้างก่อนหน้า
// แต่ถ้าคุณวางไฟล์ error.rs ไว้ที่ src/error.rs โดยตรง ให้ uncomment บรรทัดนี้ครับ

use axum::{
    middleware as axum_middleware, // ตั้งชื่อเล่นเพื่อไม่ให้ซ้ำกับ module middleware ของเรา
    routing::{get, post},
    Router,
};
use config::{init_db, AppState};
use controllers::auth_controller::{login_handler, register_handler};
use middleware::auth::auth_middleware; // Import middleware ที่เราสร้าง
use dotenvy::dotenv;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // 1. Load Environment Variables
    dotenv().ok();

    // 2. Init Database Connection Pool
    let pool = init_db().await;
    let state = AppState { db: pool };

    // 3. Define Public Routes (Login, Register)
    // เส้นทางพวกนี้ ใครก็เข้าได้ ไม่ต้องเช็ค Token
    let auth_routes = Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler));

    // 4. Define Protected Routes (User Profile, Dashboard, etc.)
    // เส้นทางพวกนี้จะถูกดักจับด้วย auth_middleware ก่อน
    let protected_routes = Router::new()
        .route("/users/me", get(|| async { "Hello! You are authorized." })) // ตัวอย่าง Handler
        // .route("/users/update", post(update_user_handler)) // ตัวอย่าง
        .route_layer(axum_middleware::from_fn(auth_middleware)); // <--- หัวใจสำคัญ: บังคับใช้ Middleware ตรงนี้

    // 5. Merge Routes & Inject State
    // เอา Public + Protected มารวมกัน แล้วส่ง State (Database) ให้ทุกเส้นทาง
    let app = Router::new()
        .merge(auth_routes)
        .merge(protected_routes)
        .with_state(state);

    // 6. Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("🚀 Server listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}