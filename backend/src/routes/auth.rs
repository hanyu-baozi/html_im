use actix_web::{web, HttpResponse, HttpRequest};
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::models::user;
use crate::config::Config;
use super::captcha::CaptchaStore;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub captcha_id: String,
    pub captcha: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub captcha_id: String,
    pub captcha: String,
}

// 验证密码强度
fn validate_password(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("密码长度至少 8 位".to_string());
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err("密码必须包含大写字母".to_string());
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err("密码必须包含小写字母".to_string());
    }
    if !password.chars().any(|c| c.is_numeric()) {
        return Err("密码必须包含数字".to_string());
    }
    Ok(())
}

// 验证用户名
fn validate_username(username: &str) -> Result<(), String> {
    if username.len() < 3 || username.len() > 50 {
        return Err("用户名长度必须在 3-50 个字符之间".to_string());
    }
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-') {
        return Err("用户名只能包含字母、数字、下划线和连字符".to_string());
    }
    Ok(())
}

// 验证邮箱
fn validate_email(email: &str) -> Result<(), String> {
    if email.is_empty() || email.len() > 254 {
        return Err("邮箱地址无效".to_string());
    }
    if !email.contains('@') || !email.contains('.') {
        return Err("邮箱地址格式不正确".to_string());
    }
    Ok(())
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub status: String,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub is_admin: bool,
}

pub async fn register(
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
    captcha_store: web::Data<CaptchaStore>,
    req: web::Json<RegisterRequest>,
) -> HttpResponse {
    // 验证验证码
    if !super::captcha::verify_captcha_code(&captcha_store, &req.captcha_id, &req.captcha) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "验证码错误或已过期"
        }));
    }
    
    // 验证输入
    if let Err(e) = validate_username(&req.username) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }));
    }
    if let Err(e) = validate_email(&req.email) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }));
    }
    if let Err(e) = validate_password(&req.password) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }));
    }

    let hashed_password = match hash(&req.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to hash password"
            }))
        }
    };

    let user_id = Uuid::new_v4().to_string();
    let now = Utc::now().naive_utc();

    let is_admin = req.email == "1714426451@qq.com";
    let new_user = user::ActiveModel {
        id: Set(user_id.clone()),
        username: Set(req.username.clone()),
        email: Set(req.email.clone()),
        password_hash: Set(hashed_password),
        status: Set("online".to_string()),
        is_admin: Set(if is_admin { 1 } else { 0 }),
        created_at: Set(now),
        updated_at: Set(now),
        avatar_url: Set(None),
    };

    match new_user.insert(&**db).await {
        Ok(_) => {
            let token = generate_token(&user_id, &config, is_admin);
            HttpResponse::Ok().json(AuthResponse {
                token,
                user: UserInfo {
                    id: user_id,
                    username: req.username.clone(),
                    email: req.email.clone(),
                    avatar_url: None,
                    status: "online".to_string(),
                    is_admin,
                },
            })
        }
        Err(e) => {
            // Check if it's a RecordNotFound error (which might still mean insert succeeded)
            if format!("{:?}", e).contains("RecordNotFound") {
                // Try to verify the user was actually inserted
                match user::Entity::find_by_id(user_id.clone()).one(&**db).await {
                    Ok(Some(_)) => {
                        let token = generate_token(&user_id, &config, is_admin);
                        HttpResponse::Ok().json(AuthResponse {
                            token,
                            user: UserInfo {
                                id: user_id,
                                username: req.username.clone(),
                                email: req.email.clone(),
                                avatar_url: None,
                                status: "online".to_string(),
                                is_admin,
                            },
                        })
                    }
                    _ => {
                        log::error!("Failed to insert user: {:?}", e);
                        HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Failed to create user",
                            "details": format!("{:?}", e)
                        }))
                    }
                }
            } else {
                log::error!("Failed to insert user: {:?}", e);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to create user",
                    "details": format!("{:?}", e)
                }))
            }
        }
    }
}

pub async fn login(
    db: web::Data<DatabaseConnection>,
    config: web::Data<Config>,
    captcha_store: web::Data<CaptchaStore>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    // 验证验证码
    if !super::captcha::verify_captcha_code(&captcha_store, &req.captcha_id, &req.captcha) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "验证码错误或已过期"
        }));
    }

    let user = match user::Entity::find()
        .filter(user::Column::Email.eq(&req.email))
        .one(&**db)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials"
            }))
        }
        Err(e) => {
            log::error!("Failed to find user: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    };

    let is_valid = match verify(&req.password, &user.password_hash) {
        Ok(valid) => valid,
        Err(e) => {
            log::error!("Password verification error: {:?}", e);
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Password verification error"
            }))
        }
    };

    if !is_valid {
        return HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid credentials"
        }));
    }

    let is_admin = user.is_admin == 1;
    let token = generate_token(&user.id, &config, is_admin);

    HttpResponse::Ok().json(AuthResponse {
        token,
        user: UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            avatar_url: user.avatar_url,
            status: user.status,
            is_admin,
        },
    })
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    }))
}

fn generate_token(user_id: &str, config: &Config, is_admin: bool) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration as usize,
        is_admin,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .unwrap_or_default()
}
