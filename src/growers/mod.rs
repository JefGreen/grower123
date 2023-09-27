pub use self::mocked_pie::MockedPie;
mod mocked_pie;

pub trait Grower {
    // what if it returned the next task instead like stop the water or turn off the lamps
    fn water(&self) -> ();

    fn check_if_need_water(&self) -> ();

    fn light(&self) -> ();
}

// pub struct RaspberryPie {
//     water_port: u8,
//     light_port: u8,
// }

// impl RaspberryPie {
//     pub fn new(water_port: u8, light_port: u8) -> RaspberryPie {
//         println! {"made it here"};
//         RaspberryPie {
//             water_port,
//             light_port,
//         }
//     }
// }

// impl Grower for RaspberryPie {
//     fn water(&self) {}

//     fn light(&self) {}
// }
