use smart::devices::smartsocket::SmartSocket;
use smart::devices::thermometer::Thermometer;
use smart::devices::types::DeviceType;
use smart::house::House;

pub fn main() {
    let mut house = House::new("sweet home");
    println!("{:?}", house);

    house.add_room("bedroom 1");
    house.add_room("kitchen");
    println!("{:?}", house);

    house.add_room("bedroom 2");
    house.remove_room("bedroom 1");
    println!("{:?}", house);

    let bedroom = house.get_room_mut("bedroom 2").unwrap();
    bedroom.add_device(DeviceType::Thermometer(Thermometer::new(
        "thermometer on the wall",
        "",
    )));
    bedroom.add_device(DeviceType::SmartSocket(SmartSocket::new(
        "socket near the bed",
        "",
    )));

    let socket = bedroom.get_socket_mut("socket near the bed").unwrap();
    socket.switch();

    println!("{:?}", house);
}
