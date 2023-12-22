# chipwhisperer-pi

Software emulation of [ChipWhisperer firmware](https://github.com/newaetech/chipwhisperer/tree/develop/hardware/victims/firmware) written in Rust. Intended to be executed as an application on a Raspberry Pi 4. Currently, (partially) implements the following firmwares:

- [`simpleserial-aes`](https://github.com/newaetech/chipwhisperer/blob/develop/hardware/victims/firmware/simpleserial-aes/simpleserial-aes.c) (SS v1.1)
  - TINYAES128C variant, AES component implemented in C
- [`simpleserial-glitch`](https://github.com/newaetech/chipwhisperer/blob/develop/hardware/victims/firmware/simpleserial-glitch/simpleserial-glitch.c) (SS v2.1)
  - Iterative summation implemented in either C or Rust (switch manually)

## Building

```shell
cargo run --release --bin simpleserial-aes
cargo run --release --bin simpleserial-glitch
```

## Authors

- Dennis Marttinen ([@twelho](https://github.com/twelho))

## License

[GPL-3.0-or-later](https://spdx.org/licenses/GPL-3.0-or-later.html) ([LICENSE](LICENSE))
