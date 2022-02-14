use std::sync::Mutex;

use actix_web::{delete, post, web, HttpResponse};
use smart::devices::smartsocket::SmartSocket;
use smart::devices::thermometer::Thermometer;
use smart::devices::types::DeviceType;
use smart::house;

use crate::errors::JsonError;

#[derive(serde::Deserialize)]
struct AddDeviceRequest {
    device_type: String,
    name: String,
    description: String,
}

#[post("/room/{room_name}/device")]
async fn add_device(
    house: web::Data<Mutex<house::House>>,
    room_name: web::Path<String>,
    req: web::Json<AddDeviceRequest>,
) -> HttpResponse {
    match house.lock().unwrap().get_room_mut(room_name.as_str()) {
        None => HttpResponse::NotFound().body(""),
        Some(room) => match req.device_type.as_str() {
            "socket" => match room.add_device(DeviceType::SmartSocket(SmartSocket::new(
                &req.name,
                &req.description,
            ))) {
                Err(e) => HttpResponse::BadRequest()
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&JsonError::new(e.to_string())).unwrap()),
                Ok(()) => HttpResponse::Created().body(""),
            },
            "thermometer" => match room.add_device(DeviceType::Thermometer(Thermometer::new(
                &req.name,
                &req.description,
            ))) {
                Err(e) => HttpResponse::BadRequest()
                    .header("Content-Type", "application/json")
                    .body(serde_json::to_string(&JsonError::new(e.to_string())).unwrap()),
                Ok(()) => HttpResponse::Created().body(""),
            },
            _ => HttpResponse::BadRequest()
                .header("Content-Type", "application/json")
                .body(
                    serde_json::to_string(&JsonError::new("unknown device type".into())).unwrap(),
                ),
        },
    }
}

#[derive(serde::Deserialize)]
struct RemoveDeviceRequest {
    name: String,
}

#[delete("/room/{room_name}/device")]
async fn remove_device(
    house: web::Data<Mutex<house::House>>,
    room_name: web::Path<String>,
    req: web::Json<RemoveDeviceRequest>,
) -> HttpResponse {
    match house.lock().unwrap().get_room_mut(room_name.as_str()) {
        None => HttpResponse::NotFound().body(""),
        Some(room) => match room.remove_device(&req.name) {
            Err(e) => HttpResponse::BadRequest()
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&JsonError::new(e.to_string())).unwrap()),
            Ok(()) => HttpResponse::NoContent().body(""),
        },
    }
}
