#![no_main]
#![no_std]

// use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;

fn main() {
    hprintln!("Hello, world!");
    // loop {}
}
