pwr-hd44780
===========

[![Version](https://img.shields.io/crates/v/pwr-hd44780.svg)](https://crates.io/crates/pwr-hd44780)

[Documentation](https://docs.rs/pwr-hd44780)

Hand-made driver for **HD44780** LCDs; intended for Raspberry Pi.

# Supported buses

- **4-bit GPIO**,
- **I2C**.

Both thanks to the [rppal](https://github.com/golemparts/rppal) library.

# Example code

```rust
use pwr_hd44780::Hd44780;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let lcd_bus = pwr_hd44780::I2CBus::new("/dev/i2c-1", 0x27)?;

    let mut lcd = pwr_hd44780::DirectLcd::new(
        Box::new(lcd_bus),
        (20, 4),
    )?;

    lcd.clear()?;
    lcd.print("Hello World! :-)")?;

    Ok(())
}
```

For more examples, take a dive into the `examples` directory.

# License

```
Copyright (c) 2018-2019, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
Licensed under the MIT license.
```