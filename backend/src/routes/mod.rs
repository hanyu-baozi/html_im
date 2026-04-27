use actix_web::{web, HttpResponse};
use actix_web::web::ServiceConfig;

pub mod auth;
pub mod user;
pub mod message;
pub mod friend;
pub mod group;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg
        .service(auth_routes())
        .service(user_routes())
        .service(message_routes())
        .service(friend_routes())
        .service(group_routes());
}

pub fn auth_routes() -> actix_web::Scope {
    web::scope("/api/auth")
        .route("/register", web::post().to(auth::register))
        .route("/login", web::post().to(auth::login))
        .route("/logout", web::post().to(auth::logout))
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

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "service": "html-im-backend"
    }))
}
