use crate::growers::Grower;

pub struct MockedPie {
    pub water_port: u8,
    pub light_port: u8,
}

impl Grower for MockedPie {
    fn water(&self) {
        println! {"watering"};
    }

    fn light(&self) {
        println! {"lighting"};
    }
}
