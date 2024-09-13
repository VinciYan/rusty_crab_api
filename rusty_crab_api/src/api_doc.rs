use utoipa::OpenApi;
use crate::handlers::user::CreateUser;
use crate::entity::user::Model;  // 确保这行存在
use crate::{handlers};

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::user::create_user,
        handlers::user::get_user,
        handlers::user::update_user,
        handlers::user::delete_user
    ),
    components(
        schemas(Model,CreateUser)
    ),
    tags(
        (name = "users", description = "User management API")
    )
)]
pub struct ApiDoc;