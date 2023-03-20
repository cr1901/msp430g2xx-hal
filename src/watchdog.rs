use core::convert::Infallible;

use super::pac;

pub struct WatchdogTimer {
    inner: pac::WATCHDOG_TIMER,
}

impl WatchdogTimer {
    pub fn new(inner: pac::WATCHDOG_TIMER) -> Self {
        WatchdogTimer { inner }
    }

    pub fn start(self, period: u8) -> Result<Self, Infallible> {
        self.inner.wdtctl.write(|w| {
            w.wdtpw()
                .password()
                .wdthold()
                .clear_bit()
                .wdtis()
                .bits(period)
        });

        Ok(self)
    }

    pub fn disable(self) -> Result<Self, Infallible> {
        self.inner
            .wdtctl
            .write(|w| w.wdtpw().password().wdthold().set_bit());
        Ok(self)
    }

    pub fn feed(&mut self) -> Result<(), Infallible> {
        self.inner
            .wdtctl
            .write(|w| w.wdtpw().password().wdtcntcl().set_bit());
        Ok(())
    }
}
