use axum::Json;
use chrono::Utc;
use jsonwebtoken::{encode, Header};
use serde::Serialize;

use crate::{claims::Claims, error::AuthError, KEYS};

#[derive(Debug, Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
struct AuthPayload {
    client_id: String,
    client_secret: String,
}

async fn authorize(Json(payload): Json<AuthPayload>) -> Result<Json<AuthBody>, AuthError> {
    // Check if the user sent the credentials
    if payload.client_id.is_empty() || payload.client_secret.is_empty() {
        return Err(AuthError::MissingCredentials);
    }

    // Normally we would use a DB here
    if &payload.client_id != "foo" || &payload.client_secret != "bar" {
        return Err(AuthError::WrongCredentials);
    }

    // Create the expiry time.
    let exp = (Utc::now().naive_utc() + chrono::naive::Days::new(1)).timestamp() as usize;
    let claims = Claims {
        username: payload.client_id,
        exp,
    };

    // Create authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    Ok(Json(AuthBody::new(token)))
}

async fn protected(claims: Claims) -> String {
    format!("Welcome to the protected area, {}!", claims.username)
}
