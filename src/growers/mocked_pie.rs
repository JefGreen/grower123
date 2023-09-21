use crate::growers::Grower;

pub struct MockedPie<'a> {
    water_port: &'a u8,
    light_port: &'a u8,
}

impl<'a> MockedPie<'a> {
    pub fn new(water_port: &'a u8, light_port: &'a u8) -> MockedPie<'a> {
        println! {"made it here"};
        MockedPie {
            water_port,
            light_port,
        }
    }
}

impl<'a> Grower for MockedPie<'a> {
    fn water(&self) {
        println!("watering using port {}", self.water_port);
    }

    fn light(&self) {
        println!("lighting using port {}", self.light_port);
    }
}
