use embedded_hal::digital::StatefulOutputPin;

pub struct Led<P> {
    pin: P,
}

impl<P: StatefulOutputPin> Led<P> {
    /// Initializes the Led with the Gpio output pin
    pub fn new(pin: P) -> Self {
        Self { pin }
    }

    /// Toggles the Led status
    pub fn toggle(&mut self) -> Result<(), P::Error> {
        self.pin.toggle()
    }

    /// Turn led on
    pub fn on(&mut self) -> Result<(), P::Error> {
        self.pin.set_high()
    }

    /// Turn led off
    pub fn off(&mut self) -> Result<(), P::Error> {
        self.pin.set_low()
    }

    // LED indicator with 3 bliks, ends off.
    pub fn signal_led(&mut self) -> Result<(), P::Error> {
        self.pin.set_low().unwrap();

        for _ in 0..=6 {
            self.pin.toggle()?;
            arduino_hal::delay_ms(200);
        }

        Ok(())
    }
}
