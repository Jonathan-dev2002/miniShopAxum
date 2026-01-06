use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// 1. สร้าง Enum เพื่อรวม Error ทุกประเภทในระบบ
#[derive(Debug)]
pub enum AppError {
    AuthError(String),
    NotFound(String),
    DatabaseError(String),
    InternalServerError(String),
    ValidationError(String),
}

// 2. บอก Axum ว่า Error แต่ละตัวคือ HTTP Status Code อะไร
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, app_code, message) = match self {
            // กำหนด Code เองตรงนี้ได้เลย เช่น AuthError = "4001"
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, "4001", msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, "4004", msg),
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, "4000", msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "5001", format!("Database error: {}", msg)),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, "5000", msg),
        };

        // สร้าง JSON ให้ตรงกับ Format ที่ต้องการ
        let body = Json(json!({
            "status": {
                "code": app_code,
                "description": message
            },
            "data": null
        }));

        (status_code, body).into_response()
    }
}

// Helper เพื่อแปลง Error จาก Library อื่นให้เป็น AppError อัตโนมัติ (Optional แต่สะดวก)
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}