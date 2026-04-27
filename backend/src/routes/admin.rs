use actix_web::{web, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, QueryOrder, Set, ActiveModelTrait};
use serde::{Serialize, Deserialize};
use crate::middleware::auth::AuthenticatedUser;
use crate::models::user;
use crate::models::message;
use crate::models::group;
use crate::models::group_member;

#[derive(Deserialize)]
pub struct GetMessagesQuery {
    pub user_id: Option<String>,
    pub group_id: Option<String>,
    pub chat_type: Option<String>, // "private" or "group"
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct AdminMessageResponse {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub content: String,
    pub message_type: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub is_read: bool,
}

#[derive(Serialize)]
pub struct ChatConversation {
    pub id: String,
    pub chat_type: String, // "private" or "group"
    pub other_user_id: Option<String>,
    pub other_username: Option<String>,
    pub group_id: Option<String>,
    pub group_name: Option<String>,
    pub last_message: String,
    pub message_count: u64,
    pub last_message_time: chrono::DateTime<chrono::Utc>,
}

pub async fn admin_get_all_messages(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    query: web::Query<GetMessagesQuery>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(50).min(100);
    let skip = (page - 1) * page_size;

    let mut query_builder = message::Entity::find()
        .order_by_desc(message::Column::Timestamp);

    if let Some(ref user_id) = query.user_id {
        query_builder = query_builder
            .filter(
                message::Column::SenderId.eq(user_id)
                    .or(message::Column::ReceiverId.eq(user_id))
            );
    }

    if let Some(ref group_id) = query.group_id {
        query_builder = query_builder
            .filter(message::Column::ReceiverId.eq(group_id));
    }

    if let Some(ref chat_type) = query.chat_type {
        if chat_type == "private" {
            query_builder = query_builder
                .filter(message::Column::ReceiverId.not_like("group_%"));
        } else if chat_type == "group" {
            query_builder = query_builder
                .filter(message::Column::ReceiverId.like("group_%"));
        }
    }

    match query_builder
        .all(&**db)
        .await
    {
        Ok(messages) => {
            let total = messages.len();
            let paginated: Vec<AdminMessageResponse> = messages
                .into_iter()
                .skip(skip as usize)
                .take(page_size as usize)
                .map(|m| AdminMessageResponse {
                    id: m.id,
                    sender_id: m.sender_id,
                    receiver_id: m.receiver_id,
                    content: m.content,
                    message_type: m.message_type,
                    timestamp: m.timestamp.and_utc(),
                    is_read: m.is_read,
                })
                .collect();

            HttpResponse::Ok().json(serde_json::json!({
                "messages": paginated,
                "total": total,
                "page": page,
                "page_size": page_size
            }))
        }
        Err(e) => {
            log::error!("Failed to get messages: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

pub async fn admin_get_chat_conversations(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    query: web::Query<GetMessagesQuery>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let chat_type_filter = query.chat_type.clone();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(50).min(100);
    let skip = (page - 1) * page_size;

    match message::Entity::find()
        .order_by_desc(message::Column::Timestamp)
        .all(&**db)
        .await
    {
        Ok(messages) => {
            let mut private_chats: std::collections::HashMap<String, (u64, String, String)> = std::collections::HashMap::new();
            let mut group_chats: std::collections::HashMap<String, (u64, String, String)> = std::collections::HashMap::new();

            for m in messages {
                let receiver_is_group = m.receiver_id.starts_with("group_");
                
                if receiver_is_group {
                    let group_id = &m.receiver_id;
                    let entry = group_chats.entry(group_id.clone()).or_insert((0, m.content.clone(), format_timestamp(m.timestamp)));
                    entry.0 += 1;
                } else {
                    let sender_id = &m.sender_id;
                    let receiver_id = &m.receiver_id;
                    let chat_pair = if sender_id < receiver_id {
                        format!("{}|{}", sender_id, receiver_id)
                    } else {
                        format!("{}|{}", receiver_id, sender_id)
                    };
                    
                    let entry = private_chats.entry(chat_pair).or_insert((0, m.content.clone(), format_timestamp(m.timestamp)));
                    entry.0 += 1;
                }
            }

            let mut conversations: Vec<ChatConversation> = Vec::new();

            for (chat_pair, (count, last_msg, last_time)) in private_chats {
                let parts: Vec<&str> = chat_pair.split('|').collect();
                if parts.len() == 2 {
                    conversations.push(ChatConversation {
                        id: format!("private_{}", chat_pair),
                        chat_type: "private".to_string(),
                        other_user_id: Some(parts[0].to_string()),
                        other_username: None,
                        group_id: None,
                        group_name: None,
                        last_message: last_msg,
                        message_count: count,
                        last_message_time: parse_datetime(&last_time).unwrap_or_default(),
                    });
                }
            }

            if chat_type_filter.as_deref() != Some("private") {
                let all_groups = match group::Entity::find().all(&**db).await {
                    Ok(g) => g,
                    Err(e) => {
                        log::error!("Failed to get groups: {:?}", e);
                        vec![]
                    }
                };

                for (group_id, (count, last_msg, last_time)) in group_chats {
                    let group_info = all_groups.iter().find(|g| g.id == group_id);
                    conversations.push(ChatConversation {
                        id: group_id.clone(),
                        chat_type: "group".to_string(),
                        other_user_id: None,
                        other_username: None,
                        group_id: Some(group_id.clone()),
                        group_name: group_info.map(|g| g.name.clone()),
                        last_message: last_msg,
                        message_count: count,
                        last_message_time: parse_datetime(&last_time).unwrap_or_default(),
                    });
                }
            }

            if chat_type_filter.as_deref() == Some("private") {
                conversations.retain(|c| c.chat_type == "private");
            } else if chat_type_filter.as_deref() == Some("group") {
                conversations.retain(|c| c.chat_type == "group");
            }

            conversations.sort_by(|a, b| b.last_message_time.cmp(&a.last_message_time));

            let total = conversations.len();
            let paginated: Vec<ChatConversation> = conversations
                .into_iter()
                .skip(skip as usize)
                .take(page_size as usize)
                .collect();

            let all_users = match user::Entity::find().all(&**db).await {
                Ok(u) => u,
                Err(_) => vec![],
            };

            let enriched: Vec<serde_json::Value> = paginated
                .into_iter()
                .map(|c| {
                    if c.chat_type == "group" {
                        serde_json::json!({
                            "id": c.id,
                            "chat_type": c.chat_type,
                            "group_id": c.group_id,
                            "group_name": c.group_name,
                            "last_message": c.last_message,
                            "message_count": c.message_count,
                            "last_message_time": c.last_message_time.to_rfc3339()
                        })
                    } else {
                        let chat_pair = c.id.strip_prefix("private_").unwrap_or(&c.id);
                        let parts: Vec<&str> = chat_pair.split('|').collect();
                        let user1_id = parts.first().map(|s| s.to_string());
                        let user2_id = parts.get(1).map(|s| s.to_string());
                        
                        let user1_name = user1_id.as_ref().and_then(|id| all_users.iter().find(|u| u.id == *id).map(|u| u.username.clone()));
                        let user2_name = user2_id.as_ref().and_then(|id| all_users.iter().find(|u| u.id == *id).map(|u| u.username.clone()));
                        
                        let usernames: Vec<String> = vec![user1_name, user2_name].into_iter().filter_map(|u| u).collect();
                        
                        serde_json::json!({
                            "id": c.id,
                            "chat_type": c.chat_type,
                            "user_ids": serde_json::json!([user1_id, user2_id]),
                            "other_usernames": if usernames.len() == 2 { Some(usernames.join(" & ")) } else { None },
                            "last_message": c.last_message,
                            "message_count": c.message_count,
                            "last_message_time": c.last_message_time.to_rfc3339()
                        })
                    }
                })
                .collect();

            HttpResponse::Ok().json(serde_json::json!({
                "conversations": enriched,
                "total": total,
                "page": page,
                "page_size": page_size
            }))
        }
        Err(e) => {
            log::error!("Failed to get conversations: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

fn format_timestamp(dt: chrono::NaiveDateTime) -> String {
    dt.and_utc().to_rfc3339()
}

fn parse_datetime(s: &str) -> Option<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&chrono::Utc))
}

pub async fn admin_delete_selected_messages(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    body: web::Json<serde_json::Value>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let message_ids: Vec<String> = match body.get("message_ids").and_then(|v| v.as_array()) {
        Some(ids) => ids.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "需要提供 message_ids 数组"
            }));
        }
    };

    if message_ids.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "message_ids 不能为空"
        }));
    }

    match message::Entity::delete_many()
        .filter(message::Column::Id.is_in(message_ids))
        .exec(&**db)
        .await
    {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({
            "message": format!("已删除 {} 条消息", r.rows_affected),
            "deleted_count": r.rows_affected
        })),
        Err(e) => {
            log::error!("Failed to delete messages: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "删除消息失败"
            }))
        }
    }
}

