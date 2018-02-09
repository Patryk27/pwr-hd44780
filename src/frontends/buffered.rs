/// Provides a buffered access to the HD44780.
///
/// Can be used like a regular HD44780, except that one must manually call the `render` method at
/// some point to actually refresh the screen.
///
/// # Additional methods
///
/// This frontend provides some new methods, which are not present in the direct one - namely:
/// - `render`,
/// - `println`.
///
/// # Caveats
///
/// 1. Although rendering the text requires a call to the `render` method, modifying the LCD's state
///    does not. Thus calling eg. the `set_backlight` method results in an instant change. Same
///    applies to `create_char` and a few other ones.
///
/// 2. `set_cursor_blinking` & `set_cursor_visible` do not play well with buffering and thus their
///    usage is discouraged.

use super::Direct;
use super::super::{Hd44780, Result, UnitResult};

pub struct Buffered {
    lcd: Box<Direct>,
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

impl Buffered {
    /// Creates a new buffered HD44780 basing on previously existing direct one.
    pub fn new(lcd: Box<Direct>) -> Result<Buffered> {
        let (height, width) = (lcd.height(), lcd.width());

        Ok(
            Buffered {
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

    /// Refreshes the screen.
    pub fn render(&mut self) -> UnitResult {
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
    pub fn println<T: Into<String>>(&mut self, str: T) -> UnitResult {
        self.print(str)?;

        self.cursor.x = 0;
        self.cursor.y += 1;

        Ok(())
    }
}

impl Hd44780 for Buffered {
    fn clear(&mut self) -> UnitResult {
        for line in &mut self.buffer.lines {
            for ch in line {
                *ch = ' ' as u8;
            }
        }

        self.move_at(0, 0)
    }

    fn home(&mut self) -> UnitResult {
        self.move_at(0, 0)
    }

    fn move_at(&mut self, y: usize, x: usize) -> UnitResult {
        if y >= self.height() || x >= self.width() {
            return Err(
                format!("Tried to move the cursor outside the screen (at y={}, x={}).", y, x).into()
            );
        }

        self.cursor.y = y;
        self.cursor.x = x;

        Ok(())
    }

    fn print_char(&mut self, ch: u8) -> UnitResult {
        if self.cursor.y >= self.buffer.height || self.cursor.x >= self.buffer.width {
            return Err("Tried to print a character outside the screen.".into());
        }

        // print the character
        self.buffer.lines[self.cursor.y][self.cursor.x] = ch;

        // move the cursor
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

    fn set_backlight(&mut self, enabled: bool) -> UnitResult {
        self.lcd.set_backlight(enabled)
    }

    fn set_cursor_blinking(&mut self, enabled: bool) -> UnitResult {
        self.lcd.set_cursor_blinking(enabled)
    }

    fn set_cursor_visible(&mut self, enabled: bool) -> UnitResult {
        self.lcd.set_cursor_visible(enabled)
    }

    fn set_text_visible(&mut self, enabled: bool) -> UnitResult {
        self.lcd.set_text_visible(enabled)
    }

    fn create_char(&mut self, idx: u8, lines: [u8; 8]) -> UnitResult {
        self.lcd.create_char(idx, lines)
    }

    fn height(&self) -> usize {
        self.buffer.height
    }

    fn width(&self) -> usize {
        self.buffer.width
    }
}