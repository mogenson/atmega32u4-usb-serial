#![no_std]
#![feature(abi_avr_interrupt)]
#![allow(dead_code)]

use avr_device::atmega32u4::{PLL, USB_DEVICE};

extern "C" {
    /* general */
    fn usb_init();
    fn usb_configured() -> u8;

    /* receiving data */
    fn usb_serial_getchar() -> i16;
    fn usb_serial_available() -> u8;
    fn usb_serial_flush_input();

    /* transmitting data */
    fn usb_serial_putchar(c: u8) -> i8;
    fn usb_serial_putchar_nowait(c: u8) -> i8;
    fn usb_serial_write(buffer: *const u8, size: u16) -> i8;
    fn usb_serial_flush_output();

    /* serial parameters */
    fn usb_serial_get_baud() -> u32;
    fn usb_serial_get_stopbits() -> u8;
    fn usb_serial_get_paritytype() -> u8;
    fn usb_serial_get_numbits() -> u8;
    fn usb_serial_get_control() -> u8;
    fn usb_serial_set_control(signals: u8) -> i8;

    /* interrupt service routines */
    fn usb_gen_handler();
    fn usb_com_handler();
}

pub struct UsbSerial {
    usb: USB_DEVICE,
}

impl UsbSerial {
    pub fn new(usb: USB_DEVICE) -> Self {
        Self { usb }
    }

    pub fn init(&self, pll: &PLL) {
        self.usb.uhwcon.write(|w| w.uvrege().set_bit());
        self.usb
            .usbcon
            .write(|w| w.usbe().set_bit().frzclk().set_bit());
        pll.pllcsr.write(|w| w.pindiv().set_bit().plle().set_bit());
        while pll.pllcsr.read().plock().bit_is_clear() {}
        self.usb
            .usbcon
            .write(|w| w.usbe().set_bit().frzclk().clear_bit().otgpade().set_bit());
        self.usb.udcon.write(|w| w.detach().clear_bit());
        self.usb
            .udien
            .write(|w| w.eorste().set_bit().sofe().set_bit());

        unsafe {
            usb_init();
            avr_device::interrupt::enable();
            while usb_configured() == 0 {}
        }
    }

    pub fn get_available(&self) -> u8 {
        unsafe { usb_serial_available() }
    }

    pub fn get_dtr(&self) -> bool {
        unsafe { usb_serial_get_control() & 0x01 != 0 }
    }

    pub fn get_rts(&self) -> bool {
        unsafe { usb_serial_get_control() & 0x02 != 0 }
    }
}

impl embedded_hal::serial::Read<u8> for UsbSerial {
    type Error = ();

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        if unsafe { usb_serial_available() } > 0 {
            let data = unsafe { usb_serial_getchar() };
            if data != -1 {
                return Ok(data as u8);
            }
        }
        Err(nb::Error::WouldBlock)
    }
}

impl embedded_hal::serial::Write<u8> for UsbSerial {
    type Error = ();

    fn write(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        if unsafe { usb_serial_putchar(word) == 0 } {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn flush(&mut self) -> nb::Result<(), Self::Error> {
        unsafe { usb_serial_flush_output() };
        Ok(())
    }
}

impl ufmt::uWrite for UsbSerial {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        if unsafe { usb_serial_write(s.as_ptr(), s.len() as u16) == 0 } {
            Ok(())
        } else {
            Err(())
        }
    }
}

#[avr_device::interrupt(atmega32u4)]
unsafe fn USB_GEN() {
    avr_device::interrupt::free(|_| usb_gen_handler());
}

#[avr_device::interrupt(atmega32u4)]
unsafe fn USB_COM() {
    avr_device::interrupt::free(|_| usb_com_handler());
}
