use sea_orm::DatabaseConnection;

pub async fn establish_connection() -> DatabaseConnection {
    sea_orm::Database::connect("postgres://postgres:123456@xxx.com:1234/mydb")
        .await
        .expect("Failed to connect to database")
}