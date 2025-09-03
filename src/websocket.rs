use actix::{Actor, StreamHandler, Handler, Message};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationUpdate {
    pub route_id: Uuid,
    pub vehicle_id: Uuid,
    pub driver_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub speed: f64,
    pub heading: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
}

pub struct WebSocketSession {
    pub id: Uuid,
    pub route_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub driver_id: Option<Uuid>,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket session started: {}", self.id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket session stopped: {}", self.id);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(message) = serde_json::from_str::<WebSocketMessage>(&text) {
                    match message.message_type.as_str() {
                        "subscribe_route" => {
                            if let Some(route_id) = message.data.get("route_id") {
                                if let Some(route_uuid) = route_id.as_str().and_then(|s| Uuid::parse_str(s).ok()) {
                                    self.route_id = Some(route_uuid);
                                    println!("Session {} subscribed to route {}", self.id, route_uuid);
                                }
                            }
                        }
                        "subscribe_vehicle" => {
                            if let Some(vehicle_id) = message.data.get("vehicle_id") {
                                if let Some(vehicle_uuid) = vehicle_id.as_str().and_then(|s| Uuid::parse_str(s).ok()) {
                                    self.vehicle_id = Some(vehicle_uuid);
                                    println!("Session {} subscribed to vehicle {}", self.id, vehicle_uuid);
                                }
                            }
                        }
                        "subscribe_driver" => {
                            if let Some(driver_id) = message.data.get("driver_id") {
                                if let Some(driver_uuid) = driver_id.as_str().and_then(|s| Uuid::parse_str(s).ok()) {
                                    self.driver_id = Some(driver_uuid);
                                    println!("Session {} subscribed to driver {}", self.id, driver_uuid);
                                }
                            }
                        }
                        _ => {
                            println!("Unknown message type: {}", message.message_type);
                        }
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(reason)) => ctx.close(reason),
            _ => (),
        }
    }
}

// Global session manager
pub struct SessionManager {
    sessions: Arc<Mutex<HashMap<Uuid, actix::Addr<WebSocketSession>>>>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn add_session(&self, id: Uuid, addr: actix::Addr<WebSocketSession>) {
        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.insert(id, addr);
        }
    }

    pub fn remove_session(&self, id: &Uuid) {
        if let Ok(mut sessions) = self.sessions.lock() {
            sessions.remove(id);
        }
    }

    pub fn broadcast_location_update(&self, update: &LocationUpdate) {
        if let Ok(sessions) = self.sessions.lock() {
            let message = serde_json::to_string(&WebSocketMessage {
                message_type: "location_update".to_string(),
                data: serde_json::to_value(update).unwrap(),
            }).unwrap();

            for (session_id, addr) in sessions.iter() {
                // Send to sessions subscribed to this route, vehicle, or driver
                if let Some(route_id) = update.route_id {
                    if let Some(session) = sessions.get(&session_id) {
                        if let Some(session_route_id) = session.route_id {
                            if session_route_id == route_id {
                                session.do_send(ws::Message::Text(message.clone()));
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Clone for SessionManager {
    fn clone(&self) -> Self {
        Self {
            sessions: Arc::clone(&self.sessions),
        }
    }
}

pub async fn ws_index(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
    session_manager: actix_web::web::Data<SessionManager>,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    let session_id = Uuid::new_v4();
    let session = WebSocketSession {
        id: session_id,
        route_id: None,
        vehicle_id: None,
        driver_id: None,
    };

    let session_manager = session_manager.into_inner().clone();
    
    let resp = ws::start(session, &req, stream)?;
    
    // Add session to manager
    session_manager.add_session(session_id, resp.1.clone());
    
    Ok(resp.0)
}
