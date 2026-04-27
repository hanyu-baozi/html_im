use actix_web::{web, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, ActiveModelTrait, ColumnTrait, QueryFilter, Condition, Set, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::message;
use crate::models::group_member;
use crate::models::group;
use crate::websocket::ConnectionManager;

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub receiver_id: String,
    pub content: String,
    pub message_type: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub content: String,
    pub message_type: String,
    pub timestamp: i64,
    pub is_read: bool,
}

pub async fn get_messages(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> HttpResponse {
    let limit = query.get("limit")
        .and_then(|l| l.parse::<u64>().ok())
        .unwrap_or(50);
    let page = query.get("page")
        .and_then(|p| p.parse::<u64>().ok())
        .unwrap_or(1);
    let offset = (page - 1) * limit;

    let messages = if let Some(group_id) = query.get("group_id") {
        // Group chat messages
        message::Entity::find()
            .filter(message::Column::ReceiverId.eq(group_id))
            .order_by_desc(message::Column::Timestamp)
            .offset(offset)
            .limit(limit)
            .all(&**db)
            .await
    } else if let Some(contact_id) = query.get("contact_id") {
        // Private chat messages
        message::Entity::find()
            .filter(
                Condition::any()
                    .add(
                        Condition::all()
                            .add(message::Column::SenderId.eq(&user.user_id))
                            .add(message::Column::ReceiverId.eq(contact_id))
                    )
                    .add(
                        Condition::all()
                            .add(message::Column::SenderId.eq(contact_id))
                            .add(message::Column::ReceiverId.eq(&user.user_id))
                    )
            )
            .order_by_desc(message::Column::Timestamp)
            .offset(offset)
            .limit(limit)
            .all(&**db)
            .await
    } else {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "contact_id or group_id is required"
        }));
    };

    match messages {
        Ok(msgs) => {
            let mut responses: Vec<MessageResponse> = msgs
                .into_iter()
                .map(|m| MessageResponse {
                    id: m.id,
                    sender_id: m.sender_id,
                    receiver_id: m.receiver_id,
                    content: m.content,
                    message_type: m.message_type,
                    timestamp: m.timestamp.and_utc().timestamp_millis(),
                    is_read: m.is_read,
                })
                .collect();
            
            // Reverse to return oldest first within the page
            responses.reverse();
            
            HttpResponse::Ok().json(responses)
        }
        Err(e) => {
            log::error!("Failed to get messages: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn send_message(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    req: web::Json<SendMessageRequest>,
    manager: web::Data<Arc<Mutex<ConnectionManager>>>,
) -> HttpResponse {
    let message_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now().naive_utc();

    let new_message = message::ActiveModel {
        id: Set(message_id.clone()),
        sender_id: Set(user.user_id.clone()),
        receiver_id: Set(req.receiver_id.clone()),
        content: Set(req.content.clone()),
        message_type: Set(req.message_type.clone()),
        timestamp: Set(timestamp),
        is_read: Set(false),
        read_at: Set(None),
    };

    let receiver_id = req.receiver_id.clone();
    let sender_id = user.user_id.clone();
    let content = req.content.clone();
    let ts = timestamp.and_utc().timestamp_millis();

    let message_json = serde_json::json!({"type": "new_message", "data": {"id": message_id, "sender_id": sender_id.clone(), "receiver_id": receiver_id.clone(), "content": content.clone(), "message_type": "text", "timestamp": ts, "is_read": false}}).to_string();

    match new_message.insert(&**db).await {
        Ok(_) => {
            let mut mgr = manager.lock().unwrap();
            
            // If it's a group message (receiver_id looks like a group UUID, not a user ID)
            // Notify all group members including the sender
            if is_group_id(&receiver_id, &db).await {
                match group_member::Entity::find()
                    .filter(group_member::Column::GroupId.eq(&receiver_id))
                    .all(&**db)
                    .await
                {
                    Ok(members) => {
                        for member in members {
                            mgr.send_to_user(&member.user_id, message_json.clone());
                        }
                    }
                    Err(e) => {
                        log::warn!("Failed to get group members: {:?}", e);
                    }
                }
            } else {
                // Private message - notify sender and receiver
                mgr.send_to_user(&receiver_id, message_json.clone());
                mgr.send_to_user(&sender_id, message_json.clone());
            }

            HttpResponse::Ok().json(MessageResponse {
                id: message_id,
                sender_id: user.user_id,
                receiver_id: req.receiver_id.clone(),
                content: req.content.clone(),
                message_type: req.message_type.clone(),
                timestamp: ts,
                is_read: false,
            })
        }
        Err(e) => {
            let err_str = format!("{:?}", e);
            if err_str.contains("RecordNotFound") {
                match message::Entity::find_by_id(message_id.clone()).one(&**db).await {
                    Ok(Some(_)) => {
                        let mut mgr = manager.lock().unwrap();
                        
                        if is_group_id(&receiver_id, &db).await {
                            match group_member::Entity::find()
                                .filter(group_member::Column::GroupId.eq(&receiver_id))
                                .all(&**db)
                                .await
                            {
                                Ok(members) => {
                                    for member in members {
                                        mgr.send_to_user(&member.user_id, message_json.clone());
                                    }
                                }
                                Err(_) => {}
                            }
                        } else {
                            mgr.send_to_user(&receiver_id, message_json.clone());
                            mgr.send_to_user(&sender_id, message_json.clone());
                        }

                        return HttpResponse::Ok().json(MessageResponse {
                            id: message_id,
                            sender_id: user.user_id,
                            receiver_id: req.receiver_id.clone(),
                            content: req.content.clone(),
                            message_type: req.message_type.clone(),
                            timestamp: ts,
                            is_read: false,
                        });
                    }
                    _ => {
                        log::error!("Message insert failed and not found: {:?}", e);
                        return HttpResponse::InternalServerError().json(serde_json::json!({
                            "error": "Failed to send message"
                        }));
                    }
                }
            }
            log::error!("Failed to send message: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to send message"
            }))
        }
    }
}

