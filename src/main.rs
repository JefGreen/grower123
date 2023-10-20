#![no_main]
#![no_std]

use core::panic::PanicInfo;
// Found good documentation for embedded rust https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/00_before_we_start
// Raspberry pi linux kernel: https://github.com/raspberrypi/linux

// good read: https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let sunflower = plantable::Microgreen::new();
    use core::time::Duration;

    // Create a file
    create_db();
    read_db();

    let one_second = time::Duration::from_secs(1);
    let mut tasks: BinaryHeap<Task> = BinaryHeap::new();

    // let a: Task = Task {
    //     created_at: 45,
    //     due_by: 1,
    //     action: water,
    // };

    // let b: Task = Task {
    //     created_at: 45,
    //     due_by: 2,
    //     action: light_on,
    // };

    // tasks.push(a);
    // tasks.push(b);
    // Load vector from json file

    loop {
        // for batch in &batches {
        //     batch.grower.water();
        // } back up bd avant une release

        // p.action();
        let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(now) => now.as_secs(),
            Err(e) => Err(e),
        };

        // .as_secs()

        let task = tasks.peek();

        match task {
            Some(t) => {
                // println!("Due by: {}", t.due_by);
                (t.action)();
            }
            None => {}
        }
        thread::sleep(one_second);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{Read, Write};
use std::time::{Duration, SystemTime};
use std::{thread, time};

// TODO: change for embeded
struct GPIO;

const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;

impl GPIO {
    pub fn set_ouput(pin: u32) {
        let reg = pin / 10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Invalid pin number!"),
        };

        let mut val: u32 = 0;

        unsafe {}
    }
}

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
