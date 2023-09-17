use growers::Grower;
mod growers;
use std::fs::File;
use std::io::Read;
use std::io::Write;

// Possibly create a trait called persistable
fn create_db() {
    let mut data_file = File::create("data.json").expect("creation failed");

    data_file.write("{}".as_bytes()).expect("write failed");
}

fn remove_db() {
    fs::remove_file("data.json").expect("could not remove file");

    println!("Removed file data.txt");
}

fn read_db() {
    let mut data_file = File::open("data.json").expect("creation failed");

    data_file.read().expect("write failed");
}

fn main() {
    let grower = growers::MockedPie::new(4, 5);

    grower.water();

    println!("Hello, world!");

    // Create a file
    create_db()
}
