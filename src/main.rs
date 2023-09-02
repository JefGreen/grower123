use crate::growers::Grower;

pub mod growers;

fn main() {
    let plant_killer = <dyn Grower as RaspberryPie>::new(4, 5);

    println!("Hello, world!");
}
