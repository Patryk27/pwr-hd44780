/// Provides an example of connecting to the HD44780 using an I2C bus.
///
/// Example assumes your HD44780 is located at the 0x27 address - you can of course change it as you
/// wish.

extern crate pwr_hd44780;

use pwr_hd44780::Hd44780;

fn main() {
    // create the interface instance;
    // use device at address 0x27 on the first i2c bus
    let mut lcd_interface = pwr_hd44780::interface::I2C::new(
        "/dev/i2c-1", 0x27
    );

    // create the LCD's frontend;
    // use interface created before and assume LCD's width x height = 20 x 4
    let mut lcd = pwr_hd44780::frontend::Direct::new(
        &mut lcd_interface,
        20, 4
    );

    // finally - print our text
    lcd.clear();
    lcd.print("Hello World! :-)");
}