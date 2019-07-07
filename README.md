pwr-hd44780
===========

[![Version](https://img.shields.io/crates/v/pwr-hd44780.svg)](https://crates.io/crates/pwr-hd44780)

[Documentation](https://docs.rs/pwr-hd44780)

Hand-made Rust driver for the brilliant **HD44780** LCDs.

# Supported buses

- **4-bit GPIO** (thanks to the [rppal](https://github.com/golemparts/rppal) library),
- **I2C** (thanks to the [rust-i2cdev](https://github.com/rust-embedded/rust-i2cdev) library).

# Example code

```rust
use pwr_hd44780::Hd44780;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    // create the LCD's bus instance;
    // use device at address 0x27 on the first I2C bus
    let lcd_bus = pwr_hd44780::I2CBus::new(
        "/dev/i2c-1", 0x27,
    )?;

    // create the direct LCD's instance;
    // use bus created before and assume LCD's width x height = 20 x 4
    let mut lcd = pwr_hd44780::DirectLcd::new(
        Box::new(lcd_bus),
        20, 4,
    )?;

    // finally - print our text
    lcd.clear()?;
    lcd.print("Hello World! :-)")?;

    Ok(())
}
```

For more examples, take a dive into the `examples` directory.

# License

```
Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
Licensed under the MIT license.
```