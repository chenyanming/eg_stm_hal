//! String serial interface loopback NOT YET    test on usart2 pins pa2, pa3.
//!
//! Short the TX and RX pins  pa2 to pa3.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
//use cortex_m::asm;
use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
//use core::fmt::Write;

use cortex_m_semihosting::hprintln;

use eg_stm_hal::to_str;

//  eg blue pill stm32f103
#[cfg(feature = "stm32f103")]
use stm32f1xx_hal::{ prelude::*,  pac,  serial::{Config, Serial}, };

//  eg Discovery-stm32f303
//use alt_stm32f30x_hal::{  ??
#[cfg(feature = "stm32f303")]
use stm32f3xx_hal::{ prelude::*, pac, serial::{Config, Serial, StopBits}, };

// eg Nucleo-64  stm32f411
#[cfg(feature = "stm32f411")]
use stm32f4xx_hal::{ prelude::*, pac, serial::{Config, Serial, StopBits}, };

// eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
#[cfg(any(feature = "stm32l100",   feature = "stnm32l151" )) ]
use stm32l1xx_hal::{ prelude::*, pac, serial::{Config, Serial, StopBits}, };


#[entry]
fn main() -> ! {

    //see examples/serial_loopback_char_test.rs for more notes regarding this setup.
    let p = pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    // let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    let channels = p.DMA1.split(&mut rcc.ahb);

    //      alternately       .baudrate(115_200.bps())
    let serial = Serial::usart2(
        p.USART2,
        (gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl),   gpioa.pa3),
        &mut afio.mapr,
        Config::default().baudrate(9600.bps()),
        clocks,
        &mut rcc.apb1,
    );

    // Split the serial struct into a receiving and a transmitting part
    let (tx2, mut rx2) = serial.split();
    let rx2 = rx2.with_dma(channels.6);
    let tx2 = tx2.with_dma(channels.7);    

    //let send = b"The quick brown fox";
    let send = b"The quick ";
    //writeln!(tx2, "{:?}", send).unwrap();   no method 
    tx2.write(send).wait();
    //let (_, tx2) = tx2.write(b" jumps").wait();

    hprintln!("sent {:?}. Finished send.", send).unwrap();

    // Read what was just sent. Blocks until the read is complete
    let buf = singleton!(: [u8; 10] = [0; 10]).unwrap();
    // let (rcvd, rx2) = rx2.read(buf).wait(); //  runtime problem. stalls waiting
    // let rcvd = rx2.read().wait();           //  compile problem. Can't print  rcvd
        //  structure  `stm32f1xx_hal::dma::RxDma<stm32f1xx_hal::serial::Rx<stm32f1::stm32f103::USART2>, stm32f1xx_hal::dma::dma1::C6>`
    let rcvd = rx2.read(buf);
    //let rcvd = block!(rx2.read()).unwrap();

    hprintln!("finished receive. ").unwrap();
    asm::bkpt();
    hprintln!("received {:?} ", rcvd).unwrap();

    hprintln!("received {:?}", to_str(rcvd)).unwrap();

    // With tx and rx connected sent should equal received
    //assert_eq!(rcvd, send, "testing rcvd = send,  {} = {}", rcvd, send);

    // PUT A TEST HERE THAT WILL SHOW FAILURE. ASSERT SEEMS TO PANIC HALT SO ...

    // breakpoint to inspect
    asm::bkpt();

    loop {}
}