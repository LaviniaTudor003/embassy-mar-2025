// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                                   Demo                                    |
// +---------------------------------------------------------------------------+

//! By default, this app blinks and LED connected to the `GP4` pin.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::select::select4;
use embassy_rp::gpio::Output;
use embassy_time::{Duration, Timer};
// Use the `panic_probe` crate to provided the panic handler and the 
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
use embassy_rp::gpio::{Input, Pull};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_4, embassy_rp::gpio::Level::High);

   
    let mut sw4 = Input::new(p.PIN_5, Pull::Up);
    let mut sw5 = Input::new(p.PIN_6, Pull::Up);
    let mut sw6 = Input::new(p.PIN_7, Pull::Up);
    let mut sw7 = Input::new(p.PIN_8, Pull::Up);

    let delay = Duration::from_secs(1);

    loop {

        select4(
            sw4.wait_for_falling_edge(),
            sw5.wait_for_falling_edge(),
            sw6.wait_for_falling_edge(),
            sw7.wait_for_falling_edge(),
        ).await;

        led.set_low();
        defmt::info!("Tudor_ML");

        Timer::after(Duration::from_secs(1)).await;
        led.set_high();
    }
}

