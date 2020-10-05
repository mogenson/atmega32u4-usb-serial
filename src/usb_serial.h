#ifndef usb_serial_h__
#define usb_serial_h__

#include <stdint.h>

// setup
void usb_init(void);          // initialize everything
uint8_t usb_configured(void); // is the USB port configured

// receiving data
int16_t usb_serial_getchar(void);   // receive a character (-1 if timeout/error)
uint8_t usb_serial_available(void); // number of bytes in receive buffer
void usb_serial_flush_input(void);  // discard any buffered input

// transmitting data
int8_t usb_serial_putchar(uint8_t c); // transmit a character
int8_t usb_serial_putchar_nowait(uint8_t c); // transmit a character, do not wait
int8_t usb_serial_write(const uint8_t *buffer,
                        uint16_t size); // transmit a buffer
void usb_serial_flush_output(void); // immediately transmit any buffered output

// serial parameters
uint32_t usb_serial_get_baud(void);      // get the baud rate
uint8_t usb_serial_get_stopbits(void);   // get the number of stop bits
uint8_t usb_serial_get_paritytype(void); // get the parity type
uint8_t usb_serial_get_numbits(void);    // get the number of data bits
uint8_t usb_serial_get_control(void);    // get the RTS and DTR signal state
int8_t usb_serial_set_control(uint8_t signals); // set DSR, DCD, RI, etc

// interrupt service routines
void usb_gen_handler(void);
void usb_com_handler(void);

// constants corresponding to the various serial parameters
#define USB_SERIAL_DTR 0x01
#define USB_SERIAL_RTS 0x02
#define USB_SERIAL_1_STOP 0
#define USB_SERIAL_1_5_STOP 1
#define USB_SERIAL_2_STOP 2
#define USB_SERIAL_PARITY_NONE 0
#define USB_SERIAL_PARITY_ODD 1
#define USB_SERIAL_PARITY_EVEN 2
#define USB_SERIAL_PARITY_MARK 3
#define USB_SERIAL_PARITY_SPACE 4
#define USB_SERIAL_DCD 0x01
#define USB_SERIAL_DSR 0x02
#define USB_SERIAL_BREAK 0x04
#define USB_SERIAL_RI 0x08
#define USB_SERIAL_FRAME_ERR 0x10
#define USB_SERIAL_PARITY_ERR 0x20
#define USB_SERIAL_OVERRUN_ERR 0x40

// Everything below this point is only intended for usb_serial.c
#ifdef USB_SERIAL_PRIVATE_INCLUDE
#include <avr/interrupt.h>
#include <avr/io.h>
#include <avr/pgmspace.h>

#define EP_TYPE_CONTROL 0x00
#define EP_TYPE_BULK_IN 0x81
#define EP_TYPE_BULK_OUT 0x80
#define EP_TYPE_INTERRUPT_IN 0xC1
#define EP_TYPE_INTERRUPT_OUT 0xC0
#define EP_TYPE_ISOCHRONOUS_IN 0x41
#define EP_TYPE_ISOCHRONOUS_OUT 0x40
#define EP_SINGLE_BUFFER 0x02
#define EP_DOUBLE_BUFFER 0x06
#define EP_SIZE(s)                                                             \
  ((s) == 64 ? 0x30 : ((s) == 32 ? 0x20 : ((s) == 16 ? 0x10 : 0x00)))

#define MAX_ENDPOINT 4

#define LSB(n) (n & 255)
#define MSB(n) ((n >> 8) & 255)

// standard control endpoint request types
#define GET_STATUS 0
#define CLEAR_FEATURE 1
#define SET_FEATURE 3
#define SET_ADDRESS 5
#define GET_DESCRIPTOR 6
#define GET_CONFIGURATION 8
#define SET_CONFIGURATION 9
#define GET_INTERFACE 10
#define SET_INTERFACE 11
// CDC (communication class device)
#define CDC_SET_LINE_CODING 0x20
#define CDC_GET_LINE_CODING 0x21
#define CDC_SET_CONTROL_LINE_STATE 0x22
#endif
#endif
