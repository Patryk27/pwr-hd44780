extern crate pwr_hd44780;

use pwr_hd44780::Hd44780;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<std::error::Error>> {
    // create the LCD's bus instance;
    // use device at address 0x27 on the first I2C bus
    let mut lcd_bus = pwr_hd44780::I2CBus::new(
        "/dev/i2c-1", 0x27,
    )?;

    // create the direct LCD's instance;
    // use bus created before and assume LCD's width x height = 20 x 4
    let mut lcd = pwr_hd44780::DirectLcd::new(
        &mut lcd_bus,
        20, 4,
    )?;

    // finally - print our text
    lcd.clear()?;
    lcd.print("Hello World! :-)")?;

    Ok(())
}