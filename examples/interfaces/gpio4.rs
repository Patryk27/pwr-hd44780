/// Provides an example of connecting to the HD44780 using a 4-bit GPIO bus.
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

extern crate pwr_hd44780;

use pwr_hd44780::Hd44780;

fn main() {
    run().unwrap();
}

fn run() -> Result<(), Box<std::error::Error>> {
    // create the LCD's bus instance
    let mut lcd_bus = pwr_hd44780::Gpio4Bus::new(
        pwr_hd44780::buses::gpio4::Pins {
            data: [26, 6, 5, 16],
            rs: 23,
            en: 24,
        },
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