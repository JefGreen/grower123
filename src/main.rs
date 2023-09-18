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

// fn remove_db() {
//     fs::remove_file("data.json").expect("could not remove file");

//     println!("Removed file data.txt");
// }

fn read_db() {
    // Read a file in the local file system
    let mut data_file = File::open("data.json").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);
}

fn main() {
    let grower = growers::MockedPie::new(4, 5);

    grower.water();

    println!("Hello, world!");

    // Create a file
    create_db();
    read_db();
}
