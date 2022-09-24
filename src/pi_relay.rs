use std::error::Error;

use rppal::gpio::Gpio;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_PIN: u8 = 17;
#[tracing::instrument(name = "changing relay status")]
pub fn change_relay(relay_status: bool) -> Result<(), Box<dyn Error>> {
    tracing::debug!("starting relay change");

    let mut pin = Gpio::new()
        .expect("gpio failed")
        .get(GPIO_PIN)
        .expect("can't get pin")
        .into_output();

    pin.set_reset_on_drop(false);

    match relay_status {
        true => {
            pin.set_high();
            tracing::debug!("setting pin high");
        }
        false => {
            pin.set_low();
            tracing::debug!("setting pin low");
        }
    }

    Ok(())
}
