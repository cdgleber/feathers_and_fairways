use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use base64::{Engine as _, engine::general_purpose};

pub async fn admin_auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let admin_password = std::env::var("ADMIN_PASSWORD")
        .unwrap_or_else(|_| "admin123".to_string());
    tracing::info!("ADMIN PASSWORD IS {admin_password}");

    // Check for Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    tracing::info!("Header is {auth_header:?}");

    if let Some(auth) = auth_header {
        // Support both "Bearer token" and "Basic base64" formats
        if auth.starts_with("Bearer ") {
            let token = &auth[7..];
            if token == admin_password {
                return Ok(next.run(request).await);
            }
        } else if auth.starts_with("Basic ") {
            let encoded = &auth[6..];
            if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
                if let Ok(credentials) = String::from_utf8(decoded) {
                    // Format is "username:password", we only care about password
                    if let Some((_, password)) = credentials.split_once(':') {
                        tracing::info!("AUTH ATTEMPT: {password} against {admin_password}");
                        if password == admin_password {
                            return Ok(next.run(request).await);
                        }
                    }
                }
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

pub fn verify_admin_password(password: &str) -> bool {
    let admin_password = std::env::var("ADMIN_PASSWORD")
        .unwrap_or_else(|_| "admin123".to_string());
    password == admin_password
}

pub fn generate_admin_token() -> String {
    let admin_password = std::env::var("ADMIN_PASSWORD")
        .unwrap_or_else(|_| "admin123".to_string());
    admin_password
}