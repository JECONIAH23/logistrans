use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    Manager,
    Driver,
    Dispatcher,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Vehicle {
    pub id: Uuid,
    pub license_plate: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub capacity: f64,
    pub fuel_type: String,
    pub status: VehicleStatus,
    pub driver_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "vehicle_status", rename_all = "lowercase")]
pub enum VehicleStatus {
    Available,
    InUse,
    Maintenance,
    OutOfService,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Cargo {
    pub id: Uuid,
    pub description: String,
    pub weight: f64,
    pub volume: f64,
    pub cargo_type: String,
    pub priority: CargoPriority,
    pub status: CargoStatus,
    pub shipper_id: Uuid,
    pub consignee_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cargo_priority", rename_all = "lowercase")]
pub enum CargoPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "cargo_status", rename_all = "lowercase")]
pub enum CargoStatus {
    Pending,
    Assigned,
    InTransit,
    Delivered,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Route {
    pub id: Uuid,
    pub source_address: String,
    pub source_lat: f64,
    pub source_lng: f64,
    pub destination_address: String,
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub estimated_distance: f64,
    pub estimated_duration: i32,
    pub vehicle_id: Uuid,
    pub driver_id: Uuid,
    pub cargo_id: Uuid,
    pub status: RouteStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "route_status", rename_all = "lowercase")]
pub enum RouteStatus {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Location {
    pub id: Uuid,
    pub route_id: Uuid,
    pub vehicle_id: Uuid,
    pub driver_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateVehicleRequest {
    pub license_plate: String,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub capacity: f64,
    pub fuel_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCargoRequest {
    pub description: String,
    pub weight: f64,
    pub volume: f64,
    pub cargo_type: String,
    pub priority: CargoPriority,
    pub shipper_id: Uuid,
    pub consignee_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRouteRequest {
    pub source_address: String,
    pub source_lat: f64,
    pub source_lng: f64,
    pub destination_address: String,
    pub destination_lat: f64,
    pub destination_lng: f64,
    pub vehicle_id: Uuid,
    pub driver_id: Uuid,
    pub cargo_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateLocationRequest {
    pub route_id: Uuid,
    pub vehicle_id: Uuid,
    pub driver_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,
}
