use sea_orm::DatabaseConnection;

pub async fn establish_connection() -> DatabaseConnection {
    sea_orm::Database::connect("postgres://postgres:961010NBMM@vinciyan.com:15003/mydb")
        .await
        .expect("Failed to connect to database")
}