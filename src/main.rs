use crate::led::led::Led;
use crate::led::led_status::LedStatus::ON;
use std::os::unix::raw::mode_t;
use std::time::Duration;
use std::{thread, time};

mod led;

fn main() {
    let one_second = Some(Duration::from_secs(1));
    let two_seconds = Some(Duration::from_secs(2));
    let led_1_name = Some("Led 1");
    let led_2_name = Some("Led 2");
    let mut led_1 = Led::new(Some(ON), one_second, led_1_name);
    let mut led_2 = Led::new(None, two_seconds, led_2_name);

    multi_threaded_blink(led_1, &mut led_2);

    let mut led_1 = Led::new(Some(ON), one_second, led_1_name);
    let mut led_2 = Led::new(None, two_seconds, led_2_name);
    single_threaded_blink(&mut led_1, &mut led_2);
}

fn single_threaded_blink(led_1: &mut Led, led_2: &mut Led) {
    let now = time::Instant::now();
    for i in 1..10 {
        led_1.blink(None);
        println!(
            "Led 1, Round: {}, Time Elapsed: {} ms",
            i,
            now.elapsed().as_millis()
        );
        led_2.blink(None);
        println!(
            "Led 2, Round: {}, Time Elapsed: {} ms",
            i,
            now.elapsed().as_millis()
        );
    }
}

fn multi_threaded_blink(mut led_1: Led, led_2: &mut Led) {
    let now = time::Instant::now();
    thread::spawn(move || {
        for i in 1..10 {
            led_1.blink(None);
            println!(
                "Led 1, Round: {}, Time Elapsed: {} ms",
                i,
                now.elapsed().as_millis()
            );
        }
    });

    for i in 1..10 {
        led_2.blink(None);
        println!(
            "Led 2, Round: {}, Time Elapsed: {} ms",
            i,
            now.elapsed().as_millis()
        );
    }
}
