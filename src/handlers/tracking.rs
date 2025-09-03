use actix_web::{web, HttpResponse, HttpRequest};
use sqlx::PgPool;
use crate::models::{UpdateLocationRequest, Location};
use crate::auth::{extract_token_from_header, validate_token};
use crate::database::DbPool;

pub async fn update_location(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    location_data: web::Json<UpdateLocationRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Verify token
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Missing authorization header"))?;

    let token = extract_token_from_header(auth_header)
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Invalid authorization header format"))?;

    let claims = validate_token(&token)
        .map_err(|_| actix_web::error::ErrorUnauthorized("Invalid token"))?;

    // Only drivers can update their own location
    if claims.role != "Driver" {
        return Err(actix_web::error::ErrorForbidden("Only drivers can update location"));
    }

    // Verify the driver is updating their own location
    if claims.sub != location_data.driver_id.to_string() {
        return Err(actix_web::error::ErrorForbidden("Can only update your own location"));
    }

    let location = sqlx::query_as::<_, Location>(
        r#"
        INSERT INTO locations (route_id, vehicle_id, driver_id, latitude, longitude, speed, heading)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING *
        "#
    )
    .bind(&location_data.route_id)
    .bind(&location_data.vehicle_id)
    .bind(&location_data.driver_id)
    .bind(&location_data.latitude)
    .bind(&location_data.longitude)
    .bind(&location_data.speed)
    .bind(&location_data.heading)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().json(location))
}

pub async fn get_location(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    route_id: web::Path<uuid::Uuid>,
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

    let location = sqlx::query_as::<_, Location>(
        r#"
        SELECT * FROM locations 
        WHERE route_id = $1 
        ORDER BY timestamp DESC 
        LIMIT 1
        "#
    )
    .bind(&route_id.into_inner())
    .fetch_optional(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    match location {
        Some(loc) => Ok(HttpResponse::Ok().json(loc)),
        None => Err(actix_web::error::ErrorNotFound("Location not found for this route")),
    }
}

pub async fn get_route_tracking_history(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    route_id: web::Path<uuid::Uuid>,
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

    let locations = sqlx::query_as::<_, Location>(
        r#"
        SELECT * FROM locations 
        WHERE route_id = $1 
        ORDER BY timestamp ASC
        "#
    )
    .bind(&route_id.into_inner())
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(locations))
}
