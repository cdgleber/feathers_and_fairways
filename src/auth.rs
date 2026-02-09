use axum::{
    extract::Request,
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};
use base64::{Engine as _, engine::general_purpose};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn get_admin_password() -> Result<String, ()> {
    std::env::var("ADMIN_PASSWORD").map_err(|_| {
        tracing::error!("ADMIN_PASSWORD environment variable is not set");
    })
}

pub async fn admin_auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let admin_password = get_admin_password().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(auth) = auth_header {
        if auth.starts_with("Bearer ") {
            let token = &auth[7..];
            let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
            let key = DecodingKey::from_secret(admin_password.as_bytes());
            match decode::<Claims>(token, &key, &validation) {
                Ok(_) => return Ok(next.run(request).await),
                Err(e) => {
                    tracing::debug!("JWT validation failed: {e}");
                }
            }
        } else if auth.starts_with("Basic ") {
            let encoded = &auth[6..];
            if let Ok(decoded) = general_purpose::STANDARD.decode(encoded) {
                if let Ok(credentials) = String::from_utf8(decoded) {
                    if let Some((_, password)) = credentials.split_once(':') {
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

pub fn verify_admin_password(password: &str) -> Result<bool, ()> {
    let admin_password = get_admin_password()?;
    Ok(password == admin_password)
}

pub fn generate_admin_token() -> Result<String, ()> {
    let admin_password = get_admin_password()?;
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: "admin".to_string(),
        exp: expiration,
    };

    Ok(encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(admin_password.as_bytes()),
    )
    .expect("JWT encoding should not fail"))
}
