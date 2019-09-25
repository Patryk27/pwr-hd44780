use crate::{Result, wait_ms};

use super::Bus;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Clear,
    Home,

    SetEntryMode {
        enable_shift: bool,
        increment_counter: bool,
    },

    SetDisplayFlags {
        cursor_blinking: bool,
        cursor_visible: bool,
        text_visible: bool,
    },

    SetFunctions {
        font_5x10: bool,
        height: u8,
        eight_bit_bus: bool,
    },

    SetCGRamAddress {
        address: u8,
    },

    SetDDRamAddress {
        address: u8,
    },
}

impl Command {
    pub fn write(self, bus: &mut dyn Bus) -> Result<()> {
        match self {
            Command::Clear => {
                bus.write_command(0x01).map(|_| wait_ms(1))
            }

            Command::Home => {
                bus.write_command(0x02).map(|_| wait_ms(1))
            }

            Command::SetEntryMode { enable_shift, increment_counter } => {
                let mut cmd = 0x04;

                cmd |= 0x01 * enable_shift as u8;
                cmd |= 0x02 * increment_counter as u8;

                bus.write_command(cmd)
            }

            Command::SetDisplayFlags { cursor_blinking, cursor_visible, text_visible } => {
                let mut cmd = 0x08;

                cmd |= 0x01 * cursor_blinking as u8;
                cmd |= 0x02 * cursor_visible as u8;
                cmd |= 0x04 * text_visible as u8;

                bus.write_command(cmd)
            }

            Command::SetFunctions { font_5x10, height, eight_bit_bus } => {
                let mut cmd = 0x20;

                cmd |= 0x04 * font_5x10 as u8;
                cmd |= 0x08 * (height >= 2) as u8;
                cmd |= 0x10 * eight_bit_bus as u8;

                bus.write_command(cmd)
            }

            Command::SetCGRamAddress { address } => {
                bus.write_command(0x40 | address)
            }

            Command::SetDDRamAddress { address } => {
                bus.write_command(0x80 | address)
            }
        }
    }
}
