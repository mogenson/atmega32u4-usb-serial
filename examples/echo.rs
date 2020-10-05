#![no_std]
#![no_main]

use arduino_leonardo::prelude::*;
use atmega32u4_usb_serial::UsbSerial;
use panic_halt as _;

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();
    let mut usb = UsbSerial::new(dp.USB_DEVICE);
    usb.init(&dp.PLL);

    ufmt::uwriteln!(usb, "Hello, World!").unwrap();

    loop {
        let mut byte = nb::block!(usb.read()).unwrap();
        byte = (byte as char).to_ascii_uppercase() as u8;
        nb::block!(usb.write(byte)).unwrap();
    }
}
