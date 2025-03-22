// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                            Sing your own tune                             |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_mar_2025::music::{Note, OCTAVE};
use embassy_rp::pwm::{self, Pwm};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
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

    // TODO: Configure the PWM pin.
    let buzzer_cfg = ();
    let buzzer = ();

    for (note, length) in OCTAVE {
        // TODO: Compute the note's duration based on
        // the length variable.
        let duration = ();
        
        match note {
            Some(note) => {
                // TODO: Configure the `top` and `compare_X` registers
                // based on the note's type and change the PWM's config.
                // Keep in mind that we are aiming for a 50% duty cycle.
                // "Play" the note for 90% of the duration, then insert
                // a 10% pause before playing the next note.
            },
            None => {
                // TODO: Just wait the whole duration.
            }
        };
    }
}
