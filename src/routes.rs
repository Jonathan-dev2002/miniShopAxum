use crate::controllers::{auth_controller, user_controller};
use crate::middleware::auth::auth_middleware;
use crate::{config::AppState, controllers::categories_controller};
use axum::{
    Router, middleware as axum_middleware,
    routing::{delete, get, post, put, patch},
};

pub fn create_routes() -> Router<AppState> {
    Router::new()
        .nest("/auth", auth_routes())
        .nest("/users", user_routes())
        .nest("/categories", categories_routes())
        .route("/healthz", axum::routing::get(health_check))
}

fn auth_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/register",
            axum::routing::post(auth_controller::register_handler),
        )
        .route(
            "/login",
            axum::routing::post(auth_controller::login_handler),
        )
}

fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(user_controller::get_me_handler))
        .route("/me", put(user_controller::update_me_handler))
        .route("/me", delete(user_controller::delete_me_handler))
        .route("/all", get(user_controller::list_users_handler))
        .route("/", get(user_controller::get_users_handler))
        .layer(axum_middleware::from_fn(auth_middleware))
}

fn categories_routes() -> Router<AppState> {
    Router::new()
        .route("/:id", get(categories_controller::get_category_handler))
        .route("/", get(categories_controller::get_categories_handler))
        .route("/", post(categories_controller::create_categories_handler))
        .route("/:id",delete(categories_controller::delete_category_handler))
        .route("/:id", patch(categories_controller::update_categories_handler))
        .layer(axum_middleware::from_fn(auth_middleware))
}

async fn health_check() -> &'static str {
    "Service is running healthy!"
}
