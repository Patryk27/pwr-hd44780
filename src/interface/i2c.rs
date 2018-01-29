/// Defines an interface for controlling the HD44780 via an I2C bus.
///
/// Inspired by:
///     <https://github.com/fdebrabander/Arduino-LiquidCrystal-I2C-library>.
///
/// Thanks to:
///     <https://github.com/rust-embedded/rust-i2cdev> for providing a nice crate allowing to
///     control the I2C.
///
/// # Example
///
/// Example assumes your HD44780 is located at the 0x27 address - you can of course change it as
/// you wish.
///
/// ```rust
/// // create the I2C device
/// let mut lcd_device = LinuxI2CDevice::new("/dev/i2c-1", 0x27).unwrap();
///
/// // create the LCD's interface
/// let mut lcd_interface = pwr_hd44780::interface::I2C::new(&mut lcd_device);
///
/// // create the LCD's frontend
/// let mut lcd = pwr_hd44780::frontend::Direct::new(
///     &mut lcd_interface,
///
///     pwr_hd44780::Properties {
///         width: 20,
///         height: 4,
///         font: pwr_hd44780::Font::Font5x8,
///     }
/// );
///
/// // finally - print our text
/// lcd.clear();
/// lcd.print(String::from("Hello World! :-)"));
/// ```
///
/// # A word on the protocol itself
///
/// The PCF8574 family (to which this driver's been written) allows us to send only a nibble
/// (4 bits) of command or data at once - the rest 4 bits are used as control bits, precisely being:
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
    /// Constructs a new HD44780 I2C interface.
    pub fn new(dev: &'a mut LinuxI2CDevice) -> I2C<'a> {
        I2C {
            dev,
            backlight_enabled: true,
        }
    }

    /// Sends a single nibble, latching the `Enable` pin.
    fn write_nibble(&mut self, value: u8) {
        // write value, pull up the `enable` pin & wait ~450ns (enable pulse must be >450ns)
        self.dev.smbus_write_byte(value | 0b00000100).unwrap();
        thread::sleep(time::Duration::new(0, 450));

        // write value again, this time pulling the `Enable` pin down & wait ~37us (commands need 37us to settle)
        self.dev.smbus_write_byte(value & !0b00000100).unwrap();
        thread::sleep(time::Duration::new(0, 37 * 1000));
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
        4 // I2C is always used in a 4-bit context
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

        self.write_nibble((value << 0) & 0xF0 | mask);
        self.write_nibble((value << 4) & 0xF0 | mask);
    }
}