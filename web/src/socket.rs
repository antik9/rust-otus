use std::sync::Mutex;

use actix_web::{post, put, web, HttpResponse};
use smart::house;

use crate::errors::JsonError;

#[derive(serde::Deserialize)]
struct ConnectSocketRequest {
    name: String,
    host: String,
}

#[put("/room/{room_name}/socket/connect")]
async fn connect_socket(
    house: web::Data<Mutex<house::House>>,
    room_name: web::Path<String>,
    req: web::Json<ConnectSocketRequest>,
) -> HttpResponse {
    match house.lock().unwrap().get_room_mut(&room_name) {
        None => HttpResponse::NotFound().body(""),
        Some(room) => match room.get_socket_mut(&req.name) {
            None => HttpResponse::NotFound().body(""),
            Some(socket) => match socket.connect(&req.host).await {
                Err(e) => HttpResponse::InternalServerError()
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&JsonError::new(e.to_string())).unwrap()),
                Ok(()) => HttpResponse::Ok().body(""),
            },
        },
    }
}

#[derive(serde::Deserialize)]
struct SwitchSocketRequest {
    name: String,
}

#[post("/room/{room_name}/socket/switch")]
async fn switch_socket(
    house: web::Data<Mutex<house::House>>,
    room_name: web::Path<String>,
    req: web::Json<SwitchSocketRequest>,
) -> HttpResponse {
    match house.lock().unwrap().get_room_mut(&room_name) {
        None => HttpResponse::NotFound().body(""),
        Some(room) => match room.get_socket_mut(&req.name) {
            None => HttpResponse::NotFound().body(""),
            Some(socket) => match socket.switch().await {
                Err(e) => HttpResponse::InternalServerError()
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&JsonError::new(e.to_string())).unwrap()),
                Ok(()) => HttpResponse::Ok().body(""),
            },
        },
    }
}
