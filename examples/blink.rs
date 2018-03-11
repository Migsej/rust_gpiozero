//! Blinks an LED : on_time: 2 seconds and off_time: 3 seconds

extern crate gpiozero;
use gpiozero::*;

fn main() {

    // Create a new LED attached to Pin 14
    let mut led = LED::new(14);
    // blink the LED
    led.blink(2,3);

  }
