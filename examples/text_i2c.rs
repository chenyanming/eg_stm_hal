//! Using crate ssd1306 to print with i2c on a generic ssd1306 based OLED display.
//!
//! Print "Hello world!" then "Hello rust!". Uses the `embedded_graphics` crate to draw.
//! Wiring pin connections for scl and sda to display as in the setup sections below.
//! Tested on generic (cheap) ssd1306 OLED 0.91" 128x32 and 0.96" 128x64 displays.
//! Note that the DisplaySize setting needs to be adjusted for 128x64 or 128x32 display
//!
//! This example based on 
//!    https://github.com/jamwaffles/ssd1306/blob/master/examples/text_i2c.rs
//! with stm32f4xx_hal setup following 
//!    https://github.com/stm32-rs/stm32f4xx-hal/blob/master/examples/ssd1306-image.rs
//!
//! Compare this example with oled_gps.


#![no_std]
#![no_main]

use cortex_m_rt::{entry, exception, ExceptionFrame};

//builtin include Font6x6, Font6x8, Font6x12, Font8x16, Font12x16, Font24x32
use embedded_graphics::{
    fonts::{Font8x16, Text}, 
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
   };
use panic_halt as _;

use ssd1306::{prelude::*, Builder, I2CDIBuilder};

#[cfg(feature = "stm32f1xx")]  //  eg blue pill stm32f103
use stm32f1xx_hal::{prelude::*,
                    pac::Peripherals, 
		    i2c::{BlockingI2c, DutyCycle, Mode},
		    gpio::{gpiob::{PB8, PB9}, Alternate, OpenDrain, },
		    device::I2C1,
		    };

#[cfg(feature = "stm32f3xx")]  //  eg Discovery-stm32f303
use stm32f3xx_hal::{prelude::*, 
                    pac::Peripherals,
                    i2c::{I2c, },  
		    gpio::{gpiob::{PB8, PB9}, AF4, },
		    pac::I2C1,
		    };

#[cfg(feature = "stm32f4xx")] // eg Nucleo-64, blackpills stm32f401 and stm32f411
use stm32f4xx_hal::{prelude::*,  
                    pac::Peripherals, 
                    i2c::{I2c, },  
		    gpio::{gpiob::{PB8, PB9}, AlternateOD, AF4, },
                    pac::I2C1,
		    }; 


#[cfg(feature = "stm32l1xx") ] // eg  Discovery STM32L100 and Heltec lora_node STM32L151CCU6
use stm32l1xx_hal::{prelude::*, 
                    stm32::Peripherals,
		    gpio::{gpiob::{PB8, PB9}, AlternateOD, AF4, },
                    stm32::I2C1,
                    };

#[entry]
fn main() -> ! {

    #[cfg(feature = "stm32f1xx")]
    fn setup() ->  BlockingI2c<I2C1,  (PB8<Alternate<OpenDrain>>, PB9<Alternate<OpenDrain>>) > {
  
       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();

       let clocks = rcc.cfgr.freeze(&mut  p.FLASH.constrain().acr);
       let mut afio = p.AFIO.constrain(&mut rcc.apb2);

       let mut gpiob = p.GPIOB.split(&mut rcc.apb2);
       
       // return i2c
       BlockingI2c::i2c1(
   	   p.I2C1,
   	   (gpiob.pb8.into_alternate_open_drain(&mut gpiob.crh),   // scl on PB8
	    gpiob.pb9.into_alternate_open_drain(&mut gpiob.crh)),  // sda on PB9
   	   &mut afio.mapr,
   	   Mode::Fast {
   	       frequency: 400_000.hz(),
   	       duty_cycle: DutyCycle::Ratio2to1,
   	   },
   	   clocks,
   	   &mut rcc.apb1,
   	   1000,
   	   10,
   	   1000,
   	   1000,
           )
       };

	   
    #[cfg(feature = "stm32f3xx")]
    fn setup() ->  I2c<I2C1, (PB8<AF4>, PB9<AF4>)> {
  
       let p = Peripherals::take().unwrap();
       let mut rcc = p.RCC.constrain();
       let clocks = rcc.cfgr.freeze(&mut  p.FLASH.constrain().acr);
       let mut gpiob = p.GPIOB.split(&mut rcc.ahb);

       let scl = gpiob.pb8.into_af4(&mut gpiob.moder, &mut gpiob.afrh);   // scl on PB8
       let sda = gpiob.pb9.into_af4(&mut gpiob.moder, &mut gpiob.afrh);   // sda on PB9
      
       // return i2c
       I2c::i2c1(p.I2C1, (scl, sda), 400_000.hz(), clocks, &mut rcc.apb1 )
       };


    #[cfg(feature = "stm32f4xx")]
    fn setup() ->  I2c<I2C1, (PB8<AlternateOD<AF4>>, PB9<AlternateOD<AF4>>)> {
  
       let  p  = Peripherals::take().unwrap();
       let rcc = p.RCC.constrain();
       let clocks = rcc.cfgr.freeze();
       let gpiob  = p.GPIOB.split();
       
       // could also have scl on PB6, sda on PB7
       //BlockingI2c::i2c1(
       let scl = gpiob.pb8.into_alternate_af4().set_open_drain();   // scl on PB8
       let sda = gpiob.pb9.into_alternate_af4().set_open_drain();   // sda on PB9
       
       // return i2c
       I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks)
       };


    #[cfg(feature = "stm32l1xx")]
    fn setup() -> I2c<I2C1, (PB8<AlternateOD<AF4>>, PB9<AlternateOD<AF4>>)> {
  
       let  p  = Peripherals::take().unwrap();
       //let rcc = p.RCC.constrain();
       let clocks = rcc.cfgr.freeze();
       let gpiob  = p.GPIOB.split();
       
       // could also have scl on PB6, sda on PB7
       //BlockingI2c::i2c1(
       let scl = gpiob.pb8.into_alternate_af4().set_open_drain();   // scl on PB8
       let sda = gpiob.pb9.into_alternate_af4().set_open_drain();   // sda on PB9
       
       // return i2c
       I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks)
       };


    // End of hal/MCU specific setup. Following should be generic code.


    let i2c = setup();
    
    let interface = I2CDIBuilder::new().init(i2c);
    let mut disp: GraphicsMode<_, _> = Builder::new()
                    .size(DisplaySize128x64)        // set display size 128x32, 128x64
		    .connect(interface)
		    .into();
    disp.init().unwrap();

    //builtin include Font6x6, Font6x8, Font6x12, Font8x16, Font12x16, Font24x32
    let text_style = TextStyleBuilder::new(Font8x16) 
        .text_color(BinaryColor::On)
        .build();

    Text::new("Hello world!", Point::zero())
        .into_styled(text_style)
        .draw(&mut disp)
        .unwrap();

    Text::new("Hello Rust!", Point::new(0, 20))
        .into_styled(text_style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}