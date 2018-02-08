extern crate pwr_hd44780;

use pwr_hd44780::Hd44780;

fn main() {
    // create the interface instance;
    // use device at address 0x27 on the first i2c bus
    let mut lcd_interface = pwr_hd44780::interface::I2C::new(
        "/dev/i2c-1", 0x27
    );

    // create the direct LCD instance;
    // use interface created before and assume LCD's width x height = 20 x 4
    let mut direct_lcd = pwr_hd44780::frontend::Direct::new(
        &mut lcd_interface,
        20, 4
    );

    // create the buffered LCD instance
    let mut lcd = pwr_hd44780::frontend::Buffered::new(&mut direct_lcd);

    // finally - print our text
    loop {
        lcd.clear();
        lcd.print("Hello World! :-)");
        lcd.render();
    }
}