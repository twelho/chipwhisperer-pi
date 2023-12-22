// SPDX-License-Identifier: GPL-3.0-or-later
// ChipWhisperer triggering system ported to Rust
// (c) Dennis Marttinen 2023

use rppal::gpio::{Gpio, OutputPin};

pub struct Trigger {
    pin: OutputPin,
}

impl Trigger {
    pub fn new(bcm_id: u8) -> Self {
        let gpio = Gpio::new().unwrap();
        let mut pin = gpio.get(bcm_id).unwrap().into_output();
        pin.set_low(); // Assert that the trigger is low upon startup
        Self { pin }
    }

    pub fn high(&mut self) {
        println!("trigger high!");
        self.pin.set_high();
    }

    pub fn low(&mut self) {
        println!("trigger low!");
        self.pin.set_low();
    }
}
