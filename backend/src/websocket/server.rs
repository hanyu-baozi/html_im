use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix::{Actor, ActorContext, StreamHandler, AsyncContext, ActorFutureExt, ContextFutureSpawner, Handler};
use std::sync::{Arc, Mutex};
use serde_json::Value;
use log::info;
use sea_orm::{DatabaseConnection, EntityTrait, Set, ActiveModelTrait};
use crate::websocket::ConnectionManager;
use crate::websocket::manager::ServerMessage;
use crate::models::user;

pub struct MyWebSocket {
    user_id: String,
    manager: Arc<Mutex<ConnectionManager>>,
    db: web::Data<DatabaseConnection>,
    addr: Option<actix::prelude::Recipient<ServerMessage>>,
}

impl MyWebSocket {
    pub fn new(user_id: String, manager: Arc<Mutex<ConnectionManager>>, db: web::Data<DatabaseConnection>) -> Self {
        Self { user_id, manager, db, addr: None }
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection established for user: {}", self.user_id);
        
        let addr = ctx.address().recipient();
        self.addr = Some(addr.clone());
        
        {
            let mut manager = self.manager.lock().unwrap();
            manager.connect(self.user_id.clone(), addr.clone());
        }

        // Update user status to online in database
        let user_id_clone = self.user_id.clone();
        let db_clone = self.db.clone();
        actix::fut::wrap_future::<_, Self>(async move {
            if let Ok(Some(user_model)) = user::Entity::find_by_id(&user_id_clone).one(&**db_clone).await {
                let mut active_user: user::ActiveModel = user_model.into();
                active_user.status = Set("online".to_string());
                if let Err(e) = active_user.update(&**db_clone).await {
                    log::error!("Failed to update user status to online: {:?}", e);
                }
            }
        })
        .map(|_, _, _| ())
        .wait(ctx);

        // Broadcast online status update
        let status_msg = serde_json::json!({
            "type": "status_update",
            "user_id": self.user_id,
            "status": "online"
        }).to_string();
        
        {
            let mut manager = self.manager.lock().unwrap();
            manager.broadcast_message(status_msg);
        }
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::prelude::Running {
        if let Some(addr) = &self.addr {
            let mut manager = self.manager.lock().unwrap();
            manager.disconnect(&self.user_id, addr);
        }
        actix::prelude::Running::Stop
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        info!("WebSocket connection closed for user: {}", self.user_id);
        
        // Update user status to offline in database
        let user_id_clone = self.user_id.clone();
        let db_clone = self.db.clone();
        actix::fut::wrap_future::<_, Self>(async move {
            if let Ok(Some(user_model)) = user::Entity::find_by_id(&user_id_clone).one(&**db_clone).await {
                let mut active_user: user::ActiveModel = user_model.into();
                active_user.status = Set("offline".to_string());
                if let Err(e) = active_user.update(&**db_clone).await {
                    log::error!("Failed to update user status to offline: {:?}", e);
                }
            }
        })
        .map(|_, _, _| ())
        .wait(ctx);

        // Broadcast offline status update
        let status_msg = serde_json::json!({
            "type": "status_update",
            "user_id": self.user_id.clone(),
            "status": "offline"
        }).to_string();
        
        {
            let mut manager = self.manager.lock().unwrap();
            manager.broadcast_message(status_msg);
        }
    }
}

impl Handler<ServerMessage> for MyWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ServerMessage, ctx: &mut Self::Context) {
        ctx.text(msg.message);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {}
            Ok(ws::Message::Text(text)) => {
                if let Ok(json) = serde_json::from_str::<Value>(&text) {
                    info!("Received message: {:?}", json);
                    self.handle_message(json, ctx);
                }
            }
            Ok(ws::Message::Close(reason)) => {
                info!("WebSocket connection closed by client: {:?}", reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}

impl MyWebSocket {
    fn handle_message(&mut self, msg: Value, ctx: &mut ws::WebsocketContext<Self>) {
        let msg_type = msg.get("type").and_then(|v| v.as_str()).unwrap_or("unknown");
        
        match msg_type {
            "ping" => {
                let pong_msg = serde_json::json!({"type": "pong"}).to_string();
                ctx.text(pong_msg);
            }
            "message" => {
                info!("Message received - broadcasting not implemented yet");
            }
            _ => {
                info!("Unknown message type: {}", msg_type);
            }
        }
    }
}

pub async fn websocket_route(
    req: HttpRequest,
    stream: web::Payload,
    manager: web::Data<Arc<Mutex<ConnectionManager>>>,
    db: web::Data<DatabaseConnection>,
) -> Result<HttpResponse, actix_web::Error> {
    let query = req.query_string();
    let user_id = query
        .split('&')
        .find(|s| s.starts_with("user_id="))
        .and_then(|s| s.strip_prefix("user_id="))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "anonymous".to_string());
    
    let ws = MyWebSocket::new(user_id, manager.get_ref().clone(), db);
    ws::start(ws, &req, stream)
}