pub async fn admin_get_user_chat_history(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
    query: web::Query<GetMessagesQuery>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let target_user_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(50).min(100);
    let skip = (page - 1) * page_size;

    match user::Entity::find_by_id(&target_user_id).one(&**db).await {
        Ok(Some(_)) => {
            match message::Entity::find()
                .filter(
                    message::Column::SenderId.eq(&target_user_id)
                        .or(message::Column::ReceiverId.eq(&target_user_id))
                )
                .order_by_desc(message::Column::Timestamp)
                .all(&**db)
                .await
            {
                Ok(messages) => {
                    let total = messages.len();
                    let paginated: Vec<AdminMessageResponse> = messages
                        .into_iter()
                        .skip(skip as usize)
                        .take(page_size as usize)
                        .map(|m| AdminMessageResponse {
                            id: m.id,
                            sender_id: m.sender_id,
                            receiver_id: m.receiver_id,
                            content: m.content,
                            message_type: m.message_type,
                            timestamp: m.timestamp.and_utc(),
                            is_read: m.is_read,
                        })
                        .collect();

                    HttpResponse::Ok().json(serde_json::json!({
                        "user_id": target_user_id,
                        "messages": paginated,
                        "total": total,
                        "page": page,
                        "page_size": page_size
                    }))
                }
                Err(e) => {
                    log::error!("Failed to get chat history: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "数据库错误"
                    }))
                }
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "用户不存在"
            }))
        }
        Err(e) => {
            log::error!("Failed to find user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

pub async fn admin_get_group_chat_history(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
    query: web::Query<GetMessagesQuery>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let group_id = path.into_inner();
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(50).min(100);
    let skip = (page - 1) * page_size;

    match group::Entity::find_by_id(&group_id).one(&**db).await {
        Ok(Some(group_info)) => {
            match message::Entity::find()
                .filter(message::Column::ReceiverId.eq(&group_id))
                .order_by_desc(message::Column::Timestamp)
                .all(&**db)
                .await
            {
                Ok(messages) => {
                    let total = messages.len();
                    
                    let all_users = match user::Entity::find().all(&**db).await {
                        Ok(u) => u,
                        Err(_) => vec![],
                    };

                    let paginated: Vec<serde_json::Value> = messages
                        .into_iter()
                        .skip(skip as usize)
                        .take(page_size as usize)
                        .map(|m| {
                            let sender = all_users.iter().find(|u| u.id == m.sender_id);
                            serde_json::json!({
                                "id": m.id,
                                "sender_id": m.sender_id,
                                "sender_name": sender.map(|u| u.username.clone()).unwrap_or_else(|| "未知".to_string()),
                                "content": m.content,
                                "message_type": m.message_type,
                                "timestamp": m.timestamp.and_utc(),
                                "is_read": m.is_read,
                            })
                        })
                        .collect();

                    HttpResponse::Ok().json(serde_json::json!({
                        "group_id": group_id,
                        "group_name": group_info.name,
                        "messages": paginated,
                        "total": total,
                        "page": page,
                        "page_size": page_size
                    }))
                }
                Err(e) => {
                    log::error!("Failed to get group chat history: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "数据库错误"
                    }))
                }
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "群聊不存在"
            }))
        }
        Err(e) => {
            log::error!("Failed to find group: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

pub async fn admin_clear_user_messages(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let target_user_id = path.into_inner();

    match message::Entity::delete_many()
        .filter(
            message::Column::SenderId.eq(&target_user_id)
                .or(message::Column::ReceiverId.eq(&target_user_id))
        )
        .exec(&**db)
        .await
    {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({
            "message": format!("已清空用户聊天记录，共删除 {} 条消息", r.rows_affected),
            "deleted_count": r.rows_affected
        })),
        Err(e) => {
            log::error!("Failed to clear messages: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "清空消息失败"
            }))
        }
    }
}

