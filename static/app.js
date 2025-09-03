// LogisTrans Frontend Application
let currentUser = null;
let authToken = null;
let map = null;
let vehicleMarkers = {};

// Initialize application
document.addEventListener('DOMContentLoaded', function() {
    setupEventListeners();
    checkAuthStatus();
});

function setupEventListeners() {
    // Login form submission
    document.getElementById('loginFormElement').addEventListener('submit', handleLogin);
}

async function handleLogin(event) {
    event.preventDefault();
    
    const username = document.getElementById('username').value;
    const password = document.getElementById('password').value;
    
    try {
        const response = await fetch('/api/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ username, password })
        });
        
        if (response.ok) {
            const data = await response.json();
            authToken = data.token;
            currentUser = data.user;
            
            localStorage.setItem('authToken', authToken);
            localStorage.setItem('user', JSON.stringify(currentUser));
            
            showMainApp();
            loadDashboardData();
        } else {
            const error = await response.text();
            alert('Login failed: ' + error);
        }
    } catch (error) {
        console.error('Login error:', error);
        alert('Login failed. Please try again.');
    }
}

function checkAuthStatus() {
    const token = localStorage.getItem('authToken');
    const user = localStorage.getItem('user');
    
    if (token && user) {
        authToken = token;
        currentUser = JSON.parse(user);
        showMainApp();
        loadDashboardData();
    }
}

function showMainApp() {
    document.getElementById('loginForm').style.display = 'none';
    document.getElementById('mainApp').style.display = 'block';
    
    // Show user info
    document.getElementById('userInfo').textContent = `${currentUser.username} (${currentUser.role})`;
    document.getElementById('authNav').style.display = 'block';
    
    // Show admin features
    if (currentUser.role === 'Admin') {
        document.getElementById('usersNav').style.display = 'block';
    }
}

function logout() {
    localStorage.removeItem('authToken');
    localStorage.removeItem('user');
    authToken = null;
    currentUser = null;
    
    document.getElementById('mainApp').style.display = 'none';
    document.getElementById('loginForm').style.display = 'block';
    document.getElementById('username').value = '';
    document.getElementById('password').value = '';
}

function showSection(sectionName) {
    // Hide all sections
    document.querySelectorAll('.content-section').forEach(section => {
        section.style.display = 'none';
    });
    
    // Show selected section
    document.getElementById(sectionName).style.display = 'block';
    
    // Update navigation active state
    document.querySelectorAll('.list-group-item').forEach(item => {
        item.classList.remove('active');
    });
    event.target.classList.add('active');
    
    // Load section data
    switch(sectionName) {
        case 'dashboard':
            loadDashboardData();
            break;
        case 'vehicles':
            loadVehicles();
            break;
        case 'cargo':
            loadCargo();
            break;
        case 'routes':
            loadRoutes();
            break;
        case 'tracking':
            initMap();
            break;
        case 'users':
            loadUsers();
            break;
    }
}

async function loadDashboardData() {
    try {
        const [vehicles, routes, cargo, users] = await Promise.all([
            fetchWithAuth('/api/vehicles'),
            fetchWithAuth('/api/routes'),
            fetchWithAuth('/api/cargo'),
            fetchWithAuth('/api/users')
        ]);
        
        document.getElementById('totalVehicles').textContent = vehicles.length || 0;
        document.getElementById('activeRoutes').textContent = routes.filter(r => r.status === 'inprogress').length || 0;
        document.getElementById('pendingCargo').textContent = cargo.filter(c => c.status === 'pending').length || 0;
        document.getElementById('totalUsers').textContent = users.length || 0;
    } catch (error) {
        console.error('Error loading dashboard data:', error);
    }
}

async function loadVehicles() {
    try {
        const vehicles = await fetchWithAuth('/api/vehicles');
        displayVehicles(vehicles);
    } catch (error) {
        console.error('Error loading vehicles:', error);
    }
}

