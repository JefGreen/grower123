use growers::Grower;
mod growers;
// use plantable::Plantable;
// mod plantable;
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

struct Plant {
    min_moisture: u32,
    hours_of_light_required: u32,
}

struct Batch {
    // a plant type being grown by a grower at a specific time
    grower: &'static dyn Grower,
    plant_type: Plant,
    planted_at: u32,
    last_watered_at: u32,
    last_light_at: u32,
}

type Callback = fn();

struct Task {
    created_at: u32,
    due_by: u32,
    action: Callback,
}

impl Task {
    pub fn new(action: Callback, due_by: u32) -> Task {
        Task {
            action,
            created_at: 043,
            due_by: 045,
        }
    }
}

struct Log {
    timestamp: u32,
}

fn needs_light() {}
// impl needs_watering
// needs_water

fn main() {
    let grower = growers::MockedPie::new(&4, &5);
    // let sunflower = plantable::Microgreen::new();

    grower.water(true);

    // Create a file
    create_db();
    read_db();

    let ten_millis = time::Duration::from_secs(1);
    let mut logs = Queue::new(11);
    let mut tasks = Queue::new(11);

    tasks.add(item);
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
    }
}
