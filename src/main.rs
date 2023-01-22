use crate::growers::raspberry_pie::RaspberryPie;

pub mod growers;

fn main() {
    let mut plant_killer = RaspberryPie::new(4, 5);
    

    println!("Hello, world!");
}
