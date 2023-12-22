// SPDX-License-Identifier: GPL-3.0-or-later
// ChipWhisperer SimpleSerial v2.1 ported to Rust
// (c) Dennis Marttinen 2023

use crate::simpleserial::SimpleSerialImpl;
use crc_any::CRC;
use embedded_hal::serial::Read;
use rppal::uart::{Queue, Uart};
use std::iter;
use std::time::Duration;

#[derive(Debug)]
#[allow(dead_code)]
enum SSError {
    Ok,
    InvalidCommand,
    BadCRC, // CRC mismatch
    Timeout,
    InvalidLength,       // dlen must be less than 250 in SS v2.1
    UnexpectedFrameByte, // COBS decoding failed?
}

pub struct SimpleSerial {
    crc: CRC,
    uart: Uart,
}

impl From<Uart> for SimpleSerial {
    fn from(uart: Uart) -> Self {
        let crc = CRC::create_crc_u8(0x4D, 8, 0, 0, false);
        Self { crc, uart }
    }
}

impl SimpleSerial {
    fn crc(&mut self, packet: &[u8]) -> u8 {
        self.crc.digest(packet);
        let crc = self.crc.get_crc();
        self.crc.reset(); // Reset for next round
        crc as u8
    }

    fn recv(&mut self) -> Result<(char, Vec<u8>), SSError> {
        // The outer framing is COBS with null termination
        let frame: Vec<_> = iter::repeat(0)
            .map(|_| Read::read(&mut self.uart).unwrap())
            .take_while(|a| *a != 0) // Capture everything until EOF
            .collect();

        println!("received frame: {:?}", frame);

        if frame.len() == 0 {
            println!("empty frame, skipping...");
            return self.recv();
        }

        (frame.len() < 0xFF) // Excludes null terminator
            .then_some(())
            .ok_or(SSError::InvalidLength)?;

        // This works without the null terminator (take_while excludes it)
        // Packet format: [cmd, scmd, dlen, data_0, data_1, ..., data_n, CRC (poly=0x4D)]
        let packet = cobs::decode_vec(&frame).map_err(|_| SSError::UnexpectedFrameByte)?;
        println!("received command: {:?}", packet);

        // Unpack CRC data
        let [crc_data @ .., crc] = &packet[..] else {
            return Err(SSError::InvalidLength);
        };

        // Check CRC
        (*crc == self.crc(crc_data))
            .then_some(())
            .ok_or(SSError::BadCRC)?;

        // Destructure packet
        let [cmd, _, _, data @ .., _] = &packet[..] else {
            return Err(SSError::InvalidLength);
        };

        // Return cmd and data
        Ok((*cmd as char, data.into()))
    }

    fn send(&mut self, cmd: char, data: &[u8]) {
        // Packet format: [cmd, dlen, data_0, data_1, ..., data_n, CRC (poly=0x4D)]
        let mut packet = vec![cmd as u8, data.len() as u8];
        packet.extend_from_slice(data);
        packet.push(self.crc(&packet));
        println!("sending response: {:?}", packet);
        let mut frame = cobs::encode_vec(&packet);
        assert!(frame.len() < 0xFF, "encoded frame too long");
        frame.push(0); // Frames must be null-terminated
        self.uart.write(&frame).unwrap();
    }

    fn send_err(&mut self, err: u8) {
        self.send('e', &[err])
    }
}

impl SimpleSerialImpl for SimpleSerial {
    fn baud_rate() -> u32 {
        230400
    }

    fn process(mut self, mut handler: impl FnMut(char, &mut Vec<u8>) -> (u8, bool)) -> ! {
        // This SimpleSerial version has predictable length messages,
        // set min_length higher so we can avoid reading one byte at a time
        self.uart
            .set_read_mode(0xFF, Duration::from_secs(0))
            .unwrap();
        self.uart.flush(Queue::Both).unwrap();
        println!("SS v2.1: waiting for commands...");

        loop {
            // Receive cmd and data
            let (cmd, mut data) = match self.recv() {
                Ok(d) => d,
                Err(e) => {
                    eprintln!("error during receive: {:?}", e);
                    self.send_err(e as u8);
                    continue;
                }
            };

            if cmd == 'v' {
                self.send_err(3); // Version command
                continue;
            }

            // Run handler, which returns status and modified flag
            let (status, modified) = handler(cmd, &mut data);

            if modified {
                // Send response if data was modified
                self.send('r', &data);
            }

            // Always send ACK
            self.send_err(status);
        }
    }
}
