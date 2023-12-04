// #![warn(unsafe_code)]
#![no_std]
#![no_main]

use board::hal::{delay::Delay, i2c::*, prelude::*, stm32}; // PAC = peripheral access crate, here equal stm32
use cortex_m::{asm, Peripherals};
use cortex_m_rt::entry;
use driver::{Mlx9061x, SlaveAddr};
use mlx9061x as driver;
// use panic_probe as _;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};
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
        // asm::bkpt();
        if let Ok(obj_temp) = sensor.object1_temperature_as_int() {
            rprintln!("Object temperature: {:.2}oC", obj_temp);
        } else {
            rprintln!("Read object error.");
        }
        if let Ok(ambient_temp) = sensor.ambient_temperature_as_int() {
            rprintln!("Ambient temperature: {:.2}ºC\n", ambient_temp);
        } else {
            rprintln!("Read ambient error.");
        }

        delay.delay_ms(1000_u16);
    }
}
