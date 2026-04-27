use sea_orm::{Database, DatabaseConnection, DbErr};
use log::info;

pub async fn establish_connection() -> DatabaseConnection {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:password@localhost:3306/html_im".to_string());
    
    info!("Connecting to database: {}", database_url);
    
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    info!("Database connection established");
    
    db
}
