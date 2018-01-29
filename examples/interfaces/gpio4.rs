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
extern crate rppal;

use pwr_hd44780::Hd44780;
use rppal::gpio::{Gpio, Mode};

fn main() {
    let mut gpio = Gpio::new().unwrap();

    // initialize the GPIOs
    for pin in [26, 6, 5, 16, 23, 24].iter() {
        gpio.set_mode(*pin, Mode::Output);
    }

    // create the LCD's interface
    let mut lcd_interface = pwr_hd44780::interface::Gpio4::new(
        &gpio,

        pwr_hd44780::interface::gpio4::Pins {
            data: [26, 6, 5, 16],
            rs: 23,
            en: 24,
        },
    );

    // create the LCD's frontend
    let mut lcd = pwr_hd44780::frontend::Direct::new(
        &mut lcd_interface,

        pwr_hd44780::Properties {
            width: 16,
            height: 2,
            font: pwr_hd44780::Font::Font5x8,
        }
    );

    // finally - print our text
    lcd.clear();
    lcd.print(String::from("Hello World! :-)"));
}