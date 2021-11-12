pub fn main() {
    let mut house = smart::House::new("sweet home".to_owned());
    println!("{:?}", house);

    house
        .add_room("bedroom 1".to_owned())
        .add_room("kitchen".to_owned());
    let rooms = house.get_rooms();
    println!("{:?}", rooms);
    println!("{:?}", house);

    house
        .add_room("bedroom 2".to_owned())
        .remove_room("bedroom 1".to_owned());
    let rooms = house.get_rooms();
    println!("{:?}", rooms);

    let bedroom = house.get_room("bedroom 2".to_owned()).unwrap();
    bedroom.add_device(
        smart::DeviceType::Thermometer(smart::Thermometer::new()),
        "thermometer on the wall".to_owned(),
    );
    bedroom.add_device(
        smart::DeviceType::SmartSocket(smart::SmartSocket::new()),
        "socket near the bed".to_owned(),
    );

    let socket = bedroom
        .get_socket("socket near the bed".to_owned())
        .unwrap();
    socket.switch();

    for info in house.get_info() {
        println!("{}", info.summary());
    }
}
