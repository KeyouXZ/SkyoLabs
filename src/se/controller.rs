use esp_hal::time::{Duration, Instant};

use crate::{driver::buzzer::Buzzer, se::core::Level};

/// Controller for the buzzer
pub struct BuzzerController<'a> {
    buzzer: Buzzer<'a>,
    last_burst: Instant,
}

impl<'a> BuzzerController<'a> {
    pub fn new(buzzer: Buzzer<'a>) -> Self {
        Self {
            buzzer,
            last_burst: Instant::now(),
        }
    }

    fn delay(ms: u64) {
        let delay_start = Instant::now();

        while delay_start.elapsed() < Duration::from_millis(ms) {}
    }

    pub fn update(&mut self, level: Level) {
        let now = Instant::now();

        if level == Level::Safe {
            self.buzzer.off();
            return;
        }

        let interval = match level {
            Level::Warning => 1500,
            Level::Danger => 500,
            Level::Safe => return,
        };

        if self.last_burst.elapsed() < Duration::from_millis(interval) {
            return;
        }

        self.last_burst = now;

        match level {
            Level::Warning => {
                for _ in 0..2 {
                    self.buzzer.on();
                    Self::delay(120);

                    self.buzzer.off();
                    Self::delay(60);
                }
            }

            Level::Danger => {
                for _ in 0..4 {
                    self.buzzer.on();
                    Self::delay(100);

                    self.buzzer.off();
                    Self::delay(30);
                }
            }

            Level::Safe => {}
        }
    }
}
