use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let house = web::Data::new(Mutex::new(smart::house::House::new("home")));
    HttpServer::new(move || {
        App::new()
            .app_data(house.clone())
            .service(::web::house::get_rooms)
            .service(::web::house::get_report)
            .service(::web::rooms::add_room)
            .service(::web::rooms::remove_room)
            .service(::web::rooms::mount_receiver)
            .service(::web::devices::add_device)
            .service(::web::devices::remove_device)
            .service(::web::socket::connect_socket)
            .service(::web::socket::switch_socket)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
