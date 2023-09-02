use grower;

pub struct MockedPie {
    water_port: u8,
    light_port: u8,
}

pub impl Grower for MockedPie {
    fn new(water_port: &u8, light_port: &u8) -> RaspberryPie {
        println! {"made it here in Mocked Pie"};
        MockedPie {
            water_port,
            light_port,
        }
    }
}
