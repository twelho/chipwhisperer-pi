// SPDX-License-Identifier: GPL-3.0-or-later
// ChipWhisperer SimpleSerial v1.1 ported to Rust
// (c) Dennis Marttinen 2023

use crate::simpleserial::SimpleSerialImpl;
use embedded_hal::serial::Read;
use rppal::uart::{Queue, Uart};

pub struct SimpleSerial {
    uart: Uart,
}

impl From<Uart> for SimpleSerial {
    fn from(uart: Uart) -> Self {
        Self { uart }
    }
}

impl SimpleSerial {
    fn write(&mut self, cmd: char, data: &[u8]) {
        self.uart.write(&[cmd as u8]).unwrap();
        self.uart.write(data).unwrap();
        self.uart.write(&['\n' as u8]).unwrap();
    }
}

impl SimpleSerialImpl for SimpleSerial {
    fn baud_rate() -> u32 {
        38400
    }

    fn process(mut self, mut handler: impl FnMut(char, &mut Vec<u8>) -> (u8, bool)) -> ! {
        self.uart.flush(Queue::Both).unwrap();
        println!("SS v1.1: waiting for commands...");

        let mut str = String::new();
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

                    if modified {
                        // Send response if data was modified
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
