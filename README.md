# Atmega32u4 USB Serial Crate

This is a Rust crate for the Atmega32u4 microcontroller. It implements a CDC ACM USB serial device using the [Teensy AVR C library](https://www.pjrc.com/teensy/usb_serial.html).

This crate provides a `UsbSerial` struct that implements the [embedded_hal serial](https://docs.rs/embedded-hal/0.2.4/embedded_hal/serial/index.html) `Read` and `Write` traits. It also implements the [ufmt](https://docs.rs/ufmt/0.1.0/ufmt/trait.uWrite.html) `uWrite` trait.

## To build

Use a nightly compiler version with AVR support by appending `+nightly` to each `cargo` command, or running `rustup override set nightly` once in the crate directory.

Run `cargo build`.

## To use

To upload the `echo.rs` example to a connected Arduino Leonardo board, run `cargo run --example echo`. Open the virtual serial port with a serial terminal and enter lowercase characters. They should be repeated back in uppercase.

Look at `src/lib.rs` and `examples/echo.rs` to see how to use the available USB serial methods.
