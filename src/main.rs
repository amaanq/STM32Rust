// Blink an LED

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;

use hal::rcc::{HSEClock, HSEClockMode};
use stm32f7xx_hal as hal;

use crate::hal::{pac, prelude::*};
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    // let gpioc = p.GPIOC.split();
    let gpiod = p.GPIOD.split();
    let gpioj = p.GPIOJ.split();

    // inputs
    // let c6 = gpioc.pc6.into_pull_up_input();
    // let c7 = gpioc.pc7.into_pull_up_input();
    // let j1 = gpioj.pj0.into_pull_up_input();
    // let f6 = gpioa.pa0.into_pull_up_input();

    // outputs
    let mut ld1 = gpioj.pj13.into_push_pull_output();
    let mut ld2 = gpioj.pj5.into_push_pull_output();
    let mut ld3 = gpioa.pa12.into_push_pull_output();
    let mut ld4 = gpiod.pd4.into_push_pull_output();

    let rcc = p.RCC.constrain();
    let clocks = rcc
        .cfgr
        .hse(HSEClock::new(25.MHz(), HSEClockMode::Bypass))
        .sysclk(48.MHz())
        .freeze();

    let mut delay = p.TIM5.delay_us(&clocks);

    loop {
        ld1.set_low();
        ld2.set_low();
        ld3.set_low();
        ld4.set_high();
        delay.delay_ms(500_u32);

        ld1.set_high();
        ld2.set_high();
        ld3.set_high();
        ld4.set_low();
        delay.delay_ms(500_u32);

        // turn each light on one after another really fast (50ms? 100ms?) then turn them off in reverse order

        ld1.set_low();
        delay.delay_ms(50_u32);
        ld2.set_low();
        delay.delay_ms(50_u32);
        ld3.set_low();
        delay.delay_ms(50_u32);
        ld4.set_high();
        delay.delay_ms(200_u32);

        ld4.set_low();
        delay.delay_ms(50_u32);
        ld3.set_high();
        delay.delay_ms(50_u32);
        ld2.set_high();
        delay.delay_ms(50_u32);
        ld1.set_high();
        delay.delay_ms(200_u32);
    }

    // loop {
    //     if c7.is_low() {
    //         ld1.set_low();
    //     } else {
    //         ld1.set_high();
    //     }
    //
    //     if c6.is_low() {
    //         ld2.set_low();
    //     } else {
    //         ld2.set_high();
    //     }
    //
    //     if j1.is_low() {
    //         ld3.set_low();
    //     } else {
    //         ld3.set_high();
    //     }
    //
    //     if f6.is_low() {
    //         ld4.set_low();
    //     } else {
    //         ld4.set_high();
    //     }
    // }
}

#[panic_handler]
const fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
