#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[cortex_m_rt::entry]
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