function displayVehicles(vehicles) {
    const container = document.getElementById('vehiclesList');
    
    if (vehicles.length === 0) {
        container.innerHTML = '<p class="text-muted">No vehicles registered yet.</p>';
        return;
    }
    
    const table = `
        <table class="table">
            <thead>
                <tr>
                    <th>License Plate</th>
                    <th>Make/Model</th>
                    <th>Year</th>
                    <th>Capacity</th>
                    <th>Fuel Type</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
                ${vehicles.map(vehicle => `
                    <tr>
                        <td>${vehicle.license_plate}</td>
                        <td>${vehicle.make} ${vehicle.model}</td>
                        <td>${vehicle.year}</td>
                        <td>${vehicle.capacity} kg</td>
                        <td>${vehicle.fuel_type}</td>
                        <td><span class="badge bg-${getStatusColor(vehicle.status)}">${vehicle.status}</span></td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;
    
    container.innerHTML = table;
}

async function loadCargo() {
    try {
        const cargo = await fetchWithAuth('/api/cargo');
        displayCargo(cargo);
    } catch (error) {
        console.error('Error loading cargo:', error);
    }
}

function displayCargo(cargo) {
    const container = document.getElementById('cargoList');
    
    if (cargo.length === 0) {
        container.innerHTML = '<p class="text-muted">No cargo registered yet.</p>';
        return;
    }
    
    const table = `
        <table class="table">
            <thead>
                <tr>
                    <th>Description</th>
                    <th>Weight</th>
                    <th>Volume</th>
                    <th>Type</th>
                    <th>Priority</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
                ${cargo.map(item => `
                    <tr>
                        <td>${item.description}</td>
                        <td>${item.weight} kg</td>
                        <td>${item.volume} m³</td>
                        <td>${item.cargo_type}</td>
                        <td><span class="badge bg-${getPriorityColor(item.priority)}">${item.priority}</span></td>
                        <td><span class="badge bg-${getStatusColor(item.status)}">${item.status}</span></td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;
    
    container.innerHTML = table;
}

async function loadRoutes() {
    try {
        const routes = await fetchWithAuth('/api/routes');
        displayRoutes(routes);
    } catch (error) {
        console.error('Error loading routes:', error);
    }
}

function displayRoutes(routes) {
    const container = document.getElementById('routesList');
    
    if (routes.length === 0) {
        container.innerHTML = '<p class="text-muted">No routes created yet.</p>';
        return;
    }
    
    const table = `
        <table class="table">
            <thead>
                <tr>
                    <th>Source</th>
                    <th>Destination</th>
                    <th>Distance</th>
                    <th>Duration</th>
                    <th>Status</th>
                </tr>
            </thead>
            <tbody>
                ${routes.map(route => `
                    <tr>
                        <td>${route.source_address}</td>
                        <td>${route.destination_address}</td>
                        <td>${route.estimated_distance ? route.estimated_distance.toFixed(2) + ' km' : 'N/A'}</td>
                        <td>${route.estimated_duration ? Math.round(route.estimated_duration / 60) + ' min' : 'N/A'}</td>
                        <td><span class="badge bg-${getStatusColor(route.status)}">${route.status}</span></td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;
    
    container.innerHTML = table;
}

async function loadUsers() {
    try {
        const users = await fetchWithAuth('/api/users');
        displayUsers(users);
    } catch (error) {
        console.error('Error loading users:', error);
    }
}

function displayUsers(users) {
    const container = document.getElementById('usersList');
    
    if (users.length === 0) {
        container.innerHTML = '<p class="text-muted">No users found.</p>';
        return;
    }
    
    const table = `
        <table class="table">
            <thead>
                <tr>
                    <th>Username</th>
                    <th>Email</th>
                    <th>Role</th>
                    <th>Created</th>
                </tr>
            </thead>
            <tbody>
                ${users.map(user => `
                    <tr>
                        <td>${user.username}</td>
                        <td>${user.email}</td>
                        <td><span class="badge bg-info">${user.role}</span></td>
                        <td>${new Date(user.created_at).toLocaleDateString()}</td>
                    </tr>
                `).join('')}
            </tbody>
        </table>
    `;
    
    container.innerHTML = table;
}

// Modal functions
function showVehicleForm() {
    const modal = new bootstrap.Modal(document.getElementById('vehicleModal'));
    modal.show();
}

function showCargoForm() {
    const modal = new bootstrap.Modal(document.getElementById('cargoModal'));
    modal.show();
}

function showRouteForm() {
    const modal = new bootstrap.Modal(document.getElementById('routeModal'));
    modal.show();
    loadFormData();
}

function showUserForm() {
    const modal = new bootstrap.Modal(document.getElementById('userModal'));
    modal.show();
}

