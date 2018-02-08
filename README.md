pwr-hd44780
===========

[![Version](https://img.shields.io/crates/v/pwr-hd44780.svg)](https://crates.io/crates/pwr-hd44780)

[Documentation](https://docs.rs/pwr-hd44780)

A Rust crate allowing to communicate with the **HD44780** LCDs.

# What interfaces are supported?

- **4-bit GPIO** bus (thanks to the [rppal](https://github.com/golemparts/rppal) library),
- **I2C** bus (thanks to the [rust-i2cdev](https://github.com/rust-embedded/rust-i2cdev) library).

# Would you mind showing me some code?

Sure, pal:

```rust
extern crate pwr_hd44780;

use pwr_hd44780::Hd44780;

fn main() {
    // create the LCD's interface
    let mut lcd_interface = pwr_hd44780::interface::I2C::new("/dev/i2c-1", 0x27);

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
```

For more examples, take a into the `examples` directory.

# License

```
Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
Licensed under the MIT license.
```