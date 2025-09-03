use sqlx::{PgPool, PgPoolOptions, postgres::PgPoolOptions as PgPoolOptionsPostgres};
use log::info;
use std::time::Duration;

pub type DbPool = PgPool;

pub async fn create_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .max_lifetime(Duration::from_secs(30))
        .connect(database_url)
        .await
}

pub async fn init_database(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Initializing database...");
    
    let pool = create_pool(database_url).await?;
    
    // Create custom types
    sqlx::query(
        r#"
        DO $$ BEGIN
            CREATE TYPE user_role AS ENUM ('admin', 'manager', 'driver', 'dispatcher');
        EXCEPTION
            WHEN duplicate_object THEN null;
        END $$;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        DO $$ BEGIN
            CREATE TYPE vehicle_status AS ENUM ('available', 'inuse', 'maintenance', 'outofservice');
        EXCEPTION
            WHEN duplicate_object THEN null;
        END $$;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        DO $$ BEGIN
            CREATE TYPE cargo_priority AS ENUM ('low', 'medium', 'high', 'urgent');
        EXCEPTION
            WHEN duplicate_object THEN null;
        END $$;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        DO $$ BEGIN
            CREATE TYPE cargo_status AS ENUM ('pending', 'assigned', 'intransit', 'delivered', 'cancelled');
        EXCEPTION
            WHEN duplicate_object THEN null;
        END $$;
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        DO $$ BEGIN
            CREATE TYPE route_status AS ENUM ('planned', 'inprogress', 'completed', 'cancelled');
        EXCEPTION
            WHEN duplicate_object THEN null;
        END $$;
        "#
    ).execute(&pool).await?;

    // Create tables
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(50) UNIQUE NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            role user_role NOT NULL,
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS vehicles (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            license_plate VARCHAR(20) UNIQUE NOT NULL,
            make VARCHAR(50) NOT NULL,
            model VARCHAR(50) NOT NULL,
            year INTEGER NOT NULL,
            capacity DOUBLE PRECISION NOT NULL,
            fuel_type VARCHAR(20) NOT NULL,
            status vehicle_status DEFAULT 'available',
            driver_id UUID REFERENCES users(id),
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS cargo (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            description TEXT NOT NULL,
            weight DOUBLE PRECISION NOT NULL,
            volume DOUBLE PRECISION NOT NULL,
            cargo_type VARCHAR(50) NOT NULL,
            priority cargo_priority DEFAULT 'medium',
            status cargo_status DEFAULT 'pending',
            shipper_id UUID REFERENCES users(id),
            consignee_id UUID REFERENCES users(id),
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS routes (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            source_address TEXT NOT NULL,
            source_lat DOUBLE PRECISION NOT NULL,
            source_lng DOUBLE PRECISION NOT NULL,
            destination_address TEXT NOT NULL,
            destination_lat DOUBLE PRECISION NOT NULL,
            destination_lng DOUBLE PRECISION NOT NULL,
            estimated_distance DOUBLE PRECISION,
            estimated_duration INTEGER,
            vehicle_id UUID REFERENCES vehicles(id),
            driver_id UUID REFERENCES users(id),
            cargo_id UUID REFERENCES cargo(id),
            status route_status DEFAULT 'planned',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#
    ).execute(&pool).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS locations (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            route_id UUID REFERENCES routes(id),
            vehicle_id UUID REFERENCES vehicles(id),
            driver_id UUID REFERENCES users(id),
            latitude DOUBLE PRECISION NOT NULL,
            longitude DOUBLE PRECISION NOT NULL,
            speed DOUBLE PRECISION DEFAULT 0.0,
            heading DOUBLE PRECISION DEFAULT 0.0,
            timestamp TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        );
        "#
    ).execute(&pool).await?;

    // Create indexes
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_locations_route_id ON locations(route_id);"
    ).execute(&pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_locations_timestamp ON locations(timestamp);"
    ).execute(&pool).await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_routes_status ON routes(status);"
    ).execute(&pool).await?;

    // Create admin user if it doesn't exist
    let admin_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM users WHERE role = 'admin')"
    ).fetch_one(&pool).await?;

    if !admin_exists {
        info!("Creating default admin user...");
        let password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)?;
        
        sqlx::query(
            r#"
            INSERT INTO users (username, email, password_hash, role)
            VALUES ('admin', 'admin@logistrans.com', $1, 'admin')
            "#
        ).bind(&password_hash).execute(&pool).await?;
        
        info!("Default admin user created: admin/admin123");
    }

    info!("Database initialization completed successfully");
    Ok(())
}
