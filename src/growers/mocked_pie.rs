// use grower;
use crate::growers::Grower;

// #[derive(Debug)]
pub struct MockedPie {
    pub water_port: u8,
    pub light_port: u8,
}

// pub impl MockedPie {
//     fn new(water_port: &u8, light_port: &u8) -> RaspberryPie {
//         println! {"made it here in Mocked Pie"};
//         MockedPie {
//             water_port,
//             light_port,
//         }
//     }
// }

impl Grower for MockedPie {
    fn water(&self) {
        println! {"watering"};
    }

    fn light(&self) {
        println! {"lighting"};
    }
}
