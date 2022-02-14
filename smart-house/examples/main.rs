use std::time::Duration;

use smart::devices::device::Summary;
use smart::devices::smartsocket::SmartSocket;
use smart::devices::thermometer::Thermometer;
use smart::devices::types::DeviceType;
use smart::formatter::JsonFormatter;
use smart::house::House;
use smart_socket::receiver::DEFAULT_ADDRESS;
use std::thread::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut house = House::new("sweet home");
    println!("{:?}", house);

    house.add_room("bedroom 1")?;
    house.add_room("kitchen")?;
    println!("{:?}", house);

    house.add_room("bedroom 2")?;
    house.remove_room("bedroom 1")?;
    println!("{:?}", house);

    let bedroom = house.get_room_mut("bedroom 2").unwrap();
    bedroom.mount_receiver("127.0.0.1:11701").await?;

    bedroom.add_device(DeviceType::Thermometer(Thermometer::new(
        "thermometer on the wall",
        "",
    )))?;

    bedroom.add_device(DeviceType::SmartSocket(SmartSocket::new(
        "socket near the bed",
        "",
    )))?;

    bedroom.connect_device_to_receiver("thermometer on the wall");

    let socket = bedroom.get_socket_mut("socket near the bed").unwrap();
    socket.connect(DEFAULT_ADDRESS).await?;
    socket.switch().await?;

    sleep(Duration::from_millis(200));
    println!("{}", house.summary_fmt(Box::new(JsonFormatter {})).await);
    print_summary(Box::new(house)).await;

    Ok(())
}

async fn print_summary(obj: Box<dyn Summary>) {
    println!("{}", obj.summary().await);
}
