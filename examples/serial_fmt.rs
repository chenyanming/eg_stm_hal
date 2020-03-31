//! Formatted string serial interface write with usart1 to serial-usb converter on pin pa9 (rx is pa10).
//!
//! Connect the Tx pin pa9 to the Rx pin of a serial-usb converter
//! so you can see the message in a serial console (e.g. minicom).
//! set for 9600bps, could be higher but needs serial console to be the same.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

use cortex_m_rt::entry;
use core::fmt::Write;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial, config::StopBits}};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };


#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let p = Peripherals::take().unwrap();

    // Take ownership of raw flash and rcc devices and convert to HAL structs
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // Freeze  all system clocks  and store the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Prepare the alternate function I/O registers
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // Prepare the GPIOB peripheral
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    // let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    //see examples/serial_loopback_char.rs for more USART config notes.

    let serial = Serial::usart1(
        p.USART1,
        (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),  gpioa.pa10),
        &mut afio.mapr,
        Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
         clocks,
        &mut rcc.apb2,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (mut tx, _rx) = serial.split();

    let number = 42;
    writeln!(tx, "\r\nHello {}. Converted number set to 42.\r\n", number).unwrap();

    loop {}
}
