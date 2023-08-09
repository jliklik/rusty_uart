// src/main.rs
// std and main are not available for bare metal software
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
use stm32f1xx_hal::{
    pac::{self}, 
    prelude::*,
    serial::{Config, Serial}
}; // STM32F1 hardware abstraction layer crate

#[entry]
fn main() -> ! {

    // let p = cortex_m::Peripherals::take().unwrap();
    // let mut itm = p.ITM;
    // iprintln!(&mut itm.stim[0], "Hello, world!");
    hprintln!("The quick brown fox jumps over the lazy dog");

    let dp = pac::Peripherals::take().unwrap();
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain();
    let mut gpioa = dp.GPIOA.split();    
    
    // Choose which AF using this: eg for alternate function 7: let rx_pin = gpioa.pa10.into_alternate::<7>();
    // https://docs.rs/stm32f1xx-hal/latest/src/stm32f1xx_hal/gpio.rs.html#1-1206
    // pass in the control register, and get back the pin
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pin_rx = gpioa.pa10;

    let serial = Serial::new(
        dp.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(9600.bps())
            .wordlength_9bits()
            .parity_none(),
        &clocks,
    );

    // Separate into tx and rx channels
    let (mut tx, mut rx) = serial.split();

    loop {
        let received = (rx.read_u16()).unwrap();
        tx.write_u16(received).unwrap();
    }

}