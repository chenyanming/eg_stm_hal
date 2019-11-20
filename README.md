# eg_stm_hal
Newbie notes - examples using embedded Rust HAL

(My development environment is Linux, so these notes are specific to that 
in many places. However, the examples should work in other development 
environments. Just the setup may change.)

##  Contents
- [Status](#status-summary)
- [Links](#Links)
- [This Package Setup](#this-package-setup)
- [Notes on the Examples](#notes-on-the-examples)
- [License](#License)
- [Contribution](#Contribution)
- [](## )


##  Status Summary (November 19, 2019)
work in progress ...

The overall Travis CI build status is [![Build Status](https://travis-ci.org/pdgilbert/eg_stm_hal.svg?branch=master)](https://travis-ci.org/pdgilbert/eg_stm_hal)
Status for [individual boards can be seen at Travis CI.](https://travis-ci.org/pdgilbert/eg_stm_hal)
Testing if the code runs and does something resembling what it is supposed to do requires hardware and
is not as automatic as CI. This is my summary as of November 2019.

|      HAL       |    MCU    |      Board          |   Builds   |  Runs  |          Notes                            |
| -------------- |:---------:|:-------------------:|:----------:|:------:| :---------------------------------------- |
| stm32f1xx-hal  | stm32f103 |      bluepill       |   mostly   |  some  | Problems using serial.                    |
| stm32f3xx-hal  | stm32f303 | discovery-stm32f303 |    no      |   no   | Hal differences. no `pac` in the root, ...|
| stm32f4xx-hal  | stm32f411 |      nucleo-64      |    no      |   no   | Hal differences. no `pac` in the root, ...|
| stm32l1xx-hal  | stm32l100 | discovery-stm32l100 |    no      |   no   | Hal does not build.                       |
| stm32l1xx-hal  | stm32l151 | heltec-lora-node151 |    no      |   no   | Hal does not build.                       |


|   HAL git       |      HAL Travis CI  Status         | 
| -----------------------  |:---------------------------:|
| [stm32f1xx-hal](https://github.com/stm32-rs/stm32f1xx-hal) | [![Build Status](https://api.travis-ci.org/stm32-rs/stm32f1xx-hal.svg?branch=master)]  (https://api.travis-ci.org/stm32-rs/stm32f1xx-hal) |
| [stm32f3xx-hal](https://github.com/stm32-rs/stm32f3xx-hal) | [![Build Status](https://api.travis-ci.org/stm32-rs/stm32f3xx-hal.svg?branch=master)]  (https://api.travis-ci.org/stm32-rs/stm32f3xx-hal) |
| [stm32f4xx-hal](https://github.com/stm32-rs/stm32f4xx-hal) | [![Build Status](https://api.travis-ci.org/stm32-rs/stm32f4xx-hal.svg?branch=master)]  (https://api.travis-ci.org/stm32-rs/stm32f4xx-hal) |
| [stm32l1xx-hal](https://github.com/stm32-rs/stm32l1xx-hal) | [![Build Status](https://api.travis-ci.org/stm32-rs/stm32l1xx-hal.svg?branch=master)]  (https://api.travis-ci.org/stm32-rs/stm32l1xx-hal) |

##  Links
- [HALs on Github](https://github.com/stm32-rs) and on [Travis CI.](https://travis-ci.org/stm32-rs)
- The CI for several rust embedded projects is [here.](https://travis-ci.org/rust-embedded)

##  This Package Setup
I am trying to have a common code base of examples that run on different boards.
(This may be wishful thinking.) I have still not decided the best way to 
organize this for Cargo. Workspaces do not seem to be intended for this.
My current setup is to have common files src/, examples/, ..., at the top level.
Then, under boards/,  use soft links to the common files.

You can get this package from Github with 
```
git clone https://github.com/pdgilbert/eg_stm_hal.git
```

##  Notes on the Examples

To build the examples use
```rust
cargo build  --target $TARGET  --features $MCU --example xxx
```
where TARGET and MCU are environment variables for your processor. Boards indicated
above use one of 
```
  export MCU=stm32f103 TARGET=thumbv7m-none-eabi     #  bluepill Cortex-M3
  export MCU=stm32f303 TARGET=thumbv7em-none-eabihf  # STM32F303 Cortex-M3
  export MCU=stm32f411 TARGET=thumbv7em-none-eabihf  # nucleo-64
  export MCU=stm32l151 TARGET=thumbv7m-none-eabi     # heltec-lora-node151 Cortex-M3
```
To run the examples in a separate windows do
```
minicom -D /dev/ttyUSB0 -b9600  #
openocd -f interface/$INTERFACE.cfg -f target/$PROC.cfg  #
```

```
cargo  run --target $TARGET --features $MCU --example xxx
```

| xxx                     | notes |   Description
| ----------------------- |:-----:|---------------------------------------------------------------:|
| blink                   |   *1  | Blinks off-board LEDs                                          |
| serial_loopback_char    |   *2  | Single character loopback + semihost output                    |
| serial_fmt              |       | Formatted string  write to console                             |
| serial-dma-tx           |       | String writes to terminal serial interface                     |
| serial_pass_thru_string |       | Read 15 chars input from console, output to semihost, repeat   |
| serial_loopback_string  |       | String serial interface loopback  + semihost output            |
| echo_by_char            |       | Echo back console input, char by char,  + semihost output      |
| serial_gps_rw           |   *3  | Read by str from GPS with echo to console + semihost output    |
| serial_gps_rw_by_char   |       | Read by char from GPS with echo to console + semihost output   |
| serial_cross            |       | Str write from one usart and read on another + semihost output |


*1/  Some of the tests needs git version of stm32f1xx-hal (in Oct 2019),
*2/  Blink_test does not blink in gdb steps, use continue.
*3/  With local echo on in terminal the characters are double <cr> gets a single <lf>.
     Without local echo there is no <lf>. trouble if you type too fast
*4/  Ublox GPS by default uses 9600bps, odd Parity, 1 stop bit (minicom 8-N1). Can be checked
      by direcstly connecting computer through usb-serial to GPS, skipping bluepill. (5v on 
      usb-serial seemed to be preferred for power.)


|  xxx                   |   bluepill    |    
                         | build |  run  | 
| ---------------------- |:-----:|:-----:|
|blink                   | yes   | works | 
|serial_loopback_char    | yes   | works | 
|serial_fmt              | yes   | works | 
|serial-dma-tx           | no    |       |
|serial_pass_thru_string | yes   | works | 
|serial_loopback_string  | yes   |       |
|echo_by_char            | yes   | works | 
|serial_gps_rw           | yes   |       |
|serial_gps_rw_by_char   | yes   |       |
|serial_cross            | yes   |       |           


## Misc Notes on STlink and OpenOCD

```
  export  PROC=stm32f1x  # bluepill
  export  PROC=stm32l1x  # discovery-stm32l100
```

```
  export INTERFACE=stlink-v2   #  WaveGat dongle
  export INTERFACE=stlink-v2   #  STlink on Discovery STM32L100
  export INTERFACE=stlink-v2-1 #  STlink on Discovery STM32F303
```


## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
