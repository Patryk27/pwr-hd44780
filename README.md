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
```

For more examples, take a dive into the `examples` directory.

# License

```
Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
Licensed under the MIT license.
```