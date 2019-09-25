// @todo describe how this bus works

use rppal::i2c::I2c;

use crate::{lcds::direct, Result, wait_ns, wait_us};

const DATA_PIN_MASK: u8 = 0b00000001;
const ENABLE_PIN_MASK: u8 = 0b00000100;
const BACKLIGHT_PIN_MASK: u8 = 0b00001000;

pub struct I2cBus {
    i2c: I2c,
    backlight_enabled: bool,
}

impl I2cBus {
    pub fn new(i2c: I2c) -> Self {
        Self { i2c, backlight_enabled: true }
    }

    fn write_nibble(&mut self, nibble: u8) -> Result<()> {
        // Pull up the `enable` pin and wait ~450ns (enable pulse must be >450ns)
        self.i2c.smbus_send_byte(nibble | ENABLE_PIN_MASK)?;
        wait_ns(450);

        // Pull down the `enable` pin and wait ~37us (commands need 37us to settle)
        self.i2c.smbus_send_byte(nibble & !ENABLE_PIN_MASK)?;
        wait_us(37);

        Ok(())
    }

    fn write_byte(&mut self, byte: u8, as_data: bool) -> Result<()> {
        let mut mask = 0u8;

        if as_data {
            mask |= DATA_PIN_MASK;
        }

        if self.backlight_enabled {
            mask |= BACKLIGHT_PIN_MASK;
        }

        self.write_nibble((byte << 0) & 0b11110000 | mask)?;
        self.write_nibble((byte << 4) & 0b11110000 | mask)?;

        Ok(())
    }
}

impl direct::Bus for I2cBus {
    fn write_command(&mut self, byte: u8) -> Result<()> {
        unimplemented!()
    }

    fn write_data(&mut self, byte: u8) -> Result<()> {
        unimplemented!()
    }

    fn enable_backlight(&mut self, enabled: bool) -> Result<()> {
        unimplemented!()
    }

    fn size(&self) -> direct::BusSize {
        direct::BusSize::FourBit
    }
}