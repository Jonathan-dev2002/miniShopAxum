use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use crate::utils::jwt::decode_jwt;

pub async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. ดึง Header Authorization
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());

    let auth_header = if let Some(auth_header) = auth_header {
        auth_header
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    // 2. เช็คว่าเป็น Bearer token ไหม
    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..];

    // 3. Validate Token
    let claims = match decode_jwt(token) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // 4. (Optional) ใส่ user_id ลงใน Request context เพื่อให้ Controller ใช้ต่อได้
    req.extensions_mut().insert(claims);

    // 5. ปล่อยผ่านไป Controller
    Ok(next.run(req).await)
}