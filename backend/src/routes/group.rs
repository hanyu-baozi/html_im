use actix_web::{web, HttpResponse};
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait, ColumnTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::{group, group_member, user};
use crate::middleware::auth::AuthenticatedUser;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
    pub member_ids: Vec<String>,
}

#[derive(Serialize)]
pub struct GroupResponse {
    pub id: String,
    pub name: String,
    pub creator_id: String,
    pub avatar_url: Option<String>,
    pub created_at: i64,
}

#[derive(Serialize)]
pub struct GroupMemberResponse {
    pub id: String,
    pub username: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct AddMemberRequest {
    pub user_id: String,
}

pub async fn create_group(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    req: web::Json<CreateGroupRequest>,
) -> HttpResponse {
    let group_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();

    let new_group = group::ActiveModel {
        id: Set(group_id.clone()),
        name: Set(req.name.clone()),
        creator_id: Set(user.user_id.clone()),
        avatar_url: Set(None),
        created_at: Set(now),
    };

    match new_group.insert(&**db).await {
        Ok(_) => {
            let mut all_member_ids = req.member_ids.clone();
            if !all_member_ids.contains(&user.user_id) {
                all_member_ids.push(user.user_id.clone());
            }

            for member_id in all_member_ids {
                let member_id_str = Uuid::new_v4().to_string();
                let new_member = group_member::ActiveModel {
                    id: Set(member_id_str),
                    group_id: Set(group_id.clone()),
                    user_id: Set(member_id),
                    joined_at: Set(now),
                };
                let _ = new_member.insert(&**db).await;
            }

            HttpResponse::Ok().json(GroupResponse {
                id: group_id,
                name: req.name.clone(),
                creator_id: user.user_id,
                avatar_url: None,
                created_at: now.and_utc().timestamp_millis(),
            })
        }
        Err(e) => {
            log::error!("Failed to create group: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to create group"
            }))
        }
    }
}

pub async fn get_user_groups(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
) -> HttpResponse {
    match group_member::Entity::find()
        .filter(group_member::Column::UserId.eq(&user.user_id))
        .all(&**db)
        .await
    {
        Ok(memberships) => {
            let group_ids: Vec<String> = memberships.iter().map(|m| m.group_id.clone()).collect();
            
            match group::Entity::find()
                .filter(group::Column::Id.is_in(group_ids))
                .all(&**db)
                .await
            {
                Ok(groups) => {
                    let responses: Vec<GroupResponse> = groups
                        .into_iter()
                        .map(|g| GroupResponse {
                            id: g.id,
                            name: g.name,
                            creator_id: g.creator_id,
                            avatar_url: g.avatar_url,
                            created_at: g.created_at.and_utc().timestamp_millis(),
                        })
                        .collect();
                    HttpResponse::Ok().json(responses)
                }
                Err(e) => {
                    log::error!("Failed to get groups: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Database error"
                    }))
                }
            }
        }
        Err(e) => {
            log::error!("Failed to get memberships: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn get_group_members(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> HttpResponse {
    let group_id = path.into_inner();

    match group_member::Entity::find()
        .filter(group_member::Column::GroupId.eq(&group_id))
        .all(&**db)
        .await
    {
        Ok(memberships) => {
            let user_ids: Vec<String> = memberships.iter().map(|m| m.user_id.clone()).collect();
            
            match user::Entity::find()
                .filter(user::Column::Id.is_in(user_ids))
                .all(&**db)
                .await
            {
                Ok(users) => {
                    let responses: Vec<GroupMemberResponse> = users
                        .into_iter()
                        .map(|u| GroupMemberResponse {
                            id: u.id,
                            username: u.username,
                            status: u.status,
                        })
                        .collect();
                    HttpResponse::Ok().json(responses)
                }
                Err(e) => {
                    log::error!("Failed to get group members: {:?}", e);
                    HttpResponse::InternalServerError().json(serde_json::json!({
                        "error": "Database error"
                    }))
                }
            }
        }
        Err(e) => {
            log::error!("Failed to get memberships: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Database error"
            }))
        }
    }
}

pub async fn add_group_member(
    db: web::Data<DatabaseConnection>,
    user: AuthenticatedUser,
    path: web::Path<String>,
    req: web::Json<AddMemberRequest>,
) -> HttpResponse {
    let group_id = path.into_inner();

    let existing = group_member::Entity::find()
        .filter(group_member::Column::GroupId.eq(&group_id))
        .filter(group_member::Column::UserId.eq(&req.user_id))
        .one(&**db)
        .await;

    if let Ok(Some(_)) = existing {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": "User is already in the group"
        }));
    }

    let member_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().naive_utc();

    let new_member = group_member::ActiveModel {
        id: Set(member_id),
        group_id: Set(group_id),
        user_id: Set(req.user_id.clone()),
        joined_at: Set(now),
    };

    match new_member.insert(&**db).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "message": "Member added successfully"
        })),
        Err(e) => {
            log::error!("Failed to add member: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": "Failed to add member"
            }))
        }
    }
}
