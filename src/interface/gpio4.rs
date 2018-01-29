/// Defines an interface for controlling the HD44780 via a 4-bit GPIO bus.
///
/// Inspired by:
///     <https://github.com/arduino-libraries/LiquidCrystal/blob/master/src/LiquidCrystal.cpp>.
///
/// Thanks to:
///     <https://github.com/golemparts/rppal>, for providing a nice crate allowing to control the
///     GPIOs.
///
/// # Example
///
/// Wiring used in the example:
///     D4 - 26
///     D5 - 6
///     D6 - 5
///     D7 - 16
///     RS - 23
///     EN - 24
///
///     RW - pulled down
///
/// BCM pin numbering is user (as it is natively in the rppal crate).
///
/// ```rust
/// let mut gpio = Gpio::new().unwrap();
///
/// // initialize the GPIOs
/// for pin in [26, 6, 5, 16, 23, 24].iter() {
///     gpio.set_mode(*pin, Mode::Output);
/// }
///
/// // create the LCD's interface
/// let mut lcd_interface = pwr_hd44780::interface::Gpio4::new(
///     &gpio,
///
///     pwr_hd44780::interface::gpio4::Pins {
///         data: [26, 6, 5, 16],
///         rs: 23,
///         en: 24,
///     },
/// );
///
/// // create the LCD's frontend
/// let mut lcd = pwr_hd44780::frontend::Direct::new(
///     &mut lcd_interface,
///
///     pwr_hd44780::Properties {
///         width: 16,
///         height: 2,
///         font: pwr_hd44780::Font::Font5x8,
///     }
/// );
///
/// // finally - print our text
/// lcd.clear();
/// lcd.print(String::from("Hello World! :-)"));
/// ```
///
/// # Caveats
///
/// 1. No backlight support yet.
///
/// # Q&A
///
/// Q1. Why aren't the GPIOs' directions set automatically?
///
/// A1. `gpio.set_mode` would require my library to mutably borrow the `gpio` reference, effectively
///     making it unusable anywhere later in the code.

use rppal::gpio::{Gpio, Level};
use std::{thread, time};
use super::super::interface::Interface;

pub struct Gpio4<'a> {
    gpio: &'a Gpio,
    pins: Pins,
}

pub struct Pins {
    /// four `data` pins
    pub data: [u8; 4],

    /// `register select` pin
    pub rs: u8,

    /// `enable` pin
    pub en: u8,
}

impl<'a> Gpio4<'a> {
    /// Constructs a new HD44780 GPIO interface with 4-bit communication bus.
    pub fn new(gpio: &'a Gpio, pins: Pins) -> Gpio4<'a> {
        Gpio4 {
            gpio,
            pins,
        }
    }

    /// Sends a single nibble, latching the `Enable` pin.
    fn write_nibble(&mut self, value: u8, as_data: bool) {
        let write_pin = |pin: u8, enabled: bool| {
            self.gpio.write(pin, if enabled { Level::High } else { Level::Low });
        };

        write_pin(self.pins.en, false);
        write_pin(self.pins.rs, as_data);

        write_pin(self.pins.data[0], value & 0b0001_0000u8 > 0);
        write_pin(self.pins.data[1], value & 0b0010_0000u8 > 0);
        write_pin(self.pins.data[2], value & 0b0100_0000u8 > 0);
        write_pin(self.pins.data[3], value & 0b1000_0000u8 > 0);

        // give LCD some time to process GPIO changes
        thread::sleep(time::Duration::new(0, 1000));

        // pull up the `enable` pin & wait ~450ns (enable pulse must be >450ns)
        write_pin(self.pins.en, true);
        thread::sleep(time::Duration::new(0, 450));

        // pull down the `enable` pin & wait ~37us (commands need 37us to settle)
        write_pin(self.pins.en, false);
        thread::sleep(time::Duration::new(0, 37 * 1000));
    }
}

impl<'a> Interface for Gpio4<'a> {
    fn initialize(&mut self) {
        // initialize the screen
        let commands = vec![
            // try to put LCD in 8-bit mode three times;
            // required for initialization when LCD has not been previously restarted
            0x03,
            0x03,
            0x03,

            // put LCD in proper 4-bit mode
            0x02,
        ];

        for c in commands {
            self.write_nibble(c << 4, false);
            thread::sleep(time::Duration::new(0, 100 * 1000));
        }
    }

    fn get_bus_width(&mut self) -> usize {
        4
    }

    fn set_backlight(&mut self, enabled: bool) {
        // @todo
    }

    fn write_byte(&mut self, value: u8, as_data: bool) {
        self.write_nibble(value << 0, as_data);
        self.write_nibble(value << 4, as_data);
    }
}