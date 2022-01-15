use smart::devices::device::{Summary, Switcher};
use smart::devices::retriable_switcher::Retry;
use smart::devices::smartsocket::SmartSocket;
use smart::devices::thermometer::Thermometer;
use smart::devices::types::DeviceType;
use smart::house::House;
use smart_socket::receiver::DEFAULT_ADDRESS;

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
    bedroom.add_device(DeviceType::Thermometer(Thermometer::new(
        "thermometer on the wall",
        "",
    )))?;

    /*
    bedroom.add_device(DeviceType::SmartSocket(SmartSocket::new(
        "socket near the bed",
        "",
    )))?;
    */

    let mut socket = SmartSocket::new("simple socket", "");
    socket.connect(DEFAULT_ADDRESS).await?;

    let mut retriable = Retry::new(Box::new(socket), 3);
    retriable.switch().await?;

    print_summary(Box::new(house)).await;
    Ok(())
}

async fn print_summary(obj: Box<dyn Summary>) {
    print!("{}", obj.summary().await);
}
