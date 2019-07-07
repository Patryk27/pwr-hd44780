/// Defines an interface (a bus) for controlling the HD44780 via a 4-bit GPIO.
///
/// Inspired by:
///     <https://github.com/arduino-libraries/LiquidCrystal/blob/master/src/LiquidCrystal.cpp>.
///
/// Thanks to:
///     <https://github.com/golemparts/rppal>, for providing a nice crate allowing to control the
///     GPIOs.
///
/// # Caveats
///
/// 1. No backlight support yet.

use rppal::gpio::{Gpio, Level, Mode};
use std::{thread, time};

use super::super::{Bus, Result, UnitResult};

pub struct Gpio4 {
    gpio: Gpio,
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

impl Gpio4 {
    /// Constructs a new HD44780 GPIO bus.
    pub fn new(pins: Pins) -> Result<Gpio4> {
        let mut gpio = Gpio::new().unwrap();

        gpio.set_mode(pins.data[0], Mode::Output);
        gpio.set_mode(pins.data[1], Mode::Output);
        gpio.set_mode(pins.data[2], Mode::Output);
        gpio.set_mode(pins.data[3], Mode::Output);

        gpio.set_mode(pins.rs, Mode::Output);
        gpio.set_mode(pins.en, Mode::Output);

        Ok(
            Gpio4 {
                gpio,
                pins,
            }
        )
    }

    /// Sends a single nibble, latching the `Enable` pin.
    fn write_nibble(&mut self, value: u8, as_data: bool) -> UnitResult {
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

        Ok(())
    }
}

impl Bus for Gpio4 {
    fn initialize(&mut self) -> UnitResult {
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
            self.write_nibble(c << 4, false)?;
            thread::sleep(time::Duration::new(0, 100 * 1000));
        }

        Ok(())
    }

    fn set_backlight(&mut self, _enabled: bool) -> UnitResult {
        Ok(()) // @todo
    }

    fn write_byte(&mut self, value: u8, as_data: bool) -> UnitResult {
        self.write_nibble(value << 0, as_data)?;
        self.write_nibble(value << 4, as_data)?;

        Ok(())
    }

    fn width(&self) -> usize {
        4
    }
}