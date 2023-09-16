use growers::Grower;
mod growers;

fn main() {
    let plant_killer = growers::MockedPie::new(4, 5);

    plant_killer.water();

    println!("Hello, world!");
}
