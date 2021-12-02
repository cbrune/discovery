#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let tick = 50_u16;
    let mut iter = 0;

    let mut on_mask = [0_u16; 16];
    on_mask[0] = 0xC001;

    for i in 1..15 {
        on_mask[i] = 0x3 << (i - 1);
    }

    loop {
        for led in 0..8 {
            if (on_mask[led] & (1 << iter)) != 0 {
                leds[led].on().ok();
            } else {
                leds[led].off().ok();
            }
        }
        delay.delay_ms(tick);
        iter = (iter + 1) % 8;
    }
}