async fn is_group_id(id: &str, db: &DatabaseConnection) -> bool {
    group::Entity::find_by_id(id)
        .one(db)
        .await
        .map(|r| r.is_some())
        .unwrap_or(false)
}

pub async fn mark_as_read(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    let message_id = path.into_inner();

    match message::Entity::find_by_id(message_id.clone())
        .one(&**db)
        .await
    {
        Ok(Some(msg)) if msg.receiver_id == user.user_id => {
            let now = Utc::now().naive_utc();
            let mut active_msg: message::ActiveModel = msg.into();
            active_msg.is_read = Set(true);
            active_msg.read_at = Set(Some(now));

            match active_msg.update(&**db).await {
                Ok(_) => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "Message marked as read",
                        "message_id": message_id
                    }))
                }
                Err(e) => {
                    log::error!("Failed to mark message as read: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({"error": "Failed to mark message as read"}))
                }
            }
        }
        Ok(Some(_)) => {
            HttpResponse::Forbidden().json(serde_json::json!({
                "error": "Not authorized to mark this message as read"
            }))
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "Message not found"
            }))
        }
        Err(e) => {
            log::error!("Failed to find message: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({"error": "Database error"}))
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteChatRequest {
    pub contact_id: String,
}

pub async fn delete_chat_messages(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    req: web::Json<DeleteChatRequest>,
) -> HttpResponse {
    let contact_id = req.contact_id.clone();

    if contact_id == user.user_id {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "Cannot delete chat with yourself"
        }));
    }

    let result = message::Entity::delete_many()
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(message::Column::SenderId.eq(&user.user_id))
                        .add(message::Column::ReceiverId.eq(&contact_id))
                )
                .add(
                    Condition::all()
                        .add(message::Column::SenderId.eq(&contact_id))
                        .add(message::Column::ReceiverId.eq(&user.user_id))
                )
        )
        .exec(&**db)
        .await;

    match result {
        Ok(result) => {
            HttpResponse::Ok().json(serde_json::json!({
                "message": "Chat messages deleted successfully",
                "deleted_count": result.rows_affected
            }))
        }
        Err(e) => {
            log::error!("Failed to delete chat messages: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to delete chat messages"
            }))
        }
    }
}
