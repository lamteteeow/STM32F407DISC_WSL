// #![warn(unsafe_code)]
#![no_std]
#![no_main]

// use board::gpio::gpiob::PB6;
use board::hal::{delay::Delay, i2c::*, prelude::*, stm32}; // PAC = peripheral access crate, here equal stm32
use cortex_m_rt::entry;
// use rtt_target::{rprintln, rtt_init_print};
// use cortex_m_semihosting::hprintln;
use cortex_m::Peripherals;
use panic_rtt_target as _;
use stm32f407g_disc as board;

// impl<I2C1> PinSda<I2C1> for Pins<I2C1> {}

#[entry]
fn main() -> ! {
    // rtt_init_print!();
    let mut dp = stm32::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let gpiob = dp.GPIOB.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    let scl = gpiob.pb6.into_alternate_af4_open_drain();
    let sda = gpiob.pb7.into_alternate_af4_open_drain();
    // let pins: PINS<I2C> = (scl, sda);

    // Setup I2C1 using PB6/PB7 at 400kHz bitrate (fast mode)
    let i2c = I2c::new(dp.I2C1, (scl, sda), 200.khz(), clocks);

    // rprintln!("I2C initialized");

    loop {
        continue;
    }
}
