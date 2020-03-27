#![no_std]
#![no_main]
#![allow(unused_imports)]

use stm32f1xx_hal as hal;

use core::{fmt::Arguments, mem::MaybeUninit, str::Utf8Error};
use cortex_m::{itm, peripheral::itm::Stim};
use cortex_m_rt::entry;
use embedded_hal::digital::v2::OutputPin;
use hal::gpio::{gpioa::*, gpiob::*, Alternate, Floating, Input, Output, PushPull};
use hal::{
    delay::Delay,
    i2c::{BlockingI2c, DutyCycle, Mode},
    pac::SPI1,
    prelude::*,
    spi::Spi,
    stm32,
};
use lm75::{Lm75, SlaveAddr};
use panic_itm;

fn log(itm: &mut Stim, msg: &str) {
    // FIXME: comment these out when not connected to openocd. itm will crash otherwise
    itm::write_str(itm, msg);
    itm::write_str(itm, "\n");
}

fn log_fmt(itm: &mut Stim, args: Arguments) {
    // FIXME: comment these out when not connected to openocd. itm will crash otherwise
    itm::write_fmt(itm, args);
    itm::write_str(itm, "\n");
}

#[entry]
fn main() -> ! {
    let mut cp: cortex_m::Peripherals = cortex_m::Peripherals::take().unwrap();
    let dp = stm32::Peripherals::take().unwrap();
    let itm = &mut cp.ITM;

    log(&mut itm.stim[0], "[INF] Initializing");

    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);
    let mut flash = dp.FLASH.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
    let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

    let i2c = BlockingI2c::i2c1(
        dp.I2C1,
        (scl, sda),
        &mut afio.mapr,
        Mode::Fast {
            frequency: 100_000,
            duty_cycle: DutyCycle::Ratio2to1,
        },
        clocks,
        &mut rcc.apb1,
        1000,
        10,
        1000,
        1000,
    );

    let mut lm75 = Lm75::new(i2c, SlaveAddr::Alternative(false, false, false));

    loop {
        delay.delay_ms(250_u16);

        if let Ok(temp) = lm75.read_temperature() {
            log_fmt(
                &mut itm.stim[0],
                format_args!("[DBG] temperature reading: {} degrees C", temp),
            );
        }
    }
}
