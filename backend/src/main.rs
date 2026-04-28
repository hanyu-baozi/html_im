use actix_web::{web, App, HttpServer, middleware as actix_middleware, http::header};
use actix_cors::Cors;
use actix_files as fs;
use dotenv::dotenv;
use log::info;
use std::sync::{Arc, Mutex};
use crate::websocket::ConnectionManager;
use crate::config::Config;

mod config;
mod db;
mod models;
mod routes;
mod websocket;
mod middleware;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    
    info!("Starting HTML-IM Backend Server...");
    
    let config = Config::from_env();
    let db_pool = db::establish_connection().await;
    let connection_manager = Arc::new(Mutex::new(ConnectionManager::new()));
    let captcha_store = routes::captcha::get_captcha_store();
    
    let server_host = config.server_host.clone();
    let server_port = config.server_port;
    
    info!("Server will bind to {}:{}", server_host, server_port);

    // Create uploads directory if it doesn't exist
    let uploads_dir = std::path::Path::new("uploads");
    if !uploads_dir.exists() {
        std::fs::create_dir_all(uploads_dir).expect("Failed to create uploads directory");
        info!("Created uploads directory");
    }

    HttpServer::new(move || {
        // 配置 CORS，限制访问来源
        let allowed_origin = std::env::var("ALLOWED_ORIGIN")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());
        
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
                header::HeaderName::from_static("x-csrf-token"),
            ])
            .max_age(3600);
        
        App::new()
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(connection_manager.clone()))
            .app_data(web::Data::new(captcha_store.clone()))
            .wrap(cors)
            .wrap(actix_middleware::Logger::default())
            .route("/", web::get().to(routes::health))
            .service(routes::auth_routes())
            .service(routes::captcha_routes())
            .service(routes::user_routes())
            .service(routes::message_routes())
            .service(routes::friend_routes())
            .service(routes::group_routes())
            .service(routes::admin_routes())
            .service(routes::upload_routes())
            .service(fs::Files::new("/uploads", "uploads").show_files_listing())
            .route("/ws", web::get().to(websocket::websocket_route))
    })
    .bind((server_host, server_port))?
    .run()
    .await
}
