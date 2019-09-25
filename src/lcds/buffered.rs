/// Provides a buffered access to the HD44780, which helps to overcome flickering in some cases.
///
/// It can be used just a regular HD44780 driver, with one exception: at some point you must
/// manually call the `render()` method - otherwise the screen won't refresh.
///
/// # Caveats
///
/// Although rendering the text requires a call to the `render()` method, modifying the LCD's state
/// does not. Calling the `enable_backlight()` method, for instance, results in an instant change.

use crate::{Lcd, Result};

pub struct BufferedLcd {
    lcd: Box<Lcd>,
    cursor: Cursor,
    buffer: Buffer,
}

struct Cursor {
    y: usize,
    x: usize,
}

struct Buffer {
    lines: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl BufferedLcd {
    /// Creates a new instance of `BufferedLcd`.
    pub fn new(lcd: Box<Lcd>) -> Result<Self> {
        let (height, width) = (lcd.height(), lcd.width());

        Ok(
            BufferedLcd {
                lcd,

                cursor: Cursor {
                    y: 0,
                    x: 0,
                },

                buffer: Buffer {
                    lines: vec![vec![' ' as u8; width]; height],
                    height,
                    width,
                },
            }
        )
    }

    /// Creates a new instance of `BufferedLcd`.
    pub fn new_impl(lcd: impl Lcd) -> Result<Self> {
        Self::new(Box::new(lcd))
    }

    /// Renders contents of the buffer onto the screen.
    pub fn render(&mut self) -> Result<()> {
        let mut y = 0;

        for line in &self.buffer.lines {
            self.lcd.move_at(y, 0)?;

            for ch in line {
                self.lcd.print_char(*ch)?;
            }

            y += 1;
        }

        Ok(())
    }

    /// Prints text at current cursor's position and moves to the next line.
    pub fn println<T: Into<String>>(&mut self, str: T) -> Result<()> {
        self.print(str)?;

        self.cursor.x = 0;
        self.cursor.y += 1;

        Ok(())
    }
}

impl Lcd for BufferedLcd {
    fn clear(&mut self) -> Result<()> {
        for line in &mut self.buffer.lines {
            for ch in line {
                *ch = ' ' as u8;
            }
        }

        self.move_at(0, 0)
    }

    fn home(&mut self) -> Result<()> {
        self.move_at(0, 0)
    }

    fn move_at(&mut self, y: usize, x: usize) -> Result<()> {
        self.validate_coords(y, x)?;

        self.cursor.y = y;
        self.cursor.x = x;

        Ok(())
    }

    fn print_char(&mut self, ch: u8) -> UnitResult {
        self.validate_coords(self.cursor.y, self.cursor.x)?;

        // Print character
        self.buffer.lines[self.cursor.y][self.cursor.x] = ch;

        // Move cursor
        self.cursor.x += 1;

        if self.cursor.x >= self.buffer.width {
            self.cursor.x = 0;
            self.cursor.y += 1;

            if self.cursor.y >= self.buffer.height {
                self.cursor.y = 0;
            }
        }

        Ok(())
    }

    fn enable_backlight(&mut self, enabled: bool) -> UnitResult {
        self.lcd.enable_backlight(enabled)
    }

    fn enable_cursor_box_blinking(&mut self, enabled: bool) -> UnitResult {
        self.lcd.enable_cursor_box_blinking(enabled)
    }

    fn enable_cursor_line_blinking(&mut self, enabled: bool) -> UnitResult {
        self.lcd.enable_cursor_line_blinking(enabled)
    }

    fn enable_text(&mut self, enabled: bool) -> UnitResult {
        self.lcd.enable_text(enabled)
    }

    fn create_char(&mut self, idx: u8, bitmap: [u8; 8]) -> UnitResult {
        self.lcd.create_char(idx, bitmap)
    }

    fn width(&self) -> usize {
        self.buffer.width
    }

    fn height(&self) -> usize {
        self.buffer.height
    }
}