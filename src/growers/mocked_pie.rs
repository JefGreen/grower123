use crate::growers::Grower;

struct MockedPie {
    water_port: u8,
    light_port: u8,
}

impl MockedPie {
    pub fn new(water_port: u8, light_port: u8) -> MockedPie {
        println! {"made it here"};
        MockedPie {
            water_port,
            light_port,
        }
    }
}

impl Grower for MockedPie {
    fn water(&self) {
        println! {"watering"};
    }

    fn light(&self) {
        println! {"lighting"};
    }
}
