use actix_web::{web, App, HttpServer};
use api_doc::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use sea_orm::DatabaseConnection;
use handlers::user::{create_user, get_user, update_user, delete_user};

mod entity;
mod handlers;
mod db;
mod api_doc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: DatabaseConnection = db::establish_connection().await;
    let db_data = web::Data::new(db);

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/users")
                            .route("", web::post().to(create_user))
                            .route("/{id}", web::get().to(get_user))
                            .route("/{id}", web::put().to(update_user))
                            .route("/{id}", web::delete().to(delete_user))
                            .route("/test", web::get().to(|| async { "Hello, World!" }))
                    )
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}