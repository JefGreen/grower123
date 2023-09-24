use growers::Grower;
mod growers;
use plantable::Plantable;
mod plantable;
use queue::Queue;
mod queue;
use std::fs::File;
use std::io::{Read, Write};
use std::{thread, time};

// Possibly create a trait called persistable
fn create_db() {
    let mut data_file = File::create("data.json").expect("creation failed");

    data_file.write("{}".as_bytes()).expect("write failed");
}

// Should save exist inside water?
fn read_db() {
    // Read a file in the local file system
    let mut data_file = File::open("data.json").unwrap();

    // Create an empty mutable string
    let mut file_content = String::new();

    // Copy contents of file to a mutable string
    data_file.read_to_string(&mut file_content).unwrap();

    println!("File content: {:?}", file_content);
}

struct Batch {
    // a plant type being grown by a grower at a specific time
    grower: &'static dyn Grower,
    plant_type: &'static dyn Plantable,
    planted_at: u32,
}

struct Log {
    timestamp: u32,
}

// impl needs_watering

fn main() {
    let grower = growers::MockedPie::new(&4, &5);
    let sunflower = plantable::Microgreen::new();

    grower.water();

    // Create a file
    create_db();
    read_db();

    // let batches: [Batch; 1] = [Batch {
    //     grower: &grower,
    //     plant_type: &sunflower,
    //     planted_at: 4,
    // }];

    // How to limit the amount of entries in a json?
    // Index reccords on a counter and reset counter when limit is reached

    let ten_millis = time::Duration::from_secs(1);
    let mut logs = Queue::new(11);
    // Load vector from json file
    let mut i = 0;
    loop {
        // for batch in &batches {
        //     batch.grower.water();
        // }
        logs.add(i);
        println!("{:?} at {}", logs.peek(), i);
        i += 1;

        thread::sleep(ten_millis);
        println!("Current time");
    }
}