pub async fn admin_clear_group_messages(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let group_id = path.into_inner();

    match message::Entity::delete_many()
        .filter(message::Column::ReceiverId.eq(&group_id))
        .exec(&**db)
        .await
    {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({
            "message": format!("已清空群聊聊天记录，共删除 {} 条消息", r.rows_affected),
            "deleted_count": r.rows_affected
        })),
        Err(e) => {
            log::error!("Failed to clear group messages: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "清空群聊消息失败"
            }))
        }
    }
}

#[derive(Serialize)]
pub struct AdminUserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub status: String,
    pub is_admin: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

fn check_admin(user: &AuthenticatedUser) -> Option<HttpResponse> {
    if !user.is_admin {
        return Some(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "需要管理员权限"
        })));
    }
    None
}

pub async fn get_all_users(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    match user::Entity::find()
        .order_by_desc(user::Column::CreatedAt)
        .all(&**db)
        .await
    {
        Ok(users) => {
            let user_responses: Vec<AdminUserResponse> = users
                .into_iter()
                .map(|u| AdminUserResponse {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                    avatar_url: u.avatar_url,
                    status: u.status,
                    is_admin: u.is_admin == 1,
                    created_at: u.created_at.and_utc(),
                })
                .collect();
            HttpResponse::Ok().json(user_responses)
        }
        Err(e) => {
            log::error!("Failed to get all users: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

pub async fn delete_user(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let target_id = path.into_inner();
    
    if target_id == user.user_id {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "不能删除自己"
        }));
    }

    match user::Entity::find_by_id(target_id.clone())
        .one(&**db)
        .await
    {
        Ok(Some(target_user)) => {
            if target_user.is_admin == 1 {
                return HttpResponse::Forbidden().json(serde_json::json!({
                    "error": "不能删除管理员"
                }));
            }

            match user::Entity::delete_many()
                .filter(user::Column::Id.eq(target_id.clone()))
                .exec(&**db)
                .await {
                Ok(_) => {
                    let _ = message::Entity::delete_many()
                        .filter(message::Column::SenderId.eq(&target_id))
                        .exec(&**db)
                        .await;
                    let _ = message::Entity::delete_many()
                        .filter(message::Column::ReceiverId.eq(&target_id))
                        .exec(&**db)
                        .await;
                    
                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "用户已删除"
                    }))
                }
                Err(e) => {
                    log::error!("Failed to delete user: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "删除用户失败"
                    }))
                }
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "用户不存在"
            }))
        }
        Err(e) => {
            log::error!("Failed to find user: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

pub async fn admin_delete_message(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    let message_id = path.into_inner();

    match message::Entity::find_by_id(&message_id).one(&**db).await {
        Ok(Some(_)) => {
            match message::Entity::delete_many()
                .filter(message::Column::Id.eq(&message_id))
                .exec(&**db)
                .await {
                Ok(_) => {
                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "消息已删除"
                    }))
                }
                Err(e) => {
                    log::error!("Failed to delete message: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "删除消息失败"
                    }))
                }
            }
        }
        Ok(None) => {
            HttpResponse::NotFound().json(serde_json::json!({
                "error": "消息不存在"
            }))
        }
        Err(e) => {
            log::error!("Failed to find message: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

pub async fn admin_delete_all_users(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    match user::Entity::find()
        .filter(user::Column::IsAdmin.eq(0))
        .filter(user::Column::Id.ne(user.user_id.clone()))
        .all(&**db)
        .await
    {
        Ok(non_admin_users) => {
            let other_ids: Vec<String> = non_admin_users.iter().map(|u| u.id.clone()).collect();

            if !other_ids.is_empty() {
                let _ = message::Entity::delete_many()
                    .filter(message::Column::SenderId.is_in(other_ids.clone()))
                    .exec(&**db)
                    .await;
                let _ = message::Entity::delete_many()
                    .filter(message::Column::ReceiverId.is_in(other_ids.clone()))
                    .exec(&**db)
                    .await;
                let _ = group_member::Entity::delete_many()
                    .filter(group_member::Column::UserId.is_in(other_ids.clone()))
                    .exec(&**db)
                    .await;
                let _ = group::Entity::delete_many()
                    .filter(group::Column::CreatorId.is_in(other_ids.clone()))
                    .exec(&**db)
                    .await;
            }

            let deleted_count = match user::Entity::delete_many()
                .filter(user::Column::Id.is_in(other_ids.clone()))
                .exec(&**db)
                .await
            {
                Ok(r) => r.rows_affected,
                Err(e) => {
                    log::error!("Failed to delete users: {:?}", e);
                    return HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "删除用户失败"
                    }));
                }
            };

            HttpResponse::Ok().json(serde_json::json!({
                "message": format!("已删除 {} 个用户", deleted_count),
                "deleted_count": deleted_count
            }))
        }
        Err(e) => {
            log::error!("Failed to get users: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

pub async fn admin_delete_all_groups(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    match group::Entity::find().all(&**db).await {
        Ok(groups) => {
            let group_ids: Vec<String> = groups.iter().map(|g| g.id.clone()).collect();
            if !group_ids.is_empty() {
                let _ = group_member::Entity::delete_many()
                    .filter(group_member::Column::GroupId.is_in(group_ids.clone()))
                    .exec(&**db)
                    .await;
            }
            match group::Entity::delete_many().exec(&**db).await {
                Ok(r) => HttpResponse::Ok().json(serde_json::json!({
                    "message": format!("已删除 {} 个群聊", r.rows_affected),
                    "deleted_count": r.rows_affected
                })),
                Err(e) => {
                    log::error!("Failed to delete groups: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "删除群聊失败"
                    }))
                }
            }
        }
        Err(e) => {
            log::error!("Failed to get groups: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "数据库错误"
            }))
        }
    }
}

pub async fn admin_delete_all_messages(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> HttpResponse {
    if let Some(resp) = check_admin(&user) {
        return resp;
    }

    match message::Entity::delete_many().exec(&**db).await {
        Ok(r) => HttpResponse::Ok().json(serde_json::json!({
            "message": format!("已删除 {} 条消息", r.rows_affected),
            "deleted_count": r.rows_affected
        })),
        Err(e) => {
            log::error!("Failed to delete messages: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "删除消息失败"
            }))
        }
    }
}
