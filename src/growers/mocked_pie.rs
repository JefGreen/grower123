use crate::growers::Grower;
// use gpio::{GpioIn, GpioOut};

// let mut gpio23 = gpio::sysfs::SysFsGpioInput::open(23).unwrap();
// let mut gpio24 = gpio::sysfs::SysFsGpioOutput::open(24).unwrap();

pub struct MockedPie<'a> {
    water_port: &'a u8,
    light_port: &'a u8,
    water_port_current_state: bool,
    light_port_current_state: bool,
}

impl<'a> MockedPie<'a> {
    pub fn new(water_port: &'a u8, light_port: &'a u8) -> MockedPie<'a> {
        MockedPie {
            water_port,
            light_port,
            water_port_current_state: false,
            light_port_current_state: false,
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
