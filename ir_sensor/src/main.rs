// #![warn(unsafe_code)]
#![no_std]
#![no_main]

// use board::gpio::gpiob::PB6;
use board::hal::{delay::Delay, i2c::*, prelude::*, stm32}; // PAC = peripheral access crate, here equal stm32
use cortex_m::Peripherals;
use cortex_m_rt::entry;
use driver::{Mlx9061x, SlaveAddr};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
// use panic_halt as _;
use mlx9061x as driver;
use stm32f407g_disc as board;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let dp = stm32::Peripherals::take().unwrap();
    let cp = Peripherals::take().unwrap();

    let gpiob = dp.GPIOB.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    let scl = gpiob.pb6.into_alternate_af4_open_drain();
    let sda = gpiob.pb7.into_alternate_af4_open_drain();
    // let pins: PINS<I2C> = (scl, sda);

    // Setup I2C1 using PB6/PB7 at 400kHz bitrate (fast mode)
    let i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), clocks);
    rprintln!("I2C initialized");

    let addr = SlaveAddr::default();
    let maybe_sensor = Mlx9061x::new_mlx90614(i2c, addr, 5);
    if let Err(err) = maybe_sensor {
        rprintln!("I2C error: {:?}", err);
        loop {}
    }
    let mut sensor = maybe_sensor.unwrap();

    loop {
        if let Ok(obj_temp) = sensor.object1_temperature() {
            rprintln!("Object temperature: {:.2}ºC", obj_temp);
        } else {
            rprintln!("Read error.");
        }
        if let Ok(ambient_temp) = sensor.ambient_temperature() {
            rprintln!("Ambient temperature: {:.2}ºC\n", ambient_temp);
        } else {
            rprintln!("Read error.");
        }

        delay.delay_ms(1000_u16);
    }
}
