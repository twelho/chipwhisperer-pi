// SPDX-License-Identifier: GPL-3.0-or-later
// simpleserial-glitch.c from ChipWhisperer ported to Rust
// (c) Dennis Marttinen 2023

use pi_target::simpleserial::*;
use pi_target::trigger::Trigger;
// use std::sync::atomic::{compiler_fence, Ordering};
// use std::thread;
// use std::time::Duration;

extern "C" {
    fn glitch() -> i32;
}

fn main() {
    let mut trigger = Trigger::new(18);
    let ss = v2_1::SimpleSerial::new().unwrap();

    ss.process(|c, buf| {
        match c {
            'g' => {
                // let mut cnt = 0u32;
                trigger.high();
                // Loop ranges adjusted to account for the performance of the Raspberry Pi,
                // see https://www.youtube.com/watch?v=dVkCNiM0PL8
                // for _i in 0..10_000 {
                //     for _j in 0..10_000 {
                //         cnt += 1;
                //
                //         // This compiler fence avoids the optimizer deleting these loops when running in release mode, see
                //         // https://github.com/korken89/panic-halt/blob/6187afdc0bd3ac5d1b3dd26838fce36a88ecc138/src/lib.rs#L33C1-L33C1
                //         compiler_fence(Ordering::SeqCst);
                //     }
                //     thread::sleep(Duration::from_micros(10));
                // }
                let cnt = unsafe { glitch() };
                trigger.low();
                buf.clear();
                buf.extend_from_slice(&cnt.to_le_bytes());
                return (if cnt != 100_000_000 { 0x10 } else { 0x00 }, true);
            }
            cmd => panic!("unknown command: {}", cmd),
        }
    });
}
