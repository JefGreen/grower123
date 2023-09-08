use grower;

pub struct MockedPie {
    water_port: u8,
    light_port: u8,
}

impl MockedPie {
    fn new(water_port: &u8, light_port: &u8) -> RaspberryPie {
        println! {"made it here in Mocked Pie"};
        MockedPie {
            water_port,
            light_port,
        }
    }
}

pub impl Grower for MockedPie {
    fn water(&self) {
        println! {"watering"};
    }

    fn light(&self) {
        println! {"lighting"};
    }
}
