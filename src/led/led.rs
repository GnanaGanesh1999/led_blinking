use crate::led::led_status::LedStatus;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Led {
    status: LedStatus,
    name: Option<&'static str>,
    sleep_time: Option<Duration>,
}

impl Led {
    pub(crate) fn new(
        status: Option<LedStatus>,
        sleep_time: Option<Duration>,
        name: Option<&'static str>,
    ) -> Self {
        if let Some(status) = status {
            return Led {
                status,
                name,
                sleep_time,
            };
        }
        Led {
            status: LedStatus::OFF,
            name,
            sleep_time,
        }
    }

    pub(crate) fn status(&self) -> String {
        self.status.to_string()
    }

    pub(crate) fn turn_off(&mut self) {
        self.status = LedStatus::OFF;
    }

    fn turn_on(&mut self) {
        self.status = LedStatus::ON;
    }

    pub(crate) fn blink(&mut self, sleep_time: Option<Duration>) {
        let mut dur = Duration::from_secs(0);
        if sleep_time.is_some() {
            dur = sleep_time.unwrap();
        } else if self.sleep_time.is_some() {
            dur = self.sleep_time.unwrap();
        }
        match self.status {
            LedStatus::ON => {
                self.turn_off();
                eprintln!("{} : {}", self.name.unwrap(), self.status.to_string());
                thread::sleep(dur);
                self.turn_on();
                eprintln!("{} : {}", self.name.unwrap(), self.status.to_string());
            }
            LedStatus::OFF => {
                self.turn_on();
                eprintln!("{} : {}", self.name.unwrap(), self.status.to_string());
                thread::sleep(dur);
                self.turn_off();
                eprintln!("{} : {}", self.name.unwrap(), self.status.to_string());
            }
        }
    }
}

#[cfg(test)]
mod led_tests {
    use crate::led::led_status::LedStatus;
    use crate::led::led_status::LedStatus::{OFF, ON};
    use crate::Led;
    use std::time;
    use std::time::Duration;

    #[test]
    fn should_create_new_led_with_off_status() {
        let led = Led::new(None, None, None);
        assert_eq!(led.status(), "OFF".to_string());
    }

    #[test]
    fn should_turn_off_led() {
        let mut led = Led::new(None, None, None);
        led.turn_off();
        assert_eq!(led.status(), "OFF".to_string());
    }

    #[test]
    fn should_turn_on_led() {
        let mut led = Led::new(None, None, None);
        led.turn_on();
        assert_eq!(led.status(), "ON".to_string());
    }

    #[test]
    fn should_blink_led() {
        let mut led_initially_off = Led::new(None, None, None);
        let mut led_initially_on = Led::new(Some(ON), None, None);

        led_initially_off.blink(None);
        led_initially_on.blink(None);

        assert_eq!(led_initially_off.status(), "OFF".to_string());
        assert_eq!(led_initially_on.status(), "ON".to_string());
    }

    #[test]
    fn should_blink_led_after_sleep() {
        let mut led_initially_on = Led::new(Some(ON), None, None);
        let one_second = Duration::from_secs(1);
        let now = time::Instant::now();

        led_initially_on.blink(Some(one_second));

        assert!(now.elapsed() >= one_second);
        assert_eq!(led_initially_on.status(), "ON".to_string());
    }
}
