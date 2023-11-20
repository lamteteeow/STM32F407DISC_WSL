// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use cortex_m_rt::entry;
// Just use the HAL for pulling in its interrupt vectors (and may be some other fairy dust).
use stm32f4xx_hal as _;

#[entry]
fn main() -> ! {
    let _y;
    let x = 42;
    _y = x;

    // infinite loop; just so we don't leave this stack frame
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