async function saveVehicle() {
    const formData = {
        license_plate: document.getElementById('licensePlate').value,
        make: document.getElementById('make').value,
        model: document.getElementById('model').value,
        year: parseInt(document.getElementById('year').value),
        capacity: parseFloat(document.getElementById('capacity').value),
        fuel_type: document.getElementById('fuelType').value
    };
    
    try {
        const response = await fetchWithAuth('/api/vehicles', {
            method: 'POST',
            body: JSON.stringify(formData)
        });
        
        if (response) {
            bootstrap.Modal.getInstance(document.getElementById('vehicleModal')).hide();
            document.getElementById('vehicleForm').reset();
            loadVehicles();
            loadDashboardData();
            alert('Vehicle saved successfully!');
        }
    } catch (error) {
        console.error('Error saving vehicle:', error);
        alert('Failed to save vehicle. Please try again.');
    }
}

async function saveCargo() {
    const formData = {
        description: document.getElementById('cargoDescription').value,
        weight: parseFloat(document.getElementById('cargoWeight').value),
        volume: parseFloat(document.getElementById('cargoVolume').value),
        cargo_type: document.getElementById('cargoType').value,
        priority: document.getElementById('cargoPriority').value,
        shipper_id: currentUser.id,
        consignee_id: currentUser.id // For demo purposes
    };
    
    try {
        const response = await fetchWithAuth('/api/cargo', {
            method: 'POST',
            body: JSON.stringify(formData)
        });
        
        if (response) {
            bootstrap.Modal.getInstance(document.getElementById('cargoModal')).hide();
            document.getElementById('cargoForm').reset();
            loadCargo();
            loadDashboardData();
            alert('Cargo saved successfully!');
        }
    } catch (error) {
        console.error('Error saving cargo:', error);
        alert('Failed to save cargo. Please try again.');
    }
}

async function saveRoute() {
    const formData = {
        source_address: document.getElementById('sourceAddress').value,
        source_lat: parseFloat(document.getElementById('sourceLat').value),
        source_lng: parseFloat(document.getElementById('sourceLng').value),
        destination_address: document.getElementById('destAddress').value,
        destination_lat: parseFloat(document.getElementById('destLat').value),
        destination_lng: parseFloat(document.getElementById('destLng').value),
        vehicle_id: document.getElementById('routeVehicle').value,
        driver_id: document.getElementById('routeDriver').value,
        cargo_id: document.getElementById('routeCargo').value
    };
    
    try {
        const response = await fetchWithAuth('/api/routes', {
            method: 'POST',
            body: JSON.stringify(formData)
        });
        
        if (response) {
            bootstrap.Modal.getInstance(document.getElementById('routeModal')).hide();
            document.getElementById('routeForm').reset();
            loadRoutes();
            loadDashboardData();
            alert('Route created successfully!');
        }
    } catch (error) {
        console.error('Error creating route:', error);
        alert('Failed to create route. Please try again.');
    }
}

async function saveUser() {
    const formData = {
        username: document.getElementById('userUsername').value,
        email: document.getElementById('userEmail').value,
        password: document.getElementById('userPassword').value,
        role: document.getElementById('userRole').value
    };
    
    try {
        const response = await fetchWithAuth('/api/users', {
            method: 'POST',
            body: JSON.stringify(formData)
        });
        
        if (response) {
            bootstrap.Modal.getInstance(document.getElementById('userModal')).hide();
            document.getElementById('userForm').reset();
            loadUsers();
            loadDashboardData();
            alert('User saved successfully!');
        }
    } catch (error) {
        console.error('Error saving user:', error);
        alert('Failed to save user. Please try again.');
    }
}

async function loadFormData() {
    try {
        const [vehicles, users, cargo] = await Promise.all([
            fetchWithAuth('/api/vehicles'),
            fetchWithAuth('/api/users'),
            fetchWithAuth('/api/cargo')
        ]);
        
        // Populate vehicle dropdown
        const vehicleSelect = document.getElementById('routeVehicle');
        vehicleSelect.innerHTML = '<option value="">Select Vehicle</option>' +
            vehicles.map(v => `<option value="${v.id}">${v.license_plate} - ${v.make} ${v.model}</option>`).join('');
        
        // Populate driver dropdown (only drivers)
        const driverSelect = document.getElementById('routeDriver');
        driverSelect.innerHTML = '<option value="">Select Driver</option>' +
            users.filter(u => u.role === 'Driver').map(u => `<option value="${u.id}">${u.username}</option>`).join('');
        
        // Populate cargo dropdown
        const cargoSelect = document.getElementById('routeCargo');
        cargoSelect.innerHTML = '<option value="">Select Cargo</option>' +
            cargo.filter(c => c.status === 'pending').map(c => `<option value="${c.id}">${c.description}</option>`).join('');
    } catch (error) {
        console.error('Error loading form data:', error);
    }
}

