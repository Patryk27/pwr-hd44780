/// Provides an example of using the buffered HD44780 version.
///
/// Using buffer reduces flickering, as the screen is never actually reset (cleared), but rather
/// constantly overwritten with new data.

extern crate pwr_hd44780;

use pwr_hd44780::Hd44780;
use std::time;

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
    let mut direct_lcd = pwr_hd44780::DirectLcd::new(
        &mut lcd_bus,
        20, 4,
    )?;

    // create the buffered LCD's instance
    let mut lcd = pwr_hd44780::BufferedLcd::new(&mut direct_lcd)?;

    // finally - print our text
    loop {
        let now = time::SystemTime::now();
        let since_the_epoch = now.duration_since(time::UNIX_EPOCH)?;

        lcd.clear()?;
        lcd.println(format!("{}", since_the_epoch.as_secs()))?;
        lcd.println(format!("{}", since_the_epoch.subsec_nanos()))?;
        lcd.render()?;
    }
}