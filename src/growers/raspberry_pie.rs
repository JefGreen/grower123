pub struct RaspberryPie {
    water_port: u8,
    light_port: u8,
}

impl Grower for RaspberryPie {
    fn new(water_port: &u8, light_port: &u8) -> RaspberryPie {
        println!{"made it here"};
        RaspberryPie {water_port, light_port}
    }
}
