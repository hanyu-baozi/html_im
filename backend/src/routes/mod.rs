use actix_web::{web, HttpResponse};
use actix_web::web::ServiceConfig;

pub mod auth;
pub mod user;
pub mod message;
pub mod friend;
pub mod group;
pub mod captcha;
pub mod admin;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(auth_routes())
        .service(captcha_routes())
        .service(user_routes())
        .service(message_routes())
        .service(friend_routes())
        .service(group_routes())
        .service(admin_routes());
}

pub fn auth_routes() -> actix_web::Scope {
    web::scope("/api/auth")
        .route("/register", web::post().to(auth::register))
        .route("/login", web::post().to(auth::login))
        .route("/logout", web::post().to(auth::logout))
}

pub fn captcha_routes() -> actix_web::Scope {
    web::scope("/api/captcha")
        .route("", web::get().to(captcha::get_captcha))
        .route("/verify", web::post().to(captcha::verify_captcha))
}

pub fn user_routes() -> actix_web::Scope {
    web::scope("/api/users")
        .route("/me", web::get().to(user::get_me))
        .route("/search", web::get().to(user::search_users))
        .route("/search-by-email", web::get().to(user::search_users_by_email))
        .route("", web::get().to(user::get_users))
        .route("/{id}", web::get().to(user::get_user))
        .route("/status", web::put().to(user::update_status))
}

pub fn message_routes() -> actix_web::Scope {
    web::scope("/api/messages")
        .route("", web::get().to(message::get_messages))
        .route("", web::post().to(message::send_message))
        .route("/{id}/read", web::post().to(message::mark_as_read))
        .route("/chat", web::delete().to(message::delete_chat_messages))
}

pub fn friend_routes() -> actix_web::Scope {
    web::scope("/api/friends")
        .route("", web::get().to(friend::get_friends))
        .route("/add", web::post().to(friend::add_friend))
        .route("/add-by-email", web::post().to(friend::add_friend_by_email))
        .route("/remove", web::delete().to(friend::remove_friend))
}

pub fn group_routes() -> actix_web::Scope {
    web::scope("/api/groups")
        .route("", web::get().to(group::get_user_groups))
        .route("", web::post().to(group::create_group))
        .route("/{id}/members", web::get().to(group::get_group_members))
        .route("/{id}/members", web::post().to(group::add_group_member))
}

pub fn admin_routes() -> actix_web::Scope {
    web::scope("/api/admin")
        .route("/users", web::get().to(admin::get_all_users))
        .route("/users/delete-all", web::delete().to(admin::admin_delete_all_users))
        .route("/groups/delete-all", web::delete().to(admin::admin_delete_all_groups))
        .route("/messages/delete-all", web::delete().to(admin::admin_delete_all_messages))
        .route("/messages", web::get().to(admin::admin_get_all_messages))
        .route("/conversations", web::get().to(admin::admin_get_chat_conversations))
        .route("/messages/delete-selected", web::delete().to(admin::admin_delete_selected_messages))
        .route("/users/{id}/messages", web::get().to(admin::admin_get_user_chat_history))
        .route("/users/{id}/messages/clear", web::delete().to(admin::admin_clear_user_messages))
        .route("/users/{id}", web::delete().to(admin::delete_user))
        .route("/messages/{id}", web::delete().to(admin::admin_delete_message))
        .route("/groups/{group_id}/messages", web::get().to(admin::admin_get_group_chat_history))
        .route("/groups/{group_id}/messages/clear", web::delete().to(admin::admin_clear_group_messages))
}

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "html-im-backend"
    }))
}
