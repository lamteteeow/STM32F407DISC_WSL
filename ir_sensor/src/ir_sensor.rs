// #![warn(unsafe_code)]
#![no_std]
#![no_main]

use board::hal::{delay::Delay, i2c::*, prelude::*, stm32}; // PAC = peripheral access crate, here equal stm32
use cortex_m::Peripherals;
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
    let clocks = rcc.cfgr.sysclk(144.mhz()).freeze();

    let mut delay = Delay::new(cp.SYST, clocks);

    // let mut scl = gpiob.pb6.into_push_pull_output();
    // let mut sda = gpiob.pb7.into_push_pull_output();

    let scl = gpiob.pb6.into_alternate_af4_open_drain();
    let sda = gpiob.pb7.into_alternate_af4_open_drain();

    // scl.internal_pull_up(true);
    // sda.internal_pull_up(true);

    // Setup I2C1 using PB6/PB7 at 400kHz bitrate (fast mode)
    let i2c = I2c::new(
        dp.I2C1,
        // cortex_m::interrupt::free(move |cs| {
        //     (
        //         gpiob.pb6.into_alternate_af4_open_drain(cs),
        //         gpiob.pb7.into_alternate_af4_open_drain(cs),
        //     )
        // }),
        (scl, sda),
        400.khz(),
        clocks,
    );
    rprintln!("I2C initialized");

    let addr = SlaveAddr::default();
    rprintln!("Slave address: {:?}", addr);

    let mut sensor = Mlx9061x::new_mlx90614(i2c, addr, 5).unwrap();
    // if let Err(err) = maybe_sensor {
    //     rprintln!("I2C error: {:?}", err);
    //     loop {}
    // }
    // let mut sensor = maybe_sensor.unwrap();

    if let Ok(id) = sensor.device_id() {
        rprintln!("Device ID: {}", id);
    } else {
        rprintln!("Read ID error.");
    }

    loop {
        if let Ok(obj_temp) = sensor.object1_temperature_as_int() {
            rprintln!("Object temperature: {}ºC", obj_temp);
        } else {
            // panic!("Read object error");
            rprintln!("Read object error.");
        }
        // if let Ok(ambient_temp) = sensor.ambient_temperature() {
        //     rprintln!("Ambient temperature: {:.2}ºC\n", ambient_temp);
        // } else {
        //     rprintln!("Read ambient error.");
        // }
        // // let t = sensor.read_u16(Register::TA)?;
        // let t_obj = sensor.object1_temperature().unwrap_or(-1.0);
        // rprintln!("Object temperature: {:.2}ºC", t_obj);
        // scl.toggle().unwrap();
        // delay.delay_ms(1000_u16);
        // delay.delay_ms(1000_u16);
        // sda.toggle().unwrap();
        // delay.delay_ms(1000_u16);
        // delay.delay_ms(1000_u16);

        // continue;
        delay.delay_ms(1000_u16);
    }
}
