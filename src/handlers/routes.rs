use actix_web::{web, HttpResponse, HttpRequest};
use sqlx::PgPool;
use crate::models::{CreateRouteRequest, Route};
use crate::auth::{extract_token_from_header, validate_token};
use crate::database::DbPool;

pub async fn create_route(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    route_data: web::Json<CreateRouteRequest>,
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

    // Only admins and managers can create routes
    if claims.role != "Admin" && claims.role != "Manager" {
        return Err(actix_web::error::ErrorForbidden("Only admins and managers can create routes"));
    }

    // Calculate estimated distance and duration (simplified calculation)
    let distance = calculate_distance(
        route_data.source_lat,
        route_data.source_lng,
        route_data.destination_lat,
        route_data.destination_lng,
    );
    
    let duration = (distance / 50.0 * 60.0) as i32; // Assuming 50 km/h average speed

    let route = sqlx::query_as::<_, Route>(
        r#"
        INSERT INTO routes (
            source_address, source_lat, source_lng,
            destination_address, destination_lat, destination_lng,
            estimated_distance, estimated_duration, vehicle_id, driver_id, cargo_id
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        RETURNING *
        "#
    )
    .bind(&route_data.source_address)
    .bind(&route_data.source_lat)
    .bind(&route_data.source_lng)
    .bind(&route_data.destination_address)
    .bind(&route_data.destination_lat)
    .bind(&route_data.destination_lng)
    .bind(&distance)
    .bind(&duration)
    .bind(&route_data.vehicle_id)
    .bind(&route_data.driver_id)
    .bind(&route_data.cargo_id)
    .fetch_one(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Created().json(route))
}

pub async fn get_routes(
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

    let routes = sqlx::query_as::<_, Route>(
        "SELECT * FROM routes ORDER BY created_at DESC"
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(|e| {
        eprintln!("Database error: {}", e);
        actix_web::error::ErrorInternalServerError("Database error")
    })?;

    Ok(HttpResponse::Ok().json(routes))
}

// Calculate distance between two points using Haversine formula
fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 6371.0; // Earth's radius in kilometers
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin() +
             lat1.to_radians().cos() * lat2.to_radians().cos() *
             (dlon / 2.0).sin() * (dlon / 2.0).sin();
    let c = 2.0 * a.sqrt().asin();
    r * c
}
