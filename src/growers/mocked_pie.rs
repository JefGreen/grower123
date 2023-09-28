use crate::growers::Grower;

pub struct MockedPie<'a> {
    water_port: &'a u8,
    light_port: &'a u8,
}

impl<'a> MockedPie<'a> {
    pub fn new(water_port: &'a u8, light_port: &'a u8) -> MockedPie<'a> {
        MockedPie {
            water_port,
            light_port,
        }
    }
}

impl<'a> Grower for MockedPie<'a> {
    fn water(&self, output: bool) {
        println!(
            "Turning {} water using port {}",
            if output { "on" } else { "off" },
            self.water_port
        );
    }

    fn mesure_moister(&self) {
        // How would the grower know?
        println!("Check if plant needs water {}", self.water_port);
    }

    fn light(&self, output: bool) {
        println!(
            "Turning lights {} using port {}",
            if output { "on" } else { "off" },
            self.light_port
        );
    }
}
