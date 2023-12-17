// SPDX-License-Identifier: GPL-3.0-or-later
// ChipWhisperer SimpleSerial v1.1 and triggering system ported to Rust
// (c) Dennis Marttinen 2023

use std::time::Duration;
use rppal::uart;
use rppal::uart::{Parity, Queue, Uart};
use embedded_hal::serial::Read;
use rppal::gpio::{Gpio, OutputPin};

pub struct SimpleSerial {
    uart: Uart,
}

impl SimpleSerial {
    pub fn new() -> uart::Result<Self> {
        let mut uart = Uart::new(38400, Parity::None, 8, 1)?;
        uart.set_read_mode(1, Duration::from_secs(0))?; // Blocking read
        uart.set_write_mode(true)?; // Blocking write
        Ok(Self { uart })
    }

    fn write(&mut self, cmd: char, data: &[u8]) {
        self.uart.write(&[cmd as u8]).unwrap();
        self.uart.write(data).unwrap();
        self.uart.write(&['\n' as u8]).unwrap();
    }

    pub fn process<H>(mut self, mut handler: H) -> ! where H: FnMut(char, &mut Vec<u8>) -> (u8, bool) {
        self.uart.flush(Queue::Both).unwrap();
        let mut str = String::new();
        println!("ss: waiting for commands...");

        loop {
            str.clear();
            let mut cmd_opt = None;

            loop {
                let c = Read::read(&mut self.uart).unwrap() as char;

                if c == '\n' {
                    let cmd = cmd_opt.take().expect("invalid input: no command specified");
                    println!("received command: {} {}", cmd, str);
                    let mut buf = hex::decode(&str).unwrap();

                    // Run handler, which returns status and modified flag
                    let (status, modified) = handler(cmd, &mut buf);

                    if modified { // Send response if data was modified
                        let res = hex::encode(buf);
                        println!("sending response: r {}", res);
                        self.write('r', res.as_ref());
                    }

                    // Always send ACK
                    self.write('z', hex::encode(&[status]).as_ref());

                    break; // Next command
                }

                match cmd_opt {
                    Some(_) => str.push(c),
                    None => cmd_opt = Some(c),
                }
            }
        }
    }
}

pub struct Trigger {
    pin: OutputPin,
}

impl Trigger {
    pub fn new(bcm_id: u8) -> Self {
        let gpio = Gpio::new().unwrap();
        let pin = gpio.get(bcm_id).unwrap().into_output();
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
