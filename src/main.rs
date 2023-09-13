use growers::Grower;

// use crate::growers::Grower;
mod growers;
// pub mod growers;

fn main() {
    let plant_killer = growers::MockedPie {
        water_port: 4,
        light_port: 5,
    };

    plant_killer.water();

    println!("Hello, world!");
}
