use actix_web::{web, App, HttpServer, middleware};
use actix_cors::Cors;
use actix_files::Files;
use log::info;
use std::env;

mod models;
mod handlers;
mod database;
mod auth;
mod websocket;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost/logistrans".to_string());
    
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("127.0.0.1:{}", port);

    info!("Starting LogisTrans server on {}", bind_address);
    info!("Database URL: {}", database_url);

    // Initialize database
    if let Err(e) = database::init_database(&database_url).await {
        eprintln!("Failed to initialize database: {}", e);
        std::process::exit(1);
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(handlers::auth::login)
                    .service(handlers::auth::register)
                    .service(handlers::users::create_user)
                    .service(handlers::users::get_users)
                    .service(handlers::vehicles::register_vehicle)
                    .service(handlers::vehicles::get_vehicles)
                    .service(handlers::cargo::create_cargo)
                    .service(handlers::cargo::get_cargo)
                    .service(handlers::routes::create_route)
                    .service(handlers::routes::get_routes)
                    .service(handlers::tracking::update_location)
                    .service(handlers::tracking::get_location)
            )
            .service(web::resource("/ws").to(websocket::ws_index))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind(&bind_address)?
    .run()
    .await
}
