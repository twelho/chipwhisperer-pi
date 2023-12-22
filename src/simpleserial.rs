// SPDX-License-Identifier: GPL-3.0-or-later
// ChipWhisperer SimpleSerial ported to Rust
// (c) Dennis Marttinen 2023

use rppal::uart;
use rppal::uart::{Parity, Uart};
use std::time::Duration;

pub mod v1_1;
pub mod v2_1;

pub trait SimpleSerialImpl: From<Uart> {
    fn baud_rate() -> u32;

    fn new() -> uart::Result<Self> {
        let mut uart = Uart::new(Self::baud_rate(), Parity::None, 8, 1)?;
        uart.set_read_mode(1, Duration::from_secs(0))?; // Blocking reads
        uart.set_write_mode(true)?; // Blocking writes
        Ok(uart.into())
    }

    fn process(self, handler: impl FnMut(char, &mut Vec<u8>) -> (u8, bool)) -> !;
}
