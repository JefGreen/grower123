use crate::growers::Grower;

pub struct MockedPie {
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
        println!("watering using port {}", self.water_port);
    }

    fn light(&self) {
        println!("lighting using port {}", self.light_port);
    }
}
