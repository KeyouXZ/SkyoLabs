use esp_hal::gpio::{Level, Output, OutputConfig, OutputPin};

pub struct Buzzer<'d> {
    pin: Output<'d>,
}

impl<'d> Buzzer<'d> {
    pub fn new<PIN>(pin: PIN) -> Self
    where
        PIN: OutputPin + 'd,
    {
        Self {
            pin: Output::new(pin, Level::Low, OutputConfig::default()),
        }
    }

    pub fn on(&mut self) {
        self.pin.set_high();
    }

    pub fn off(&mut self) {
        self.pin.set_low();
    }
}
