use actix_web::{web, HttpResponse, Responder};
use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use utoipa::{schema, ToSchema};
use crate::entity::user::{self, ActiveModel as UserActiveModel};

use serde::Deserialize;
use sea_orm::prelude::DateTime;


/// 表示创建新用户的请求体结构
#[derive(Debug, Deserialize, ToSchema)]
#[schema(example = json!({
    "username": "johndoe",
    "birthday": "2023-09-09T15:53:00",
    "sex": "male",
    "address": "123 Main St, Anytown, USA"
}))]
pub struct CreateUser {
    /// 用户名    
    pub username: String,
    /// 生日（可选）
    #[schema(value_type = String)]
    pub birthday: Option<DateTime>,
    /// 性别（可选）
    pub sex: Option<String>,
    /// 地址（可选）
    pub address: Option<String>,
}
/// 创建新用户
///
/// # 请求体
///
/// 需要一个JSON对象，包含以下字段：
/// - `username`: 字符串，用户名（必填）
/// - `birthday`: ISO 8601格式的日期时间字符串，用户生日（可选）
/// - `sex`: 字符串，用户性别（可选）
/// - `address`: 字符串，用户地址（可选）
///
/// # 响应
///
/// - 成功：返回状态码200和新创建的用户JSON对象
/// - 失败：返回状态码500
///
/// # 示例
///
/// ```
/// POST /users
/// Content-Type: application/json
///
/// {
///     "username": "johndoe",
///     "birthday": "1990-01-01T00:00:00",
///     "sex": "M",
///     "address": "123 Main St, Anytown, USA"
/// }
/// ```
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUser,
    responses(
        (status = 200, description = "User created successfully", body = Model),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    user_data: web::Json<CreateUser>,
) -> impl Responder {
    let user = UserActiveModel {
        username: Set(user_data.username.clone()),
        birthday: Set(user_data.birthday),
        sex: Set(user_data.sex.clone()),
        address: Set(user_data.address.clone()),
        ..Default::default()
    };

    let result = user.insert(db.get_ref()).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
/// 获取指定ID的用户信息
///
/// # 路径参数
///
/// - `id`: 整数，用户ID
///
/// # 响应
///
/// - 成功：返回状态码200和用户JSON对象
/// - 未找到：返回状态码404
/// - 失败：返回状态码500
///
/// # 示例
///
/// ```
/// GET /users/1
/// ```
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    responses(
        (status = 200, description = "User found", body = Model),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "User ID")
    )
)]
pub async fn get_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i32>,
) -> impl Responder {
    let user = user::Entity::find_by_id(*id).one(db.get_ref()).await;
    println!("{id}");
    match user {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
/// 更新指定ID的用户信息
///
/// # 路径参数
///
/// - `id`: 整数，用户ID
///
/// # 请求体
///
/// 需要一个JSON对象，包含以下字段（所有字段都是可选的）：
/// - `username`: 字符串，新的用户名
/// - `birthday`: ISO 8601格式的日期时间字符串，新的用户生日
/// - `sex`: 字符串，新的用户性别
/// - `address`: 字符串，新的用户地址
///
/// # 响应
///
/// - 成功：返回状态码200和更新后的用户JSON对象
/// - 未找到：返回状态码404
/// - 失败：返回状态码500
///
/// # 示例
///
/// ```
/// PUT /users/1
/// Content-Type: application/json
///
/// {
///     "username": "johndoe_updated",
///     "address": "456 Elm St, Newtown, USA"
/// }
/// ```
#[utoipa::path(
    put,
    path = "/api/users/{id}",
    request_body = CreateUser,
    responses(
        (status = 200, description = "User updated successfully", body = Model),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "User ID")
    )
)]
pub async fn update_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i32>,
    user_data: web::Json<CreateUser>,
) -> impl Responder {
    let user = user::Entity::find_by_id(*id).one(db.get_ref()).await;

    match user {
        Ok(Some(user)) => {
            let mut user: UserActiveModel = user.into();
            user.username = Set(user_data.username.clone());
            user.birthday = Set(user_data.birthday);
            user.sex = Set(user_data.sex.clone());
            user.address = Set(user_data.address.clone());

            let result = user.update(db.get_ref()).await;

            match result {
                Ok(updated_user) => HttpResponse::Ok().json(updated_user),
                Err(_) => HttpResponse::InternalServerError().finish(),
            }
        }
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
/// 删除指定ID的用户
///
/// # 路径参数
///
/// - `id`: 整数，用户ID
///
/// # 响应
///
/// - 成功：返回状态码204（无内容）
/// - 失败：返回状态码500
///
/// # 示例
///
/// ```
/// DELETE /users/1
/// ```
#[utoipa::path(
    delete,
    path = "/api/users/{id}",
    responses(
        (status = 204, description = "User deleted successfully"),
        (status = 500, description = "Internal server error")
    ),
    params(
        ("id" = i32, Path, description = "User ID")
    )
)]
pub async fn delete_user(
    db: web::Data<sea_orm::DatabaseConnection>,
    id: web::Path<i32>,
) -> impl Responder {
    let result = user::Entity::delete_by_id(*id).exec(db.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}