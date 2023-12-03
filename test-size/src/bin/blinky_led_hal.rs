#![no_main]
#![no_std]

// use panic_halt as _;
use core::panic::PanicInfo;
use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use stm32f407g_disc::{
    hal::delay::Delay,
    hal::prelude::*,
    hal::stm32,
    led::{LedColor, Leds},
};
use volatile::Volatile;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain(); //=> need to check Reset and Clock Control (RCC) in STM32F407 Reference Manual

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider // clock does not work therefore delay_ms has no effect
        let mut delay = Delay::new(cp.SYST, clocks);

        let mut one_sec: u16 = 1000;
        let v_one_sec = Volatile::new(&mut one_sec);

        loop {
            // Turn LEDs on one after the other with 1000ms delay between them
            leds[LedColor::Orange].on();
            delay.delay_ms(v_one_sec.read());
            leds[LedColor::Red].on();
            delay.delay_ms(v_one_sec.read());
            leds[LedColor::Blue].on();
            delay.delay_ms(v_one_sec.read());
            leds[LedColor::Green].on();
            delay.delay_ms(v_one_sec.read());

            // delay.delay_ms(v_one_sec.read());
            // delay.delay_ms(v_one_sec.read());

            // Turn LEDs off one after the other with 1000ms delay between them
            leds[LedColor::Orange].off();
            delay.delay_ms(v_one_sec.read());
            leds[LedColor::Red].off();
            delay.delay_ms(v_one_sec.read());
            leds[LedColor::Blue].off();
            delay.delay_ms(v_one_sec.read());
            leds[LedColor::Green].off();
            delay.delay_ms(v_one_sec.read());

            // delay.delay_ms(v_one_sec.read());
            // delay.delay_ms(v_one_sec.read());
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
