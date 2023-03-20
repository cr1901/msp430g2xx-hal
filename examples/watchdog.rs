#![no_main]
#![no_std]
#![feature(abi_msp430_interrupt)]

use msp430_rt::entry;
use msp430g2xx_hal::pac::Peripherals;
use msp430g2xx_hal::watchdog::WatchdogTimer;

extern crate panic_msp430;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    WatchdogTimer::new(p.WATCHDOG_TIMER).disable().unwrap();

    loop {}
}
