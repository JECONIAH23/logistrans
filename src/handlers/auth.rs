use actix_web::{web, HttpResponse, HttpRequest};
use serde_json::json;
use sqlx::PgPool;
use crate::models::{LoginRequest, CreateUserRequest, AuthResponse, User};
use crate::auth::{hash_password, verify_password, create_token, extract_token_from_header, validate_token};
use crate::database::DbPool;

pub async fn login(
    pool: web::Data<DbPool>,
    login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&login_data.username)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    let user = user.ok_or_else(|| {
        actix_web::error::ErrorUnauthorized("Invalid credentials")
    })?;

    if !verify_password(&login_data.password, &user.password_hash)
        .map_err(|e| {
            eprintln!("Password verification error: {}", e);
            actix_web::error::ErrorInternalServerError("Password verification error")
        })? {
        return Err(actix_web::error::ErrorUnauthorized("Invalid credentials"));
    }

    let token = create_token(&user.id.to_string(), &user.username, &format!("{:?}", user.role))
        .map_err(|e| {
            eprintln!("Token creation error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation error")
        })?;

    let response = AuthResponse {
        token,
        user,
    };

    Ok(HttpResponse::Ok().json(response))
}

pub async fn register(
    pool: web::Data<DbPool>,
    user_data: web::Json<CreateUserRequest>,
) -> Result<HttpResponse, actix_web::Error> {
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

    let token = create_token(&user.id.to_string(), &user.username, &format!("{:?}", user.role))
        .map_err(|e| {
            eprintln!("Token creation error: {}", e);
            actix_web::error::ErrorInternalServerError("Token creation error")
        })?;

    let response = AuthResponse {
        token,
        user,
    };

    Ok(HttpResponse::Created().json(response))
}

pub async fn verify_token(
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization header"))?;

    let token = extract_token_from_header(auth_header)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid authorization header format"))?;

    let claims = validate_token(&token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    Ok(HttpResponse::Ok().json(json!({
        "valid": true,
        "user_id": claims.sub,
        "username": claims.username,
        "role": claims.role
    })))
}
