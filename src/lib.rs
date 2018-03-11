#![crate_type = "lib"]
#![crate_name = "gpiozero"]

//! A simple interface to GPIO devices with Raspberry Pi.
//!
//! This library is based on [GPIOZero](https://gpiozero.readthedocs.io/en/stable/index.html)
//! library created by [Ben Nuttall](https://github.com/bennuttall) of the `Raspberry Pi Foundation`,
//! [Dave Jones](https://github.com/waveform80), and other contributors.
//!
//! _Note: This is a work in progress. The library will eventually support `embedded-hal` based drivers_
//!
//!
//! The idea is to get started with physical computing using Rust with little coding
//! by hiding the underlying complexity.
//! # Example : Blinking an LED
//!
//! ```no_run
//!
//! extern crate gpiozero;
//! use gpiozero::*;
//!
//! fn main() {
//!
//! // Create a new LED attached to Pin 14
//! let mut led = LED::new(14);
//! // blink the LED
//! led.blink(2,3);
//!
//! }
//! ```
//!
//!
//!  # Example : Wait for a Button Press
//!
//! ```no_run
//!
//! let button = Button::new(17);
//! button.wait_for_press();
//! println!("button pressed");
//! ```

extern crate sysfs_gpio;

#[cfg(nightly)]
extern crate embedded_hal as hal;

pub use self::output_devices::*;
pub use self::devices::*;
pub use self::traits::*;
pub use self::input_devices::*;
//pub mod led;
pub mod devices;
pub mod output_devices;
pub mod input_devices;
pub mod traits;



