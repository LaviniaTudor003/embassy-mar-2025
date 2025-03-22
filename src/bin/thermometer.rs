// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            My own thermometer!                            |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mar_2025::bmp280::{self, Control};
use embassy_rp::i2c::I2c;
use embassy_time::Timer;
// Use the `panic_probe` crate to provided the panic handler and the 
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;

embassy_rp::bind_interrupts!(struct Irqs {
    // Do not forget to bind the I2C peripheral interrupt to its handler
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let _p = embassy_rp::init(Default::default());

    loop {
    }
}
