// use crate::growers::Grower;
mod growers;
// pub mod growers;

fn main() {
    let plant_killer = growers::MockedPie::new(4, 5);

    println!("Hello, world!");
}
