use growers::Grower;
mod growers;

fn main() {
    let grower = growers::MockedPie::new(4, 5);

    grower.water();

    println!("Hello, world!");
}
