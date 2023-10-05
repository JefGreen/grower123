use growers::Grower;
mod growers;
// use plantable::Plantable;
// mod plantable;
use queue::Queue;
use std::time::{Duration, SystemTime};
mod queue;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
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

// type Callback = fn();

// Saving a callback could refer to method name in a hashmap
struct Task {
    created_at: u32,
    due_by: u32,
    action: fn(),
}

impl Task {
    pub fn new(action: fn(), due_by: u32) -> Task {
        Task {
            action,
            created_at: 043,
            due_by: 045,
        }
    }
}

impl Eq for Task {}

impl Ord for Task {
    fn cmp(&self, other: &Task) -> Ordering {
        self.due_by.cmp(&other.due_by).reverse()
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Task {
    fn eq(&self, other: &Task) -> bool {
        self.due_by == other.due_by
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
    fn water() {
        println!("Calling water method")
    }

    fn light_on() {
        println!("Calling light on method")
    }

    // Create a file
    create_db();
    read_db();

    let ten_millis = time::Duration::from_secs(1);
    let mut logs = Queue::new(11);
    let mut tasks: BinaryHeap<Task> = BinaryHeap::new();

    let a: Task = Task {
        created_at: 45,
        due_by: 1,
        action: water,
    };

    let b: Task = Task {
        created_at: 45,
        due_by: 2,
        action: light_on,
    };

    tasks.push(a);
    tasks.push(b);
    // Load vector from json file
    let mut i = 0;
    loop {
        // for batch in &batches {
        //     batch.grower.water();
        // } back up bd avant une release
        logs.add(i);
        println!("{:?} at {}", logs.peek(), i);
        i += 1;

        let task = tasks.pop();

        match task {
            Some(t) => {
                println!("Due by: {}", t.due_by);
                (t.action)();
            }
            None => {}
        }

        // p.action();
        // let now = SystemTime::now();
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(now) => {
                println!("time here {}", now.as_secs());
            }
            Err(e) => {
                // an error occurred!
                println!("Error: {e:?}");
            }
        }
        thread::sleep(ten_millis);
    }
}
