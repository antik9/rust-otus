pub fn main() {
    let mut house = smart::House::new("sweet home");
    println!("{:?}", house);

    house.add_room("bedroom 1");
    house.add_room("kitchen");
    println!("{:?}", house);

    house.add_room("bedroom 2");
    house.remove_room("bedroom 1");
    println!("{:?}", house);

    let bedroom = house.get_room_mut("bedroom 2").unwrap();
    bedroom.add_device(smart::DeviceType::Thermometer(smart::Thermometer::new(
        "thermometer on the wall",
        "",
    )));
    bedroom.add_device(smart::DeviceType::SmartSocket(smart::SmartSocket::new(
        "socket near the bed",
        "",
    )));

    let socket = bedroom.get_socket_mut("socket near the bed").unwrap();
    socket.switch();

    print!("{}", house.get_report().summary());
}
