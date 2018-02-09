use self::command::*;
pub use self::gpio4::Gpio4;
pub use self::i2c::I2C;
use std::{thread, time};
use super::UnitResult;

pub(crate) mod command;

pub mod i2c;
pub mod gpio4;

pub trait Bus {
    /// Initializes the bus (eg. puts LCD in appropriate 4/8-bit mode).
    fn initialize(&mut self) -> UnitResult;

    /// Enables / disables the backlight.
    fn set_backlight(&mut self, enabled: bool) -> UnitResult;

    /// Sends a single byte to the device.
    /// When `as_data` is `true`, the `RS` register is pulled up and byte is sent as `data`.
    fn write_byte(&mut self, value: u8, as_data: bool) -> UnitResult;

    /// Sends a raw command to the device.
    fn write_command(&mut self, value: u8) -> UnitResult {
        self.write_byte(value, false)
    }

    /// Sends a raw data to the device.
    fn write_data(&mut self, value: u8) -> UnitResult {
        self.write_byte(value, true)
    }

    /// Executes given command.
    fn execute(&mut self, command: Command) -> UnitResult {
        match command {
            // -- clear -- //
            Command::Clear => {
                self.write_command(CommandValue::Clear as u8)?;

                // "clear" command requires additional delay
                thread::sleep(time::Duration::new(0, 1000 * 1000));
            }

            // -- home -- //
            Command::Home => {
                self.write_command(CommandValue::Home as u8)?;

                // "home" command requires additional delay
                thread::sleep(time::Duration::new(0, 1000 * 1000));
            }

            // -- set entry mode -- //
            Command::SetEntryMode { enable_shift, increment_counter } => {
                let mut cmd = CommandValue::SetEntryMode as u8;

                cmd |= 0x01 * enable_shift as u8;
                cmd |= 0x02 * increment_counter as u8;

                self.write_command(cmd)?;
            }

            // -- set display flags -- //
            Command::SetDisplayFlags { cursor_blinking, cursor_visible, text_visible } => {
                let mut cmd = CommandValue::SetDisplayFlags as u8;

                cmd |= 0x01 * cursor_blinking as u8;
                cmd |= 0x02 * cursor_visible as u8;
                cmd |= 0x04 * text_visible as u8;

                self.write_command(cmd)?;
            }

            // -- set functions -- //
            Command::SetFunctions { font_5x10, height, eight_bit_bus } => {
                let mut cmd = CommandValue::SetFunctions as u8;

                cmd |= 0x04 * font_5x10 as u8;
                cmd |= 0x08 * (height >= 2) as u8;
                cmd |= 0x10 * eight_bit_bus as u8;

                self.write_command(cmd)?;
            }

            // -- set CGRAM address -- //
            Command::SetCGRamAddress { address } => {
                self.write_command(
                    (CommandValue::SetCGRamAddress as u8) | address
                )?;
            }

            // -- set DDRAM address -- //
            Command::SetDDRamAddress { address } => {
                self.write_command(
                    (CommandValue::SetDDRamAddress as u8) | address
                )?;
            }
        }

        Ok(())
    }

    /// Returns bus width (4 / 8 bit).
    fn width(&self) -> usize;
}