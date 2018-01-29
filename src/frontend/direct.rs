/// Provides a 'direct' access to the HD44780.
/// (`direct` in the meaning of `less abstracted than the buffered one`).
///
/// # Example
///
/// ```rust
/// let mut lcd_device = LinuxI2CDevice::new("/dev/i2c-1", 0x27).unwrap();
/// let mut lcd_interface = pwr_hd44780::interface::I2C::new(&mut lcd_device);
/// let mut lcd = pwr_hd44780::frontend::Direct::new(&mut lcd_interface);
///
/// lcd.clear();
/// lcd.print("Hello World! :-)");
/// ```
///
/// # Caveats
///
/// 1. The `clear` and `home` methods are slow (HD44780 requires an additional delay to process
///    them). If performance is a concern, please consider using the buffered frontend.

use super::super::*;
use super::super::interface::command::*;
use super::super::interface::Interface;

pub struct Direct<'a> {
    interface: &'a mut Interface,
    properties: Properties,
    state: State,
}

struct State {
    cursor_blinking: bool,
    cursor_visible: bool,
    text_visible: bool,
}

impl<'a> Direct<'a> {
    /// Creates a new raw HD44780 instance on given interface.
    pub fn new(interface: &'a mut Interface, properties: Properties) -> Direct<'a> {
        let mut lcd = Direct {
            interface,
            properties,

            state: State {
                cursor_blinking: false,
                cursor_visible: false,
                text_visible: true,
            },
        };

        lcd.initialize();

        lcd
    }

    /// Prints a single character.
    pub fn print_char(&mut self, ch: u8) {
        self.interface.write_data(ch);
    }

    /// Initializes the screen.
    fn initialize(&mut self) {
        // initialize the interface
        self.interface.initialize();

        // initialize the screen
        let height = self.get_height();
        let bus_width = self.interface.get_bus_width();

        self.interface.execute(Command::SetFunctions {
            font_5x10: self.properties.font == Font::Font5x10,
            height: height,
            eight_bit_bus: bus_width == 8,
        });

        self.interface.execute(Command::SetEntryMode {
            enable_shift: false,
            increment_counter: true,
        });

        self.refresh_display_flags()
    }

    /// Issues the "set display flags" command with current LCD's state.
    fn refresh_display_flags(&mut self) {
        self.interface.execute(Command::SetDisplayFlags {
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
    fn clear(&mut self) {
        self.interface.execute(Command::Clear {});
    }

    /// Moves cursor at (0, 0).
    /// It's actually slower than "move_at(0, 0)", because HD44780 takes some time to process this
    /// one.
    fn home(&mut self) {
        self.interface.execute(Command::Home {});
    }

    fn move_at(&mut self, y: usize, x: usize) {
        if y as usize >= self.get_height() || x as usize >= self.get_width() {
            return;
        }

        let addresses = vec![0x00, 0x40, 0x14, 0x54];

        self.interface.execute(Command::SetDDRamAddress {
            address: (addresses[y] + x) as u8,
        });
    }

    fn print(&mut self, str: String) {
        for ch in str.chars() {
            self.interface.write_data(ch as u8);
        }
    }

    fn set_backlight(&mut self, enabled: bool) {
        self.interface.set_backlight(enabled);
    }

    fn set_cursor_blinking(&mut self, enabled: bool) {
        self.state.cursor_blinking = enabled;
        self.refresh_display_flags()
    }

    fn set_cursor_visible(&mut self, enabled: bool) {
        self.state.cursor_visible = enabled;
        self.refresh_display_flags()
    }

    fn set_text_visible(&mut self, enabled: bool) {
        self.state.text_visible = enabled;
        self.refresh_display_flags()
    }

    fn set_char(&mut self, idx: u8, lines: [u8; 8]) {
        if idx > 7 {
            return;
        }

        self.interface.execute(Command::SetCGRamAddress {
            address: idx << 3,
        });

        for line in lines.iter() {
            self.interface.write_data(*line);
        }
    }

    fn get_width(&mut self) -> usize {
        return self.properties.width;
    }

    fn get_height(&mut self) -> usize {
        return self.properties.height;
    }
}