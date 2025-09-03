use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use sqlx::PgPool;
use crate::models::{CreateUserRequest, User};
use crate::auth::{hash_password, extract_token_from_header, validate_token};
use crate::database::DbPool;

pub async fn create_user(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Verify admin token
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization header"))?;

    let token = extract_token_from_header(auth_header)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid authorization header format"))?;

    let claims = validate_token(&token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // Only admins can create users
    if claims.role != "Admin" {
        return Err(actix_web::error::ErrorForbidden("Only admins can create users"));
    }

    // Check if user already exists
    let existing_user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1 OR email = $2"
    )
    .bind(&user_data.username)
    .bind(&user_data.email)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    if existing_user.is_some() {
        return Err(actix_web::error::ErrorBadRequest("Username or email already exists"));
    }

    let password_hash = hash_password(&user_data.password)
        .map_err(|e| {
            eprintln!("Password hashing error: {}", e);
            actix_web::error::ErrorInternalServerError("Password hashing error")
        })?;

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, email, password_hash, role)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#
    )
    .bind(&user_data.username)
    .bind(&user_data.email)
    .bind(&password_hash)
    .bind(&user_data.role)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().json(user))
}

pub async fn get_users(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // Verify admin token
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization header"))?;

    let token = extract_token_from_header(auth_header)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid authorization header format"))?;

    let claims = validate_token(&token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // Only admins can view all users
    if claims.role != "Admin" {
        return Err(actix_web::error::ErrorForbidden("Only admins can view all users"));
    }

    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(users))
}
