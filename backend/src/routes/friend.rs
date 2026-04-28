use actix_web::{web, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter, QuerySelect, ModelTrait, Condition};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::{Arc, Mutex};
use crate::models::session;
use crate::models::message;
use crate::websocket::ConnectionManager;
use crate::middleware::auth::AuthenticatedUser;

#[derive(Deserialize)]
pub struct AddFriendRequest {
    pub friend_id: String,
}

#[derive(Deserialize)]
pub struct AddFriendByEmailRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct RemoveFriendRequest {
    pub friend_id: String,
}

#[derive(Serialize)]
pub struct FriendResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub status: String,
}

pub async fn add_friend(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    req: web::Json<AddFriendRequest>,
    manager: web::Data<Arc<Mutex<ConnectionManager>>>,
) -> HttpResponse {
    let friend_id = req.friend_id.clone();
    
    // Check if trying to add self
    if friend_id == user.user_id {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Cannot add yourself as friend"
        }));
    }
    
    // Check if session already exists
    let existing_session = session::Entity::find()
        .filter(session::Column::User1Id.eq(&user.user_id))
        .filter(session::Column::User2Id.eq(&friend_id))
        .one(&**db)
        .await;
    
    if let Ok(Some(_)) = existing_session {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Already friends"
        }));
    }
    
    // Check reverse
    let existing_session_reverse = session::Entity::find()
        .filter(session::Column::User1Id.eq(&friend_id))
        .filter(session::Column::User2Id.eq(&user.user_id))
        .one(&**db)
        .await;
    
    if let Ok(Some(_)) = existing_session_reverse {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Already friends"
        }));
    }
    
    // Create session
    let session_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();
    
    let new_session = session::ActiveModel {
        id: Set(session_id),
        user1_id: Set(user.user_id.clone()),
        user2_id: Set(friend_id.clone()),
        last_message_at: Set(now),
        created_at: Set(now),
    };

    match new_session.insert(&**db).await {
        Ok(_) => {
            let notify_json = serde_json::json!({
                "type": "friend_added",
                "friend_id": user.user_id.clone(),
            }).to_string();

            let mut mgr = manager.lock().unwrap();
            mgr.send_to_user(&friend_id, notify_json);

            HttpResponse::Ok().json(serde_json::json!({
                "message": "Friend added successfully"
            }))
        }
        Err(e) => {
            log::error!("Failed to add friend: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to add friend"
            }))
        }
    }
}

