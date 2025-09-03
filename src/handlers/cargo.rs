use actix_web::{web, HttpResponse, HttpRequest};
use sqlx::PgPool;
use crate::models::{CreateCargoRequest, Cargo};
use crate::auth::{extract_token_from_header, validate_token};
use crate::database::DbPool;

pub async fn create_cargo(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    cargo_data: web::Json<CreateCargoRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Verify token
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization header"))?;

    let token = extract_token_from_header(auth_header)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid authorization header format"))?;

    let _claims = validate_token(&token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let cargo = sqlx::query_as::<_, Cargo>(
        r#"
        INSERT INTO cargo (description, weight, volume, cargo_type, priority, shipper_id, consignee_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#
    )
    .bind(&cargo_data.description)
    .bind(&cargo_data.weight)
    .bind(&cargo_data.volume)
    .bind(&cargo_data.cargo_type)
    .bind(&cargo_data.priority)
    .bind(&cargo_data.shipper_id)
    .bind(&cargo_data.consignee_id)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().json(cargo))
}

pub async fn get_cargo(
    pool: web::Data<DbPool>,
    req: HttpRequest,
) -> Result<HttpResponse, actix_web::Error> {
    // Verify token
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization header"))?;

    let token = extract_token_from_header(auth_header)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid authorization header format"))?;

    let _claims = validate_token(&token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    let cargo = sqlx::query_as::<_, Cargo>(
        "SELECT * FROM cargo ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(cargo))
}
