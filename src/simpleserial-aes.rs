// SPDX-License-Identifier: GPL-3.0-or-later
// simpleserial-aes.c (for SS v1.1) from ChipWhisperer ported to Rust
// (c) Dennis Marttinen 2023

use pi_target::simpleserial::*;
use pi_target::trigger::Trigger;

// Default key from aes-independant.h
const DEFAULT_KEY: &[u8] = &[
    0x2b, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf, 0x4f, 0x3c,
];

extern "C" {
    fn AES128_ECB_indp_setkey(key: *const u8);
    fn AES128_ECB_indp_crypto(input: *mut u8);
}

fn safe_aes128_ecb_indp_setkey(buf: &[u8]) {
    assert_eq!(buf.len(), 16, "AES buffer has invalid length");
    unsafe { AES128_ECB_indp_setkey(buf.as_ptr()) }
}

fn safe_aes128_ecb_indp_crypto(buf: &mut [u8]) {
    assert_eq!(buf.len(), 16, "AES buffer has invalid length");
    unsafe { AES128_ECB_indp_crypto(buf.as_mut_ptr()) }
}

fn main() {
    safe_aes128_ecb_indp_setkey(DEFAULT_KEY);

    let mut trigger = Trigger::new(18);
    let ss = v1_1::SimpleSerial::new().unwrap();

    ss.process(|c, buf| {
        match c {
            'k' => safe_aes128_ecb_indp_setkey(buf),
            'p' => {
                //aes_indep_enc_pretrigger(pt);

                trigger.high();

                safe_aes128_ecb_indp_crypto(buf); /* encrypting the data block */
                trigger.low();

                //aes_indep_enc_posttrigger(pt);

                // Handled by setting the modification flag on return
                //simpleserial_put('r', 16, pt);
                return (0, true);
            }
            'x' => (), // This does nothing in the reference implementation
            cmd => panic!("unknown command: {}", cmd),
        }

        (0, false)
    });
}
