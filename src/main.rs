use rppal::gpio::Gpio;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn main() {
    // Constants
    const GPIO_TRIG: u8 = 11;
    const GPIO_ECHO: u8 = 18;
    const SOUND_SPEED: f64 = 34300.0; // in cm/s
    const TIMEOUT: Duration = Duration::from_millis(100);
    const DELAY_BETWEEN_MEASUREMENTS: Duration = Duration::from_millis(100);

    // Setup GPIO
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let mut trig = gpio.get(GPIO_TRIG).unwrap().into_output();
    let echo = gpio.get(GPIO_ECHO).unwrap().into_input();

    loop {
        // Trigger the sensor
        trig.set_low();
        sleep(Duration::from_millis(2));
        trig.set_high();
        sleep(Duration::from_micros(10));
        trig.set_low();

        // Wait for the echo start
        let mut start_time = Instant::now();
        let mut timeout_time = start_time + TIMEOUT;

        while !echo.is_high() {
            if Instant::now() > timeout_time {
                println!("Echo signal timed out on waiting for rising edge");
                break;
            }
        }

        start_time = Instant::now();
        timeout_time = start_time + TIMEOUT;

        // Wait for the echo end
        while echo.is_high() {
            if Instant::now() > timeout_time {
                println!("Echo signal timed out on waiting for falling edge");
                break;
            }
        }

        let pulse_duration = start_time.elapsed().as_secs_f64();
        let distance = pulse_duration * SOUND_SPEED / 2.0;

        // Print the distance
        println!("Distance: {:.0} cm", distance);

        // Delay between measurements
        sleep(DELAY_BETWEEN_MEASUREMENTS);
    }
}
