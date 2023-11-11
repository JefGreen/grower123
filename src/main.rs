#![no_main]
#![no_std]
use core::arch::asm;
use core::panic::PanicInfo;
// Found good documentation for embedded rust https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials/tree/master/00_before_we_start
// Raspberry pi linux kernel: https://github.com/raspberrypi/linux
// From official doc: https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
// good read: https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050
// pins setup https://pi4j.com/1.2/pins/model-3b-rev1.html
// datasheet: https://www.raspberrypi.org/app/uploads/2012/02/BCM2835-ARM-Peripherals.pdf
mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

struct GPIO;

const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;

const GPIO_SET0: u32 = 0x3f20_001c;
const GPIO_CLR0: u32 = 0x3f20_0028;

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

        unsafe {
            val = core::ptr::read_volatile(register as *mut u32);
        }

        let mut mask: u32 = 0b111;

        let pinnum = pin % 10;
        mask = mask << pinnum * 3;

        val = val & !(mask);

        val |= 1 << pinnum * 3;

        unsafe {
            core::ptr::write_volatile(register as *mut u32, val);
        }
    }

    pub fn on(pin: u32) {
        let bitposition = pin;
        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(GPIO_SET0 as *mut u32);
        }

        val |= 1 << bitposition;

        unsafe {
            core::ptr::write_volatile(GPIO_SET0 as *mut u32, val);
        }
    }

    pub fn off(pin: u32) {
        let bitposition = pin;
        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(GPIO_CLR0 as *mut u32);
        }

        val |= 1 << bitposition;

        unsafe {
            core::ptr::write_volatile(GPIO_CLR0 as *mut u32, val);
        }
    }
}

fn sleep() {
    // See https://developer.arm.com/documentation/den0013/d/ARM-Thumb-Unified-Assembly-Language-Instructions/Miscellaneous-instructions/Other-instructions
    for _ in 1..50000 {
        unsafe {
            asm!("nop");
        }
    }
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    GPIO::set_ouput(21);

    loop {
        GPIO::on(21);

        sleep();
        GPIO::off(21);

        sleep();
    }
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
