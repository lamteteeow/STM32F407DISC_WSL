#![deny(unsafe_code)]
#![no_main]
#![no_std]

// use core::panic::PanicInfo;
#[allow(unused_imports)]
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use cortex_m_rt::entry;
use panic_itm as _;

// Need stm32f3xx_hal::prelude::* otherwise
//   'Error(corex-m-rt): The interrupt vectors are missing`
#[allow(unused_imports)]
use stm32f407g_disc::hal::prelude::*;

#[entry]
fn main() -> ! {
    // panic!("Hello, world!");

    let mut itm = init();

    iprintln!(&mut itm.stim[0], "Hello, world!");

    loop {}
}

fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();
    p.ITM
}

// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }
