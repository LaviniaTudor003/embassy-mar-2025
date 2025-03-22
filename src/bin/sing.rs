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

// PWM config
use embassy_rp::{
    adc::InterruptHandler,
    config,
    pwm::{Config as ConfigPwm, SetDutyCycle},
};

// ADC config
use embassy_rp::adc::{Adc, Channel, Config as ConfigAdc};
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
    /// Beats per minute.
    const TEMPO: u64 = 100;
    /// A whole note duration in milliseconds.
    const WHOLE_NOTE: u64 = 4 * (60_000 / TEMPO);
    /// The microcontroller clock frequency
    const CLOCK_FREQ: u64 = 150_000_000;
    /// PWM clock divider
    const PWM_DIV: u64 = 64;

    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    // TODO: Configure the PWM pin.
    
    let mut buzzer_cfg: ConfigPwm = Default::default();
    buzzer_cfg.divider = PWM_DIV.to_fixed();
    //buzzer_cfg.top = 0x9088;
    //buzzer_cfg.compare_a = buzzer_cfg.top / 2;
    let mut buzzer = Pwm::new_output_a(peripherals.PWM_SLICE6, peripherals.PIN_28, buzzer_cfg.clone());

    for (note, mut length) in OCTAVE {
        // TODO: Compute the note's duration based on
        // the length variable.
        if length < 0 {
            length = -3/2 * length;
        }
        let duration = length as u64 * WHOLE_NOTE / 4;
        
        match note {
            Some(note) => {
                // TODO: Configure the `top` and `compare_X` registers
                // based on the note's type and change the PWM's config.
                // Keep in mind that we are aiming for a 50% duty cycle.
                // "Play" the note for 90% of the duration, then insert
                // a 10% pause before playing the next note.
                info!("Playing note ms");
                let top = CLOCK_FREQ  / (note as u64 * PWM_DIV) - 1;
                buzzer_cfg.top = top as u16;
                buzzer_cfg.compare_a = buzzer_cfg.top / 2;
                buzzer.set_config(&buzzer_cfg);
                Timer::after_millis(duration * 9 / 10).await;
                buzzer_cfg.compare_a = 0;
                buzzer.set_config(&buzzer_cfg);
                Timer::after_millis(duration / 10).await;
            },
            None => {
                // TODO: Just wait the whole duration.
                Timer::after_millis(duration).await;
            }
        };
    }
}
