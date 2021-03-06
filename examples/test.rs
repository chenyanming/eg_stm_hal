//! Just misc testing

#![deny(unsafe_code)]
//#![no_main]
//#![no_std]

//#[cfg(debug_assertions)]
//extern crate panic_semihosting;
//
//#[cfg(not(debug_assertions))]
//extern crate panic_halt;

//use cortex_m::asm;
//use cortex_m_rt::entry;
//use core::fmt::Write;
//use cortex_m_semihosting::hprintln;
//use core::str;
//use nb::block;

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,   pac::Peripherals, serial::{Config, Serial, StopBits}, };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, stm32::Peripherals, serial::{ Serial}, };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64  stm32f411
use stm32f4xx_hal::{prelude::*, pac::Peripherals, serial::{config::Config, Serial }};

#[cfg(feature = "stm32l1xx") ] // eg  Discovery kit stm32l100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, stm32::Peripherals, serial::{config::Config, Serial }};


//#[entry]
//fn main() -> ! {

struct Foo<F>
where
    F: Fn(usize) -> usize,
{
    pub foo: F,
}

impl<F> Foo<F>
where
    F: Fn(usize) -> usize,
{
    fn new(foo: F) -> Self {
        Self { foo }
    }
}

fn main() {
    let foo = Foo { foo: |a| a + 1 };
    (foo.foo)(42);
    println!("{}", (foo.foo)(42));

    (Foo::new(|a| a + 1).foo)(42);

    println!("{}", (Foo::new(|a| a + 1).foo)(42));
}


//    loop {}
//}
