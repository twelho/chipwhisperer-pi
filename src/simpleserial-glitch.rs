// SPDX-License-Identifier: GPL-3.0-or-later
// simpleserial-glitch.c from ChipWhisperer ported to Rust
// (c) Dennis Marttinen 2023

use std::sync::atomic::{compiler_fence, Ordering};
use pi_target::{SimpleSerial, Trigger};

fn main() {
    let mut trigger = Trigger::new(18);

    SimpleSerial::new().unwrap().process(|c, buf| {
        match c {
            'g' => {
                let mut cnt = 0u32;
                trigger.high();
                for _i in 0..50 { // TODO: These counts need updating...
                    for _j in 0..50 {
                        cnt += 1;

                        // This compiler fence should hopefully avoid optimization of the loops, see
                        // https://github.com/korken89/panic-halt/blob/6187afdc0bd3ac5d1b3dd26838fce36a88ecc138/src/lib.rs#L33C1-L33C1
                        compiler_fence(Ordering::SeqCst);
                    }
                }
                trigger.low();
                buf.clear();
                buf.extend_from_slice(&cnt.to_le_bytes()); // TODO: LE or BE?
                //simpleserial_put('r', 4, (uint8_t *) & cnt);
                return ((cnt != 2500) as u8, true);
            }
            cmd => panic!("unknown command: {}", cmd),
        }
    });
}