// Map functionality
function initMap() {
    if (!map) {
        map = L.map('map').setView([51.505, -0.09], 13);
        
        // Add OpenStreetMap tiles
        L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
            attribution: '© OpenStreetMap contributors'
        }).addTo(map);
        
        // Add some sample vehicle markers for demonstration
        addSampleVehicles();
    }
}

function addSampleVehicles() {
    const sampleVehicles = [
        { id: '1', lat: 51.505, lng: -0.09, name: 'Truck 001' },
        { id: '2', lat: 51.51, lng: -0.1, name: 'Truck 002' },
        { id: '3', lat: 51.49, lng: -0.08, name: 'Truck 003' }
    ];
    
    sampleVehicles.forEach(vehicle => {
        const marker = L.marker([vehicle.lat, vehicle.lng])
            .bindPopup(vehicle.name)
            .addTo(map);
        
        vehicleMarkers[vehicle.id] = marker;
    });
}

// Utility functions
async function fetchWithAuth(url, options = {}) {
    if (!authToken) {
        throw new Error('No authentication token');
    }
    
    const defaultOptions = {
        headers: {
            'Authorization': `Bearer ${authToken}`,
            'Content-Type': 'application/json',
        }
    };
    
    const finalOptions = { ...defaultOptions, ...options };
    
    try {
        const response = await fetch(url, finalOptions);
        
        if (response.status === 401) {
            // Token expired or invalid
            logout();
            return null;
        }
        
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        
        return await response.json();
    } catch (error) {
        console.error('API request failed:', error);
        throw error;
    }
}

function getStatusColor(status) {
    switch (status) {
        case 'available':
        case 'completed':
        case 'delivered':
            return 'success';
        case 'inuse':
        case 'inprogress':
        case 'assigned':
        case 'intransit':
            return 'warning';
        case 'maintenance':
        case 'outofservice':
        case 'cancelled':
            return 'danger';
        case 'planned':
        case 'pending':
            return 'info';
        default:
            return 'secondary';
    }
}

function getPriorityColor(priority) {
    switch (priority) {
        case 'urgent':
            return 'danger';
        case 'high':
            return 'warning';
        case 'medium':
            return 'info';
        case 'low':
            return 'success';
        default:
            return 'secondary';
    }
}

// WebSocket connection for real-time updates
function connectWebSocket() {
    const ws = new WebSocket(`ws://${window.location.host}/ws`);
    
    ws.onopen = function() {
        console.log('WebSocket connected');
    };
    
    ws.onmessage = function(event) {
        try {
            const data = JSON.parse(event.data);
            if (data.message_type === 'location_update') {
                updateVehicleLocation(data.data);
            }
        } catch (error) {
            console.error('Error parsing WebSocket message:', error);
        }
    };
    
    ws.onclose = function() {
        console.log('WebSocket disconnected');
        // Reconnect after 5 seconds
        setTimeout(connectWebSocket, 5000);
    };
    
    ws.onerror = function(error) {
        console.error('WebSocket error:', error);
    };
}

function updateVehicleLocation(locationData) {
    // Update vehicle marker on map
    if (vehicleMarkers[locationData.vehicle_id]) {
        const marker = vehicleMarkers[locationData.vehicle_id];
        marker.setLatLng([locationData.latitude, locationData.longitude]);
        marker.getPopup().setContent(`Vehicle ${locationData.vehicle_id}<br>Speed: ${locationData.speed} km/h`);
    }
}

// Initialize WebSocket when tracking section is shown
document.addEventListener('DOMContentLoaded', function() {
    // Connect WebSocket when tracking section is shown
    const trackingButton = document.querySelector('button[onclick="showSection(\'tracking\')"]');
    if (trackingButton) {
        trackingButton.addEventListener('click', function() {
            setTimeout(connectWebSocket, 1000); // Connect after map is initialized
        });
    }
});
