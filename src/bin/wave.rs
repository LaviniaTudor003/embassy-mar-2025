// +---------------------------------------------------------------------------+
// |                            Embassy March 2025                             |
// |                             Smile and Wave!                               |
// +---------------------------------------------------------------------------+

#![no_std]
#![no_main]

/*use embassy_executor::Spawner;
use fixed::traits::ToFixed;
use embassy_rp::pwm::{self, Pwm};
// Use the `panic_probe` crate to provided the panic handler and the 
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;*/
use embassy_executor::Spawner;
use embassy_mar_2025::{bmp280::Config, music::{Note, OCTAVE}};
use embassy_rp::{adc::{self, Adc, Channel}, bind_interrupts, gpio::Pull, pwm::{self, Pwm}};
use embassy_time::{Duration, Timer};
use fixed::traits::ToFixed;
use embassy_rp::adc::Config as ConfigAdc; 
// PWM config
use embassy_rp::{
    adc::InterruptHandler,
    config,
    pwm::{Config as ConfigPwm, SetDutyCycle},
};

// ADC config
// Use the `panic_probe` crate to provided the panic handler and the
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
bind_interrupts!(struct Irqs {
    ADC_IRQ_FIFO => InterruptHandler;
});
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    // Configure PWM for servo control
    let mut servo_config: ConfigPwm = Default::default();

    // Set the calculated TOP value for 50 Hz PWM
    servo_config.top = 0xB71A; 

    // Set the clock divider to 64
    servo_config.divider = 64_i32.to_fixed(); // Clock divider = 64

    // Servo timing constants
    const PERIOD_US: usize = 20_000; // 20 ms period for 50 Hz
    const MIN_PULSE_US: usize = 500; // 0.5 ms pulse for 0 degrees
    const MAX_PULSE_US: usize = 2500; // 2.5 ms pulse for 180 degrees

    // Calculate the PWM compare values for minimum and maximum pulse widths
    let min_pulse = (MIN_PULSE_US * servo_config.top as usize) / PERIOD_US;
    let max_pulse = (MAX_PULSE_US * servo_config.top as usize) / PERIOD_US;

    // Initialize PWM for servo control
    let mut servo = Pwm::new_output_a(
        peripherals.PWM_SLICE1, 
        peripherals.PIN_2, 
        servo_config.clone()
    );

    let mut adc = Adc::new(peripherals.ADC, Irqs, ConfigAdc::default());
    let mut adc_pin = Channel::new_pin(peripherals.PIN_28, Pull::None); 
    let mut adc_pin2 = Channel::new_pin(peripherals.PIN_27, Pull::None); 

    loop {
        let x_val: u16 = adc.read(&mut adc_pin).await.unwrap();
        info!("x_val: {}", x_val);

        if x_val < 1500 {
            servo.set_duty_cycle(min_pulse as u16);
        } else if x_val > 3000 {
            servo.set_duty_cycle(max_pulse as u16);
        } else {
            let middle = (min_pulse + max_pulse) / 2;
            servo.set_duty_cycle(middle as u16);
        }
        Timer::after(Duration::from_millis(20)).await;
    }
    
}
