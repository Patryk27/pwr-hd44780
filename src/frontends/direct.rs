/// Provides a direct access to the HD44780.
///
/// # Caveats
///
/// 1. The `clear` and `home` methods are slow (HD44780 requires an additional delay to process
///    them). If performance is a concern, please consider using the buffered frontend.

use super::super::*;
use super::super::buses::Bus;
use super::super::buses::command::*;

pub struct Direct<'a> {
    bus: &'a mut Bus,
    properties: Properties,
    state: State,
}

struct State {
    cursor_blinking: bool,
    cursor_visible: bool,
    text_visible: bool,
}

impl<'a> Direct<'a> {
    /// Creates a new direct HD44780 on given bus.
    pub fn new(bus: &'a mut Bus, width: usize, height: usize) -> Result<Direct<'a>> {
        Direct::new_ex(
            bus,
            Properties {
                width,
                height,

                font: if height == 1 { Font::Font5x10 } else { Font::Font5x8 },
            },
        )
    }

    /// Creates a new direct HD44780 on given bus.
    pub fn new_ex(bus: &'a mut Bus, properties: Properties) -> Result<Direct<'a>> {
        let mut lcd = Direct {
            bus,
            properties,

            state: State {
                cursor_blinking: false,
                cursor_visible: false,
                text_visible: true,
            },
        };

        lcd.initialize()?;

        Ok(lcd)
    }

    /// Initializes the screen.
    fn initialize(&mut self) -> UnitResult {
        // initialize the bus
        self.bus.initialize()?;

        // initialize the screen
        let height = self.height();
        let bus_width = self.bus.width();

        self.bus.execute(Command::SetFunctions {
            font_5x10: self.properties.font == Font::Font5x10,
            height: height,
            eight_bit_bus: bus_width == 8,
        })?;

        self.bus.execute(Command::SetEntryMode {
            enable_shift: false,
            increment_counter: true,
        })?;

        self.refresh_display_flags()
    }

    /// Issues the "set display flags" command with current LCD's state.
    fn refresh_display_flags(&mut self) -> UnitResult {
        self.bus.execute(Command::SetDisplayFlags {
            cursor_blinking: self.state.cursor_blinking,
            cursor_visible: self.state.cursor_visible,
            text_visible: self.state.text_visible,
        })
    }
}

impl<'a> Hd44780 for Direct<'a> {
    /// Clears the screen.
    /// It's a slow command, re-writing screen with new data should be a preferred way if one is
    /// concerned about the performance (that's precisely what the "buffered" frontend does).
    fn clear(&mut self) -> UnitResult {
        self.bus.execute(Command::Clear {})
    }

    /// Moves cursor at (0, 0).
    /// It's actually slower than "move_at(0, 0)", because HD44780 takes some time to process this
    /// one.
    fn home(&mut self) -> UnitResult {
        self.bus.execute(Command::Home {})
    }

    fn move_at(&mut self, y: usize, x: usize) -> UnitResult {
        if y as usize >= self.height() || x as usize >= self.width() {
            return Err("Tried to move the cursor outside the screen.".into());
        }

        let addresses = vec![0x00, 0x40, 0x14, 0x54];

        self.bus.execute(Command::SetDDRamAddress {
            address: (addresses[y] + x) as u8,
        })
    }

    fn print_char(&mut self, ch: u8) -> UnitResult {
        self.bus.write_data(ch)
    }

    fn set_backlight(&mut self, enabled: bool) -> UnitResult {
        self.bus.set_backlight(enabled)
    }

    fn set_cursor_blinking(&mut self, enabled: bool) -> UnitResult {
        self.state.cursor_blinking = enabled;
        self.refresh_display_flags()
    }

    fn set_cursor_visible(&mut self, enabled: bool) -> UnitResult {
        self.state.cursor_visible = enabled;
        self.refresh_display_flags()
    }

    fn set_text_visible(&mut self, enabled: bool) -> UnitResult {
        self.state.text_visible = enabled;
        self.refresh_display_flags()
    }

    fn create_char(&mut self, idx: u8, lines: [u8; 8]) -> UnitResult {
        if idx > 7 {
            return Err("Index out of range - character index must be in range <0, 7>".into());
        }

        self.bus.execute(Command::SetCGRamAddress {
            address: idx << 3,
        })?;

        for line in lines.iter() {
            self.bus.write_data(*line)?;
        }

        Ok(())
    }

    fn height(&mut self) -> usize {
        self.properties.height
    }

    fn width(&mut self) -> usize {
        self.properties.width
    }
}