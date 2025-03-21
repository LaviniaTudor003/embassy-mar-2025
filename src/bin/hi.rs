// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Hi, I am new here!                             |
// +---------------------------------------------------------------------------+

//! By default, this app blinks and LED connected to the `GP4` pin.

#![no_std]
#![no_main]

use embassy_executor::Spawner;
// Use the `panic_probe` crate to provided the panic handler and the 
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let _p = embassy_rp::init(Default::default());

    loop {
    }
}
