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

// use std::io;

// fn main() {
//     println!("Hello, world!");
//     let arr: (&str, i32, f32) = ("vai dai", 3, 3.2);
//     println!("{} and {} but {}", arr.0, arr.1, arr.2);
//     let mut input: String = String::new();
//     io::stdin()
//         .read_line(&mut input)
//         .expect("failed to read line"); //to catch the Warning of 'Result' object that must be used, it it is 'Err' then throw exception
//     println!("{}", input);
//     let big_num = 1250_00_i64;
//     println!("{}", big_num);

//     let max_i32 = i32::MAX as i64 + 1; // should result overflow
//     let divider = 10_i32;
//     let result = max_i32 as i32 / divider; // intentionally typecast for overflow
//     println!("{}", result);
//     let int_result: i64 = input.trim().parse().unwrap();
//     println!("{}", int_result + 5);
// }
