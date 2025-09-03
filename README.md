# LogisTrans - Logistics & Transport Management System

A comprehensive logistics and transport management application built with Rust (Actix-web) backend and modern web frontend with OpenStreetMap integration for real-time vehicle tracking.

## Features

### 🚛 **Vehicle Management**
- Register and manage fleet vehicles
- Track vehicle specifications (make, model, year, capacity, fuel type)
- Monitor vehicle status (available, in-use, maintenance, out-of-service)

### 📦 **Cargo Management**
- Create and track shipments
- Set cargo priorities (low, medium, high, urgent)
- Monitor cargo status throughout delivery process

### 🗺️ **Route Planning**
- Plan delivery routes with source and destination coordinates
- Automatic distance and duration calculation
- Route status tracking (planned, in-progress, completed, cancelled)

### 📍 **Real-time Tracking**
- Live vehicle location tracking using OpenStreetMap
- WebSocket-based real-time updates
- Historical route tracking data

### 👥 **User Management**
- Role-based access control (Admin, Manager, Driver, Dispatcher)
- Secure JWT authentication
- Admin-only user creation and management

### 📊 **Dashboard & Analytics**
- Real-time system overview
- Vehicle, route, and cargo statistics
- Performance metrics and status monitoring

## Technology Stack

### Backend
- **Rust** - High-performance systems programming language
- **Actix-web** - Fast, powerful web framework
- **SQLx** - Async SQL toolkit with compile-time checked queries
- **PostgreSQL** - Robust, open-source database
- **JWT** - Secure authentication tokens
- **WebSocket** - Real-time communication

### Frontend
- **HTML5/CSS3** - Modern, responsive web interface
- **JavaScript (ES6+)** - Dynamic frontend functionality
- **Bootstrap 5** - Professional UI components
- **Leaflet.js** - Interactive maps with OpenStreetMap
- **Font Awesome** - Beautiful icons

## Prerequisites

- **Rust** (1.70+)
- **PostgreSQL** (12+)
- **Cargo** (Rust package manager)

## Installation & Setup

### 1. Clone the Repository
```bash
git clone <repository-url>
cd logistrans
```

### 2. Install Dependencies
```bash
cargo build
```

### 3. Database Setup
Create a PostgreSQL database and set environment variables:

```bash
# Create database
createdb logistrans

# Set environment variables (create .env file)
echo "DATABASE_URL=postgresql://username:password@localhost/logistrans" > .env
echo "JWT_SECRET=your-secret-key-change-in-production" >> .env
echo "PORT=8080" >> .env
echo "HOST=127.0.0.1" >> .env
```

### 4. Run the Application
```bash
cargo run
```

The application will be available at `http://localhost:8080`

## Default Login

- **Username:** `admin`
- **Password:** `admin123`

## API Endpoints

### Authentication
- `POST /api/login` - User login
- `POST /api/register` - User registration

### Users
- `POST /api/users` - Create new user (Admin only)
- `GET /api/users` - Get all users (Admin only)

### Vehicles
- `POST /api/vehicles` - Register new vehicle
- `GET /api/vehicles` - Get all vehicles

### Cargo
- `POST /api/cargo` - Create new cargo
- `GET /api/cargo` - Get all cargo

### Routes
- `POST /api/routes` - Create new route
- `GET /api/routes` - Get all routes

### Tracking
- `POST /api/tracking/location` - Update vehicle location
- `GET /api/tracking/location/{route_id}` - Get current location
- `GET /api/tracking/history/{route_id}` - Get tracking history

### WebSocket
- `WS /ws` - Real-time location updates

## Database Schema

The application automatically creates the following database structure:

- **users** - User accounts and roles
- **vehicles** - Fleet vehicle information
- **cargo** - Shipment details
- **routes** - Delivery route planning
- **locations** - Real-time tracking data

## Usage Guide

### 1. **Login & Access**
- Access the application at `http://localhost:8080`
- Login with admin credentials
- Navigate through different sections using the sidebar

### 2. **Vehicle Management**
- Go to Vehicles section
- Click "Add Vehicle" to register new fleet vehicles
- View all registered vehicles in a table format

### 3. **Cargo Management**
- Navigate to Cargo section
- Create new shipments with detailed specifications
- Set priority levels and track status

### 4. **Route Planning**
- Access Routes section
- Create delivery routes with source/destination coordinates
- Assign vehicles, drivers, and cargo to routes

### 5. **Live Tracking**
- Go to Live Tracking section
- View real-time vehicle locations on OpenStreetMap
- Monitor delivery progress

### 6. **User Management** (Admin only)
- Access Users section
- Create new user accounts with appropriate roles
- Manage system access permissions

## Real-time Features

### WebSocket Integration
- Automatic connection when tracking section is active
- Real-time location updates from vehicles
- Live map marker updates

### OpenStreetMap Integration
- Free, open-source mapping solution
- Interactive vehicle markers
- Route visualization capabilities

## Security Features

- **JWT Authentication** - Secure token-based authentication
- **Role-based Access Control** - Different permissions for different user types
- **Password Hashing** - Bcrypt password security
- **CORS Support** - Cross-origin resource sharing configuration

## Development

### Project Structure
```
logistrans/
├── src/
│   ├── main.rs          # Application entry point
│   ├── models.rs        # Data structures and models
│   ├── database.rs      # Database connection and setup
│   ├── auth.rs          # Authentication and security
│   ├── handlers/        # API endpoint handlers
│   ├── websocket.rs     # Real-time communication
│   └── config.rs        # Configuration management
├── static/              # Frontend assets
│   ├── index.html       # Main application page
│   ├── styles.css       # Custom styling
│   └── app.js          # Frontend JavaScript
├── Cargo.toml           # Rust dependencies
└── README.md            # This file
```

### Adding New Features
1. Define data models in `src/models.rs`
2. Create database tables in `src/database.rs`
3. Implement API handlers in `src/handlers/`
4. Add frontend components in `static/`
5. Update routing in `src/main.rs`

## Deployment

### Production Considerations
- Change default JWT secret
- Use environment variables for configuration
- Set up proper database credentials
- Configure reverse proxy (nginx/Apache)
- Enable HTTPS
- Set up monitoring and logging

### Docker Support
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y postgresql-client
COPY --from=builder /app/target/release/logistrans /usr/local/bin/
EXPOSE 8080
CMD ["logistrans"]
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Support

For support and questions:
- Create an issue in the repository
- Check the documentation
- Review the code examples

## Roadmap

- [ ] Mobile application support
- [ ] Advanced analytics and reporting
- [ ] Integration with external logistics APIs
- [ ] Multi-tenant support
- [ ] Advanced route optimization algorithms
- [ ] Fleet maintenance scheduling
- [ ] Fuel consumption tracking
- [ ] Driver performance metrics

---

**LogisTrans** - Streamlining logistics operations with modern technology and real-time insights.
