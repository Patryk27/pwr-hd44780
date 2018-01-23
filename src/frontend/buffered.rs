/// Provides a buffered access to the Hd44780.
///
/// Can be used like a regular Hd44780, except that one must manually call the `render` method at
/// some point.
///
/// # Example

/// ```rust
/// let mut lcd_dev = LinuxI2CDevice::new("/dev/i2c-1", 0x27).unwrap();
/// let mut lcd_interface = hd44780::interface::i2c::I2CInterface::new(&mut lcd_dev);
/// let mut lcd_direct = hd44780::DirectHd44780::new(&mut lcd_interface);
/// let mut lcd = hd44780::BufferedHd44780::new(&mut lcd_direct);
///
/// lcd.print("Hello World! :-)");
/// lcd.render();
/// ```
///
/// # New methods
///
/// The buffered frontend provides this new methods:
/// - `render`,
/// - `println`.
///
/// # Caveats
///
/// 1. Although rendering the text requires a call to the `render` method, modifying the LCD's state
/// does not. Thus calling eg. the `set_backlight` method works instantly (does not require calling
/// `render`) and it's entirely by the design.
///
/// 2. `set_cursor_blinking` / `set_cursor_visible` do not play well with buffering and thus are
/// discouraged.

use super::super::Hd44780;
use super::Direct;

pub struct Buffered<'a> {
    lcd: &'a mut Direct<'a>,

    cursor: Cursor,
    buffer: Buffer,
}

struct Cursor {
    y: usize,
    x: usize,
}

struct Buffer {
    lines: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl<'a> Buffered<'a> {
    /// Creates a new buffered Hd44780 basing on previously existing direct one.
    pub fn new(lcd: &'a mut Direct<'a>) -> Buffered<'a> {
        let (width, height) = (lcd.get_width(), lcd.get_height());

        Buffered {
            lcd,

            cursor: Cursor {
                y: 0,
                x: 0,
            },

            buffer: Buffer {
                lines: vec![vec![' ' as u8; width]; height],
                width,
                height,
            },
        }
    }

    /// Refreshes the screen.
    pub fn render(&mut self) {
        let mut y = 0;

        for line in &self.buffer.lines {
            self.lcd.move_at(y, 0);

            for ch in line {
                self.lcd.print_char(*ch);
            }

            y += 1;
        }
    }

    /// Prints text at current cursor's position and moves to the next line.
    pub fn println(&mut self, str: String) {
        self.print(str);

        self.cursor.x = 0;
        self.cursor.y += 1;
    }

    /// Prints a single character.
    fn print_char(&mut self, ch: u8) {
        if self.cursor.y >= self.buffer.height || self.cursor.x >= self.buffer.width {
            return;
        }

        // print the character
        self.buffer.lines[self.cursor.y][self.cursor.x] = ch;

        // move the cursor
        self.cursor.x += 1;

        if self.cursor.x >= self.buffer.width {
            self.cursor.x = 0;
            self.cursor.y += 1;
        }
    }
}

impl<'a> Hd44780 for Buffered<'a> {
    fn clear(&mut self) {
        for line in &mut self.buffer.lines {
            for ch in line {
                *ch = ' ' as u8;
            }
        }

        self.move_at(0, 0);
    }

    fn home(&mut self) {
        self.move_at(0, 0);
    }

    fn move_at(&mut self, y: usize, x: usize) {
        if y as usize >= self.get_height() || x as usize >= self.get_width() {
            return;
        }

        self.cursor.y = y;
        self.cursor.x = x;
    }

    fn print(&mut self, str: String) {
        for ch in str.chars() {
            self.print_char(ch as u8);
        }
    }

    fn set_backlight(&mut self, enabled: bool) {
        self.lcd.set_backlight(enabled);
    }

    fn set_cursor_blinking(&mut self, enabled: bool) {
        self.lcd.set_cursor_blinking(enabled);
    }

    fn set_cursor_visible(&mut self, enabled: bool) {
        self.lcd.set_cursor_visible(enabled);
    }

    fn set_text_visible(&mut self, enabled: bool) {
        self.lcd.set_text_visible(enabled);
    }

    fn set_char(&mut self, idx: u8, lines: [u8; 8]) {
        self.lcd.set_char(idx, lines);
    }

    fn get_width(&mut self) -> usize {
        self.buffer.width
    }

    fn get_height(&mut self) -> usize {
        self.buffer.height
    }
}