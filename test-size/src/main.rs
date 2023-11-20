#![no_main]
#![no_std]

// use panic_halt as _;
use core::panic::PanicInfo;

use stm32f407g_disc as board;

use crate::board::{
    hal::delay::Delay,
    hal::prelude::*,
    hal::stm32,
    led::{LedColor, Leds},
};

// use cortex_m::delay::Delay;
use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider // clock does not work therefore delay_ms has no effect
        let mut delay = Delay::new(cp.SYST, clocks);

        // let mut delay =

        loop {
            // Turn LEDs on one after the other with 1000ms delay between them
            leds[LedColor::Orange].on();
            delay.delay_ms(1000_u32);
            leds[LedColor::Red].on();
            delay.delay_ms(1000_u32);
            leds[LedColor::Blue].on();
            delay.delay_ms(1000_u32);
            leds[LedColor::Green].on();
            delay.delay_ms(1000_u32);

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(1000_u32);
            delay.delay_ms(1000_u32);

            // Turn LEDs off one after the other with 1000ms delay between them
            leds[LedColor::Orange].off();
            delay.delay_ms(1000_u32);
            leds[LedColor::Red].off();
            delay.delay_ms(1000_u32);
            leds[LedColor::Blue].off();
            delay.delay_ms(1000_u32);
            leds[LedColor::Green].off();
            delay.delay_ms(1000_u32);

            // Delay twice for half a second due to limited timer resolution
            delay.delay_ms(1000_u32);
            delay.delay_ms(1000_u32);
        }
    }

    loop {
        continue;
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
