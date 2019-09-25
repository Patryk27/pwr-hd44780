/// Provides a direct (unbuffered) access to the HD44780.
///
/// # Caveats
///
/// 1. `clear` and `home` methods are rather slow - HD44780 requires an additional delay to process
///    them. If performance is a concern, please consider using the buffered LCD variant.

pub use bus::{Bus, BusSize};
pub use buses::*;

use crate::{Error, Font, Lcd, Point, Properties, Result};

use self::command::*;

mod bus;
mod buses;
mod command;

pub struct DirectLcd<B: Bus> {
    bus: B,
    properties: Properties,
    display_flags: DisplayFlags,
}

struct DisplayFlags {
    cursor_blinking: bool,
    cursor_visible: bool,
    text_visible: bool,
}

impl<B: Bus> DirectLcd<B> {
    pub fn new(bus: B, properties: Properties) -> Result<Self> {
        let mut lcd = DirectLcd {
            bus,
            properties,

            display_flags: DisplayFlags {
                cursor_blinking: false,
                cursor_visible: false,
                text_visible: true,
            },
        };

        lcd.initialize()?;

        Ok(lcd)
    }

    fn initialize(&mut self) -> Result<()> {
        let height = self.dimensions().y;
        let bus_size = self.bus.size();

        Command::SetFunctions {
            font_5x10: self.properties.font == Font::Font5x10,
            height,
            eight_bit_bus: bus_size == BusSize::EightBit,
        }.write(&mut self.bus)?;

        Command::SetEntryMode {
            enable_shift: false,
            increment_counter: true,
        }.write(&mut self.bus)?;

        self.push_display_flags()
    }

    fn push_display_flags(&mut self) -> Result<()> {
        Command::SetDisplayFlags {
            cursor_blinking: self.display_flags.cursor_blinking,
            cursor_visible: self.display_flags.cursor_visible,
            text_visible: self.display_flags.text_visible,
        }.write(&mut self.bus)
    }
}

impl<B: Bus> Lcd for DirectLcd<B> {
    fn clear(&mut self) -> Result<()> {
        Command::Clear.write(&mut self.bus)
    }

    fn home(&mut self) -> Result<()> {
        Command::Home.write(&mut self.bus)
    }

    fn goto(&mut self, p: Point) -> Result<()> {
        p.validate(self)?;

        let addresses = [0x00, 0x40, 0x14, 0x54];

        Command::SetDDRamAddress {
            address: (addresses[p.y as usize] + p.x) as u8,
        }.write(&mut self.bus)
    }

    fn print_char(&mut self, ch: u8) -> Result<()> {
        self.bus.write_data(ch)
    }

    fn enable_backlight(&mut self, enabled: bool) -> Result<()> {
        self.bus.enable_backlight(enabled)
    }

    fn enable_cursor_box_blinking(&mut self, enabled: bool) -> Result<()> {
        self.display_flags.cursor_blinking = enabled;
        self.push_display_flags()
    }

    fn enable_cursor_line_blinking(&mut self, enabled: bool) -> Result<()> {
        self.display_flags.cursor_visible = enabled;
        self.push_display_flags()
    }

    fn enable_text_visibility(&mut self, enabled: bool) -> Result<()> {
        self.display_flags.text_visible = enabled;
        self.push_display_flags()
    }

    fn create_char(&mut self, char: u8, lines: [u8; 8]) -> Result<()> {
        if char > 7 {
            return Err(Error::CharOutOfBounds { char });
        }

        Command::SetCGRamAddress {
            address: char << 3,
        }.write(&mut self.bus)?;

        for line in lines.iter() {
            self.bus.write_data(*line)?;
        }

        Ok(())
    }

    fn dimensions(&self) -> Point {
        self.properties.dimensions
    }
}
