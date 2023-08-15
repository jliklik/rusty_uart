// src/main.rs
// std and main are not available for bare metal software
#![no_std]
#![no_main]

use nb::block;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use cortex_m::asm;
use stm32f1xx_hal::{
    pac::{self, USART1}, 
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
        Config::default()
            .baudrate(9600.bps())
            .wordlength_9bits()
            .parity_none(),
        &clocks,
    );

    //let usart1: &mut USART1 = unsafe { &mut *(USART1::ptr() as *mut _) };
    // Separate into tx and rx channels
    //let (mut tx, mut rx) = serial.split();

    hprintln!("Entering loop!");

    let sent = b'X';
    block!(serial.tx.write(sent)).unwrap();

    // Read the byte that was just sent. Blocks until the read is complete
    let received = block!(serial.rx.read()).unwrap();

    // Since we have connected tx and rx, the byte we sent should be the one we received
    assert_eq!(received, sent);
    hprintln!("Sent: {}", sent);
    hprintln!("Received: {}", received);

    // Trigger a breakpoint to allow us to inspect the values
    asm::bkpt();

    loop {
        // hprintln!("Inside loop!");
        //let received = (rx.read_u16()).unwrap(); // this blocks
        //tx.write_u16(received).unwrap();

        // while usart1.sr.read().rxne().bit_is_clear() {}
        // let byte = usart1.dr.read().dr().bits() as u8;

        let byte = block!(serial.rx.read()).unwrap();

        hprintln!("Received: {}", byte);
        
    }

}