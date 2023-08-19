// src/main.rs
// std and main are not available for bare metal software
#![no_std]
#![no_main]

use nb::block;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
// use cortex_m::asm;
use stm32f1xx_hal::{
    pac::{self, rcc::RegisterBlock}, 
    gpio::{Pin,Alternate},
    prelude::*,
    serial::{Config, Serial}
}; // STM32F1 hardware abstraction layer crate

// fn echo(serial: &mut Serial<USART1, (Pin<'A',9,Alternate>, Pin<'A',10>)>) {
//     let sent = b'X';
//     let received = block!(serial.rx.read()).unwrap();
//     hprintln!("Received: {}", received);
//     block!(serial.tx.write(sent)).unwrap();  
// }

#[entry]
fn main() -> ! {

    hprintln!("Starting program");

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain();
    
    // Get GPIO
    let mut gpioa = dp.GPIOA.split(); 
    let mut gpiob = dp.GPIOB.split();

    // Set up pin 13 as LED pin
    let mut pin13 = gpiob.pb13.into_push_pull_output(&mut gpiob.crh);
    // Manually set up 
    // Enable clock to GPIOB
    //rcc.apb2enr.write(|w| w.iopben().set_bit());

    // Choose which AF using this: eg for alternate function 7: let rx_pin = gpioa.pa10.into_alternate::<7>();
    // https://docs.rs/stm32f1xx-hal/latest/src/stm32f1xx_hal/gpio.rs.html#1-1206
    // pass in the control register, and get back the pin
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // TX should be alternate push pull since it can be driven by the alternate function output, not from output which comes from
    // output data register. Only outputs are set to push pull
    let pin_rx = gpioa.pa10; // have to use pa10 for UART1 because it is linked to RNXE bit and UART1 registers

    // If we were not using a HAL, we would have to
    // 1) Enable UART by setting UE bit
    // 2) Manually program number of stop bits
    // 3) Set desired baud rate
    // 4) Set RE bit USART_CR1
    // 5) When character received, RXNE bit is set
    let mut serial = Serial::new( // Serial uses Generics
        dp.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr, // alternate function map
        Config::default().baudrate(9600.bps()),
        &clocks,
    );

    loop {
        // echo
        let received = block!(serial.rx.read()).unwrap();
        hprintln!("Received: {}", received);

        match received {
            b'a' => {
                hprintln!("a");
                pin13.set_high();
            }
            b'b' => pin13.set_low(),
            _ => () // do nothing
        }

        block!(serial.tx.write(received)).unwrap();
    }

}