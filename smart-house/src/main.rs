pub fn main() {
    let mut house = smart::House::new("sweet home");
    println!("{:?}", house);

    house.add_room("bedroom 1").add_room("kitchen");
    let rooms = house.get_rooms();
    println!("{:?}", rooms);
    println!("{:?}", house);

    house.add_room("bedroom 2").remove_room("bedroom 1");
    let rooms = house.get_rooms();
    println!("{:?}", rooms);

    let bedroom = house.get_room("bedroom 2").unwrap();
    bedroom.add_device(
        smart::DeviceType::Thermometer(smart::Thermometer::new()),
        "thermometer on the wall",
    );
    bedroom.add_device(
        smart::DeviceType::SmartSocket(smart::SmartSocket::new()),
        "socket near the bed",
    );

    let socket = bedroom.get_socket("socket near the bed").unwrap();
    socket.switch();

    for info in house.get_info() {
        println!("{}", info.summary());
    }
}
