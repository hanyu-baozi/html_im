use actix_web::{dev::ServiceRequest, Error, FromRequest, HttpMessage};
use actix_web::error::ErrorUnauthorized;
use jsonwebtoken::{decode, Validation, DecodingKey};
use serde::Deserialize;
use std::future::{ready, Ready};

#[derive(Debug, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub is_admin: bool,
}

pub struct AuthenticatedUser {
    pub user_id: String,
    pub is_admin: bool,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        let token = match auth_header {
            Some(header) => {
                let header_str = match header.to_str() {
                    Ok(s) => s,
                    Err(_) => {
                        return ready(Err(ErrorUnauthorized("Invalid authorization header")))
                    }
                };
                
                if header_str.starts_with("Bearer ") {
                    Some(header_str[7..].to_string())
                } else {
                    None
                }
            }
            None => None,
        };

        if let Some(token) = token {
            let secret = std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
            
            match decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()) {
                Ok(data) => {
                    ready(Ok(AuthenticatedUser {
                        user_id: data.claims.sub,
                        is_admin: data.claims.is_admin,
                    }))
                }
                Err(_) => {
                    ready(Err(ErrorUnauthorized("Invalid token")))
                }
            }
        } else {
            ready(Err(ErrorUnauthorized("Missing authorization token")))
        }
    }
}