pub async fn add_friend_by_email(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    req: web::Json<AddFriendByEmailRequest>,
    manager: web::Data<Arc<Mutex<ConnectionManager>>>,
) -> HttpResponse {
    use crate::models::user;

    let target_email = req.email.clone();

    let target_user = user::Entity::find()
        .filter(user::Column::Email.eq(&target_email))
        .one(&**db)
        .await;

    match target_user {
        Ok(Some(friend)) => {
            let friend_id = friend.id;

            if friend_id == user.user_id {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Cannot add yourself as friend"
                }));
            }

            let existing_session = session::Entity::find()
                .filter(session::Column::User1Id.eq(&user.user_id))
                .filter(session::Column::User2Id.eq(&friend_id))
                .one(&**db)
                .await;

            if let Ok(Some(_)) = existing_session {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Already friends"
                }));
            }

            let existing_session_reverse = session::Entity::find()
                .filter(session::Column::User1Id.eq(&friend_id))
                .filter(session::Column::User2Id.eq(&user.user_id))
                .one(&**db)
                .await;

            if let Ok(Some(_)) = existing_session_reverse {
                return HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Already friends"
                }));
            }

            let session_id = Uuid::new_v4().to_string();
            let now = chrono::Utc::now().naive_utc();

            let new_session = session::ActiveModel {
                id: Set(session_id),
                user1_id: Set(user.user_id.clone()),
                user2_id: Set(friend_id.clone()),
                last_message_at: Set(now),
                created_at: Set(now),
            };

            match new_session.insert(&**db).await {
                Ok(_) => {
                    let notify_json = serde_json::json!({
                        "type": "friend_added",
                        "friend_id": user.user_id.clone(),
                    }).to_string();

                    let mut mgr = manager.lock().unwrap();
                    mgr.send_to_user(&friend_id, notify_json);

                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "Friend added successfully"
                    }))
                }
                Err(e) => {
                    log::error!("Failed to add friend by email: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to add friend"
                    }))
                }
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found with this email"
            }))
        }
        Err(e) => {
            log::error!("Failed to find user by email: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn get_friends(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> HttpResponse {
    use crate::models::user;
    
    // Get all sessions where user is either user1 or user2
    let sessions = session::Entity::find()
        .filter(
            session::Column::User1Id.eq(&user.user_id)
                .or(session::Column::User2Id.eq(&user.user_id))
        )
        .all(&**db)
        .await;
    
    match sessions {
        Ok(sessions) => {
            let mut friend_ids = Vec::new();
            for session in sessions {
                if session.user1_id == user.user_id {
                    friend_ids.push(session.user2_id);
                } else {
                    friend_ids.push(session.user1_id);
                }
            }
            
            // Get user details for each friend
            let mut friends = Vec::new();
            for friend_id in friend_ids {
                if let Ok(Some(friend_user)) = user::Entity::find_by_id(friend_id).one(&**db).await {
                    friends.push(FriendResponse {
                        id: friend_user.id,
                        username: friend_user.username,
                        email: friend_user.email,
                        avatar_url: friend_user.avatar_url,
                        status: friend_user.status,
                    });
                }
            }
            
            HttpResponse::Ok().json(friends)
        }
        Err(e) => {
            log::error!("Failed to get friends: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to get friends"
            }))
        }
    }
}

pub async fn remove_friend(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    req: web::Json<RemoveFriendRequest>,
    manager: web::Data<Arc<Mutex<ConnectionManager>>>,
) -> HttpResponse {
    let friend_id = req.friend_id.clone();
    
    // Check if trying to remove self
    if friend_id == user.user_id {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Cannot remove yourself as friend"
        }));
    }
    
    // Find and delete the session
    let session = session::Entity::find()
        .filter(
            session::Column::User1Id.eq(&user.user_id)
                .and(session::Column::User2Id.eq(&friend_id))
                .or(
                    session::Column::User1Id.eq(&friend_id)
                        .and(session::Column::User2Id.eq(&user.user_id))
                )
        )
        .one(&**db)
        .await;
    
    match session {
        Ok(Some(session)) => {
            // 删除双方之间的聊天消息
            match message::Entity::delete_many()
                .filter(
                    Condition::any()
                        .add(
                            Condition::all()
                                .add(message::Column::SenderId.eq(&user.user_id))
                                .add(message::Column::ReceiverId.eq(&friend_id))
                        )
                        .add(
                            Condition::all()
                                .add(message::Column::SenderId.eq(&friend_id))
                                .add(message::Column::ReceiverId.eq(&user.user_id))
                        )
                )
                .exec(&**db)
                .await
            {
                Ok(r) => {
                    log::info!("Deleted {} messages between users", r.rows_affected);
                }
                Err(e) => {
                    log::error!("Failed to delete messages: {:?}", e);
                }
            }

            match session.delete(&**db).await {
                Ok(_) => {
                    let notify_json = serde_json::json!({
                        "type": "friend_removed",
                        "friend_id": user.user_id.clone(),
                    }).to_string();

                    let mut mgr = manager.lock().unwrap();
                    mgr.send_to_user(&friend_id, notify_json);

                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "Friend removed successfully"
                    }))
                }
                Err(e) => {
                    log::error!("Failed to remove friend: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Failed to remove friend"
                    }))
                }
            }
        }
        Ok(None) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Not friends with this user"
            }))
        }
        Err(e) => {
            log::error!("Failed to find session: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}
