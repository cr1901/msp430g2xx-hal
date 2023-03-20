#![no_std]

pub use embedded_hal as hal;

#[cfg(feature = "msp430g2553")]
pub use msp430g2553 as pac;

#[cfg(feature = "msp430g2211")]
pub use msp430g2211 as pac;

pub mod watchdog;
