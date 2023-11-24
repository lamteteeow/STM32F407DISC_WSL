#![no_std]
// #![allow(non_camel_case_types)]

pub use stm32f407g_disc as board;
pub use stm32f4xx_hal as hal;

pub use cortex_m_rt::entry;
pub use hal::pac::interrupt::*;
pub use hal::pac::Interrupt;
pub use hal::pac::Peripherals;

pub use board::{
    hal::delay::Delay,
    hal::prelude::*,
    hal::stm32,
    led::{LedColor, Leds},
};

// pub mod accelerometer;
// pub mod led;
// pub mod dummy;
