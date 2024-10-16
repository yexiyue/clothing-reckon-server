use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    typed_header::TypedHeader,
};
use jsonwebtoken::{errors::ErrorKind, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::{error::AuthError, state::AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: i32,
    pub exp: i64,
}

impl Claims {
    pub fn new(user_id: i32) -> Self {
        let exp = (chrono::Utc::now() + chrono::Duration::days(15)).timestamp();
        Self { user_id, exp }
    }

    // 生成token
    pub fn encode(&self, secret: &str) -> Result<String, AuthError> {
        let res = jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref()),
        )
        .map_err(|_| AuthError::TokenCreation)?;

        Ok(res)
    }

    // 解析token
    pub fn decode(token: &str, secret: &str) -> Result<Self, AuthError> {
        let res = jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| {
            if let ErrorKind::ExpiredSignature = e.kind() {
                AuthError::ExpiredSignature
            } else {
                AuthError::InvalidToken
            }
        })?;

        Ok(res.claims)
    }
}

// 从请求中获取claims, 并校验token
#[async_trait]
impl FromRequestParts<AppState> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;

        Self::decode(header.token(), &state.jwt_secret)
    }
}
