/// Defines an interface for controlling the HD44780 via the I2C bus.
/// Loosely based on <https://github.com/fdebrabander/Arduino-LiquidCrystal-I2C-library>.
///
/// # The Protocol
///
/// The PCF8574 family (to which this driver's been written) allows us to send only a nibble
/// (4 bits) of command / data at once - the rest 4 bits are used as control bits, precisely being:
///
/// `U1 U2 U3 U4 | Bl En Rw Rs`
/// `1  2  3  4  | 5  6  7  8`
///
/// `U1..U4` - currently sent part of the command or data
/// `Bl`     - `backlight` pin (`0` - disabled, `1` - enabled)
/// `En`     - `enable` pin (as above)
/// `Rw`     - `read / write` pin (as above)
/// `Rs`     - `register select` pin (`0` - command, `1` - data)

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use std::{thread, time};
use super::super::interface::Interface;

pub struct I2C<'a> {
    dev: &'a mut LinuxI2CDevice,
    backlight_enabled: bool,
}

impl<'a> I2C<'a> {
    /// Constructs a new Hd44780 I2C interface.
    pub fn new(dev: &'a mut LinuxI2CDevice) -> I2C<'a> {
        I2C {
            dev,
            backlight_enabled: true,
        }
    }

    /// Sends a single nibble, latching the `Enable` pin.
    fn write_nibble(&mut self, value: u8) {
        // write value and pull up the `Enable` pin
        self.dev.smbus_write_byte(value | 0b00000100).unwrap();

        // write value again, this time pulling the `Enable` pin down
        self.dev.smbus_write_byte(value & !0b00000100).unwrap();

        // @note technically, two delays should be put here (450us + 37ns), but my tests have shown
        // no negative effect when running without them - so for the sake of performance, they have
        // been omitted.
    }
}

impl<'a> Interface for I2C<'a> {
    fn initialize(&mut self) {
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
            self.write_nibble(c << 4);
            thread::sleep(time::Duration::new(0, 100 * 1000));
        }
    }

    fn get_bus_width(&mut self) -> usize {
        4 // I2C is always used in 4-bit context
    }

    fn set_backlight(&mut self, enabled: bool) {
        self.backlight_enabled = enabled;

        // write a byte to update the backlight state
        self.write_byte(0, false);
    }

    fn write_byte(&mut self, value: u8, as_data: bool) {
        let mut mask = 0u8;

        mask |= 0b00001000 * (self.backlight_enabled as u8);
        mask |= 0b00000001 * (as_data as u8);

        // write high nibble
        self.write_nibble((value << 0) & 0xF0 | mask);

        // write low nibble
        self.write_nibble((value << 4) & 0xF0 | mask);
    }
}