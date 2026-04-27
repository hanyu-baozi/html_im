use actix_web::{web, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set, ActiveModelTrait};
use serde::{Serialize, Deserialize};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn get_me(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> HttpResponse {
    match user::Entity::find_by_id(user.user_id.clone())
        .one(&**db)
        .await
    {
        Ok(Some(user_model)) => {
            HttpResponse::Ok().json(UserResponse {
                id: user_model.id,
                username: user_model.username,
                email: user_model.email,
                avatar_url: user_model.avatar_url,
                status: user_model.status,
            })
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to get user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn get_users(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> HttpResponse {
    match user::Entity::find()
        .filter(user::Column::Id.ne(user.user_id.clone()))
        .all(&**db)
        .await
    {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users
                .into_iter()
                .map(|u| UserResponse {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                    avatar_url: u.avatar_url,
                    status: u.status,
                })
                .collect();
            HttpResponse::Ok().json(user_responses)
        }
        Err(e) => {
            log::error!("Failed to get users: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn get_user(
    db: web::Data<DatabaseConnection>,
    path: web::Path<String>,
) -> HttpResponse {
    let user_id = path.into_inner();
    
    match user::Entity::find_by_id(user_id.clone())
        .one(&**db)
        .await
    {
        Ok(Some(user_model)) => {
            HttpResponse::Ok().json(UserResponse {
                id: user_model.id,
                username: user_model.username,
                email: user_model.email,
                avatar_url: user_model.avatar_url,
                status: user_model.status,
            })
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to get user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn search_users(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let username = match query.get("username") {
        Some(u) => u,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "username is required"
            }));
        }
    };

    match user::Entity::find()
        .filter(user::Column::Username.contains(username))
        .filter(user::Column::Id.ne(user.user_id.clone()))
        .all(&**db)
        .await
    {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users
                .into_iter()
                .map(|u| UserResponse {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                    avatar_url: u.avatar_url,
                    status: u.status,
                })
                .collect();
            HttpResponse::Ok().json(user_responses)
        }
        Err(e) => {
            log::error!("Failed to search users: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn search_users_by_email(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let email = match query.get("email") {
        Some(e) => e,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "email is required"
            }));
        }
    };

    match user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .filter(user::Column::Id.ne(user.user_id.clone()))
        .all(&**db)
        .await
    {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users
                .into_iter()
                .map(|u| UserResponse {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                    avatar_url: u.avatar_url,
                    status: u.status,
                })
                .collect();
            HttpResponse::Ok().json(user_responses)
        }
        Err(e) => {
            log::error!("Failed to search users by email: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn update_status(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    req: web::Json<UpdateStatusRequest>,
) -> HttpResponse {
    match user::Entity::find_by_id(user.user_id.clone())
        .one(&**db)
        .await
    {
        Ok(Some(user_model)) => {
            let mut active_user: user::ActiveModel = user_model.into();
            active_user.status = Set(req.status.clone());
            
            match active_user.update(&**db).await {
                Ok(_) => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "Status updated",
                        "status": req.status
                    }))
                }
                Err(e) => {
                    log::error!("Failed to update status: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to update status"
                    }))
                }
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to get user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}
