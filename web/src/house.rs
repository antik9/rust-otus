use std::sync::Mutex;

use actix_web::{get, web, HttpResponse};
use smart::devices::device::Device;
use smart::formatter::JsonFormatter;
use smart::house;

#[derive(serde::Serialize)]
struct Room {
    name: String,
    devices: Vec<RoomDevice>,
}

impl Room {
    pub fn new(name: &str, devices: Vec<RoomDevice>) -> Self {
        Self {
            name: name.into(),
            devices,
        }
    }
}

#[derive(serde::Serialize)]
struct RoomDevice {
    name: String,
    description: String,
}

impl RoomDevice {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
        }
    }
}

#[get("/rooms")]
pub async fn get_rooms(house: web::Data<Mutex<house::House>>) -> HttpResponse {
    let mut rooms: Vec<Room> = Vec::new();
    for room in house.lock().unwrap().get_rooms() {
        let devices = room
            .get_devices()
            .map(|d| RoomDevice::new(d.get_name(), d.get_description()))
            .collect();
        rooms.push(Room::new(room.get_name(), devices));
    }
    HttpResponse::Ok().body(serde_json::to_string(&rooms).unwrap())
}

#[get("/report")]
pub async fn get_report(house: web::Data<Mutex<house::House>>) -> HttpResponse {
    HttpResponse::Ok()
        .header("Content-Type", "application/json")
        .body(
            house
                .lock()
                .unwrap()
                .summary_fmt(Box::new(JsonFormatter {}))
                .await,
        )
}
