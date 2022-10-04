//! pi_relay.rs
//! Controls a relay connected to the Pi - by setting a pin high or low
//! on demnad.

use rppal::gpio::Gpio;
use std::error::Error;

// Gpio uses BCM pin numbering. BCM GPIO 17 is tied to physical pin 11.
const GPIO_PIN: u8 = 17;

#[tracing::instrument(name = "changing relay status")]
pub fn change_relay(relay_status: bool) -> Result<(), Box<dyn Error>> {
    tracing::debug!("starting relay change");

    // Grab a handle to the pin we want to control, and set it up to be an
    // output pin
    let mut pin = Gpio::new()
        .expect("gpio failed")
        .get(GPIO_PIN)
        .expect("can't get pin")
        .into_output();

    // Set this so the pin stays put after leaving this function - and having
    // pin fall out of scope. Default behavior is to reset the pin back to low (false)
    // when pin falls out of scope. This is simpler than trying to keep pin as a long-running
    // (static) value.
    pin.set_reset_on_drop(false);

    // set the pin according to the relay_status
    match relay_status {
        true => {
            tracing::debug!("setting pin high");
            pin.set_high();
        }
        false => {
            tracing::debug!("setting pin low");
            pin.set_low();
        }
    }

    Ok(())
}
