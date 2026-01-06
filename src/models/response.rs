use serde::Serialize;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

// 1. ส่วนของ Status (Code & Description)
#[derive(Serialize)]
pub struct Status {
    pub code: String,
    pub description: String,
}

// 2. ส่วนของ Response Wrapper (Generic T)
// T คือ Data ข้างใน จะเป็น User, Token หรืออะไรก็ได้
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: Status,
    #[serde(skip_serializing_if = "Option::is_none")] // ถ้าไม่มี data ไม่ต้องส่ง field นี้ไป
    pub data: Option<T>,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    // Constructor สำหรับ Success Case
    pub fn success(data: T, code: &str, description: &str) -> Self {
        Self {
            status: Status {
                code: code.to_string(),
                description: description.to_string(),
            },
            data: Some(data),
        }
    }
    
    // Constructor สำหรับ Success แต่ไม่มี Data (เช่น Register สำเร็จ)
    pub fn success_no_data(code: &str, description: &str) -> Self {
        Self {
            status: Status {
                code: code.to_string(),
                description: description.to_string(),
            },
            data: None,
        }
    }
}

// Implement IntoResponse เพื่อให้ Controller return struct นี้ออกไปได้เลย
impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        // ปกติ Success จะเป็น 200 OK เสมอ แล้วไปดู code ข้างในเอา
        (StatusCode::OK, Json(self)).into_response()
    }
}