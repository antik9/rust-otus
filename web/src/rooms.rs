use std::sync::Mutex;

use actix_web::{delete, post, put, web, HttpResponse};
use smart::{devices::device::Device, devices::types::DeviceType, house};

use crate::errors::JsonError;

#[derive(serde::Deserialize)]
struct AddRoomRequest {
    name: String,
}

#[post("/room")]
async fn add_room(
    house: web::Data<Mutex<house::House>>,
    req: web::Json<AddRoomRequest>,
) -> HttpResponse {
    match house.lock().unwrap().add_room(&req.name) {
        Err(e) => HttpResponse::BadRequest()
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&JsonError::new(e.to_string())).unwrap()),
        Ok(()) => HttpResponse::Created().body(""),
    }
}

#[derive(serde::Deserialize)]
struct RemoveRoomRequest {
    name: String,
}

#[delete("/room")]
async fn remove_room(
    house: web::Data<Mutex<house::House>>,
    req: web::Json<RemoveRoomRequest>,
) -> HttpResponse {
    match house.lock().unwrap().remove_room(&req.name) {
        Err(e) => HttpResponse::BadRequest()
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&JsonError::new(e.to_string())).unwrap()),
        Ok(()) => HttpResponse::NoContent().body(""),
    }
}

#[derive(serde::Deserialize)]
struct MountReceiverRequest {
    address: String,
}

#[put("/room/{room_name}/receiver")]
async fn mount_receiver(
    house: web::Data<Mutex<house::House>>,
    room_name: web::Path<String>,
    req: web::Json<MountReceiverRequest>,
) -> HttpResponse {
    match house.lock().unwrap().get_room_mut(&room_name) {
        None => HttpResponse::NotFound().body(""),
        Some(room) => match room.mount_receiver(&req.address).await {
            Err(e) => HttpResponse::InternalServerError()
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&JsonError::new(e.to_string())).unwrap()),
            Ok(()) => {
                let mut thermometers: Vec<String> = Vec::new();
                for device in room.get_devices_mut() {
                    if let DeviceType::Thermometer(t) = device {
                        thermometers.push(t.get_name().into());
                    }
                }
                thermometers
                    .iter()
                    .for_each(|t| room.connect_device_to_receiver(t));
                HttpResponse::Ok().body("")
            }
        },
    }
}
