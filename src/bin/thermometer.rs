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
use embassy_rp::{adc::InterruptHandler, config, pwm::{Config as ConfigPwm, SetDutyCycle}};
use embassy_rp::pwm::Pwm;
// Use the `panic_probe` crate to provided the panic handler and the 
// `defmt_rtt` to import the runtime for defmt, to be able to use
// the print macros.
use {defmt_rtt as _, panic_probe as _};
// Use the logging macros provided by defmt.
#[allow(unused)]
use defmt::*;
use bmp280::i2c::BMP280;

embassy_rp::bind_interrupts!(struct Irqs {
    // Do not forget to bind the I2C peripheral interrupt to its handler
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Get a handle to the RP's peripherals.
    let peripherals = embassy_rp::init(Default::default());

    let mut config_red: ConfigPwm = Default::default();
    config_red.top = 0x9088; // in HEX, equals 37000 in decimal
    config_red.compare_a = config_red.top;

    // GREEN
    let mut config_green: ConfigPwm = Default::default();
    config_green.top = 0x9088;
    config_green.compare_a = config_green.top;

    // BLUE
    let mut config_blue: ConfigPwm = Default::default();
    config_blue.top = 0x9088;
    config_blue.compare_a = config_blue.top;

    // RED
    let mut pwm_red = Pwm::new_output_a(
        peripherals.PWM_SLICE3,
        peripherals.PIN_6,
        config_red.clone()
    );

    // GREEN
    let mut pwm_green = Pwm::new_output_a(
        peripherals.PWM_SLICE4,
        peripherals.PIN_8,
        config_green.clone()
    );

    // BLUE
    let mut pwm_blue = Pwm::new_output_a(
        peripherals.PWM_SLICE5,
        peripherals.PIN_10,
        config_blue.clone()
    );

    config_red.compare_a = 0;
    pwm_red.set_config(&config_red);
    pwm_green.set_config(&config_green);
    pwm_blue.set_config(&config_blue);

    embassy_rp::bind_interrupts!(struct Irqs {
        // Do not forget to bind the I2C peripheral interrupt to its handler
        I2C0_IRQ => embassy_rp::i2c::InterruptHandler<embassy_rp::peripherals::I2C0>;
    });

    let bus = I2c::new_async(
            peripherals.I2C0,
            peripherals.PIN_21,
            peripherals.PIN_20,
            Irqs, Default::default());
    let mut bmp = BMP280::new(bus).unwrap();
    let control: Control = Control { osrs_t: bmp280::Oversampling::x1, osrs_p: bmp280::Oversampling::x1, mode: bmp280::PowerMode::Forced};
    bmp.set_control(control).await;

    let mut asc = true;
    let step = config_red.top / 37000;
    loop {
        if asc {
            config_red.compare_a += step;
            config_blue.compare_a -= step;

            pwm_red.set_config(&config_red);
            pwm_blue.set_config(&config_blue);

            if config_blue.compare_a < step {
                asc = false;

                config_blue.compare_a = 0;
                config_red.compare_a = config_red.top;
                pwm_red.set_config(&config_red);
                pwm_blue.set_config(&config_blue);

                Timer::after_secs(1).await;
            }
        } else {
            config_red.compare_a -= step;
            config_blue.compare_a += step;

            pwm_red.set_config(&config_red);
            pwm_blue.set_config(&config_blue);

            if config_red.compare_a < step {
                asc = true;

                config_red.compare_a = 0;
                config_blue.compare_a = config_red.top;
                pwm_red.set_config(&config_red);
                pwm_blue.set_config(&config_blue);

                Timer::after_secs(1).await;
            }
        }

        Timer::after_nanos(50).await;

        info!("Temperature: {} - Pressure: {}", bmp.temp().await, bmp.pressure().await);
        bmp.set_control(control).await;
    }
}
