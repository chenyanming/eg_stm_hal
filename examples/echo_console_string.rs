//! Serial DMA RX transfer. Read 15 chars input from console on USART1, echo back to console, 
//!  and output to semihost. Repeat.
//! 
//! See examples/serial_char.rs for notes about connecting usart1 to 
//!   serial ttl-usb converter on computer for console output.
//! That file also has more notes regarding setup.

#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[cfg(debug_assertions)]
extern crate panic_semihosting;

#[cfg(not(debug_assertions))]
extern crate panic_halt;

use cortex_m::singleton;
use cortex_m_rt::entry;
use core::fmt::Write;  // for writeln, but not supported by stm32f3xx_hal
use cortex_m_semihosting::hprintln;
//use nb::block;

use eg_stm_hal::to_str;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   
                    pac::Peripherals, 
                    serial::{Config, Serial, StopBits, Tx, Rx},
		    dma::{RxDma, dma1::{C5}},     //TxDma,  C4, 
		    device::USART1 }; 

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    stm32::Peripherals,
		    serial::{Serial, Tx, Rx}, 
		    stm32::USART1 
		    };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, 
                    pac::Peripherals, 
		    serial::{config::Config, Serial, Tx, Rx},
		    pac::USART1 
		    };

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
                    stm32::Peripherals, 
		    serial::{Config, Serial, Tx, Rx},
		    stm32::USART1 
		    };


#[entry]
fn main() -> ! {
     
    //see serial_char.rs and  echo_console_by_char.rs for additional comments.
    
    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  (Tx<USART1>, RxDma<Rx<USART1>, C5>)  {
       
       // with TxDma return    TxDma<Tx<USART1>, C4>
       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr); 
       let mut afio = p.AFIO.constrain(&mut rcc.apb2);
       let mut gpioa = p.GPIOA.split(&mut rcc.apb2);

       //let (tx1, rx1) = Serial::usart1(
       let txrx1 = Serial::usart1(
	   p.USART1,
	   (gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh),     //tx pa9, 
            gpioa.pa10),					    //rx pa10
	   &mut afio.mapr,
	   Config::default() .baudrate(9600.bps()) .stopbits(StopBits::STOP1),
	   clocks,
	   &mut rcc.apb2,
	   );  //.split();

       let channels = p.DMA1.split(&mut rcc.ahb);
       let (tx1, rx1)  = txrx1.split();
       //let tx1 = tx1.with_dma(channels.4);
       let rx1 = rx1.with_dma(channels.5);
       (tx1, rx1)
       };



    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>)  {

       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks    = rcc.cfgr.freeze(&mut p.FLASH.constrain().acr);
       let mut gpioa = p.GPIOA.split(&mut rcc.ahb); 

       let txrx1 = Serial::usart1(
    	   p.USART1,
    	   (gpioa.pa9.into_af7( &mut gpioa.moder, &mut gpioa.afrh), 
 	    gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh)),
    	   9600.bps(),
    	   clocks,
    	   &mut rcc.apb2,
           );

       let (mut tx1, mut rx1)  = txrx1.split();
       (tx1, rx1)
       };


    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>)  {

       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks = rcc.cfgr.freeze();
       let gpioa = p.GPIOA.split();
       p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 

       let txrx1 =  Serial::usart1(
          p.USART1,
             (gpioa.pa9.into_alternate_af7(), 
	      gpioa.pa10.into_alternate_af7()), 
    	  Config::default() .baudrate(9600.bps()),
    	  clocks,
          ).unwrap();    

       let (mut tx1, mut rx1)  = txrx1.split();
       (tx1, rx1)
       };


    #[cfg(feature = "stm32l1xx")]
    fn setup() ->  (Tx<USART1>, Rx<USART1>)  {

       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();  
       let clocks = rcc.cfgr.freeze();
       let gpioa = p.GPIOA.split();
       p.USART1.cr1.modify(|_,w| w.rxneie().set_bit());  //need RX interrupt? 
       let txrx1 =  Serial::usart1(
          p.USART1,
    	  (gpioa.pa9.into_alternate_af7(),
    	   gpioa.pa10.into_alternate_af7()), 
    	  Config::default() .baudrate(9600.bps()),
    	  clocks,
          ).unwrap();    
 
       let (mut tx1, mut rx1)  = txrx1.split();
       (tx1, rx1)
       };


    // End of hal/MCU specific setup. Following should be generic code.

    let (mut tx1, rx1) = setup();

    let mut rx1buf = (singleton!(: [u8; 15] = [0; 15]).unwrap(), rx1);
    //let mut tx1buf = (singleton!(: [u8; 15] = [0; 15]).unwrap(), tx1);

    hprintln!("test write to console ...").unwrap();

    // writeln! does not work with TxDma
    writeln!(tx1, "\r\ncheck console output.\r\n").unwrap();
    // and without dma next expects u8 not &[u8; 25]
    //let (_, tx1) = tx1.write(b"\r\ncheck console output.\r\n").wait();
    //let tx1buf = tx1buf.1.write(b"\r\ncheck console output.\r\n").wait();


    hprintln!("Enter 15 characters in console. Repeat.").unwrap();
    hprintln!("Use ^C in gdb to exit.").unwrap();

    writeln!(tx1, "\r\nEnter 15 characters here in the console. Repeat.\r\n").unwrap();
    //let (_, tx1) =  tx1.write(b"\r\nEnter 15 characters below. Repeat.\r\n").wait();
    //let tx1buf = tx1buf.1.write(b"\r\nEnter 15 characters below. Repeat.\r\n").wait();

    // cannot get loop to work with tuple (buf, rx1), there seem to be circular problems
    // with move/borrow/mut  but rx1buf structure works ...

    //each pass in loop waits for input of 15 chars typed in console
    loop { 
       rx1buf = rx1buf.1.read(rx1buf.0).wait();
       hprintln!("received {:?}", to_str(rx1buf.0)).unwrap();
       //tx1buf = tx1buf.1.write(rx1buf.0).wait();
       //tx1.write(rx1buf.0).wait();
       writeln!(tx1, "{}\r", to_str(rx1buf.0)).unwrap();
       }
}
