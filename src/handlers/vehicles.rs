use actix_web::{web, HttpResponse, HttpRequest};
use sqlx::PgPool;
use crate::models::{CreateVehicleRequest, Vehicle};
use crate::auth::{extract_token_from_header, validate_token};
use crate::database::DbPool;

pub async fn register_vehicle(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    vehicle_data: web::Json<CreateVehicleRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Verify admin or manager token
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization header"))?;

    let token = extract_token_from_header(auth_header)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid authorization header format"))?;

    let claims = validate_token(&token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // Only admins and managers can register vehicles
    if claims.role != "Admin" && claims.role != "Manager" {
        return Err(actix_web::error::ErrorForbidden("Only admins and managers can register vehicles"));
    }

    // Check if vehicle already exists
    let existing_vehicle = sqlx::query_as::<_, Vehicle>(
        "SELECT * FROM vehicles WHERE license_plate = $1"
    )
    .bind(&vehicle_data.license_plate)
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    if existing_vehicle.is_some() {
        return Err(actix_web::error::ErrorBadRequest("Vehicle with this license plate already exists"));
    }

    let vehicle = sqlx::query_as::<_, Vehicle>(
        r#"
        INSERT INTO vehicles (license_plate, make, model, year, capacity, fuel_type)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    )
    .bind(&vehicle_data.license_plate)
    .bind(&vehicle_data.make)
    .bind(&vehicle_data.model)
    .bind(&vehicle_data.year)
    .bind(&vehicle_data.capacity)
    .bind(&vehicle_data.fuel_type)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().json(vehicle))
}

pub async fn get_vehicles(
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

    let vehicles = sqlx::query_as::<_, Vehicle>(
        "SELECT * FROM vehicles ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(vehicles))
}
