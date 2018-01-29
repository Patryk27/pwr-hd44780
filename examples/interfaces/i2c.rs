/// Provides an example of connecting to the HD44780 using an I2C bus.
///
/// Example assumes your HD44780 is located at the 0x27 address - you can of course change it as
/// you wish.

extern crate pwr_hd44780;
extern crate i2cdev;

use i2cdev::linux::LinuxI2CDevice;
use pwr_hd44780::Hd44780;

fn main() {
    // create the I2C device
    let mut lcd_device = LinuxI2CDevice::new("/dev/i2c-1", 0x27).unwrap();

    // create the LCD's interface
    let mut lcd_interface = pwr_hd44780::interface::I2C::new(&mut lcd_device);

    // create the LCD's frontend
    let mut lcd = pwr_hd44780::frontend::Direct::new(
        &mut lcd_interface,

        pwr_hd44780::Properties {
            width: 20,
            height: 4,
            font: pwr_hd44780::Font::Font5x8,
        },
    );

    // finally - print our text
    lcd.clear();
    lcd.print(String::from("Hello World! :-)"));
}