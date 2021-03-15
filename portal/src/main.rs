use {rppal::system::DeviceInfo, tradfri::TradfriConnection};

fn main() {
    let dev_info = DeviceInfo::new().expect("Failed to obtain device info");
    println!("{:?}", dev_info);

    let mut tc = TradfriConnection::connect(
        "192.168.1.100:5684"
            .parse::<std::net::SocketAddrV4>()
            .unwrap(),
        "tradfri-gtk-1615779664",
        "eD8sKFTfEbnUh6kg",
    )
    .unwrap();
    let devices = tc.devices();
    println!("{:#?}", devices);
}
