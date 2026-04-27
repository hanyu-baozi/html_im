use actix_web::{web, App, HttpServer, middleware as actix_middleware};
use actix_cors::Cors;
use dotenv::dotenv;
use log::info;
use std::sync::{Arc, Mutex};
use crate::websocket::ConnectionManager;

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
    
    let db_pool = db::establish_connection().await;
    let connection_manager = Arc::new(Mutex::new(ConnectionManager::new()));
    
    HttpServer::new(move || {
        let cors = Cors::permissive();
        
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(connection_manager.clone()))
            .wrap(cors)
            .wrap(actix_middleware::Logger::default())
            .route("/", web::get().to(routes::health))
            .service(routes::auth_routes())
            .service(routes::user_routes())
            .service(routes::message_routes())
            .service(routes::friend_routes())
            .service(routes::group_routes())
            .route("/ws", web::get().to(websocket::websocket_route))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
