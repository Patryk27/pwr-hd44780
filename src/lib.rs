/// A convenient, high-level driver for the HD44780 display.
/// Supports both the `I2C` and `GPIO` buses + has a buffered implementation.
///
/// # License
///
/// Copyright (c) 2018, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
/// Licensed under the MIT license.

extern crate i2cdev;
extern crate rppal;

pub(crate) use buses::Bus;
pub use buses::Gpio4 as Gpio4Bus;
pub use buses::I2C as I2CBus;
pub use frontends::Buffered as BufferedLcd;
pub use frontends::Direct as DirectLcd;

pub mod buses;
pub mod frontends;

pub type Result<T> = ::std::result::Result<T, Box<std::error::Error>>;
pub type UnitResult = Result<()>;

pub trait Hd44780 {
    /// Clears the screen and moves cursor at (0, 0).
    fn clear(&mut self) -> UnitResult;

    /// Moves the cursor at (0, 0).
    fn home(&mut self) -> UnitResult;

    /// Moves the cursor at given position.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.move_at(2, 2);
    /// ```
    ///
    /// # Errors
    ///
    /// When passed an invalid coordinates (eg. beyond the screen), returns an error and does not
    /// update the cursor position.
    fn move_at(&mut self, y: usize, x: usize) -> UnitResult;

    /// Prints a single ASCII character and moves cursor by one character.
    /// Can be used to print custom-made characters (ie. the ones created by `create_char`).
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.print_char(2);
    /// ```
    fn print_char(&mut self, ch: u8) -> UnitResult;

    /// Prints a string at current cursor's position and moves the cursor.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.print("Hello World!");
    /// lcd.print(format!("Hello, {}!", someone));
    /// ```
    ///
    /// # Errors
    ///
    /// When string overflows current line, the behaviour is undefined.
    fn print<T: Into<String>>(&mut self, str: T) -> UnitResult {
        for ch in str.into().chars() {
            self.print_char(ch as u8)?;
        }

        Ok(())
    }

    /// Prints a string at given position.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.print_at(1, 0, "Hello World!");
    /// lcd.print_at(2, 0, format!("Hello, {}!", someone));
    /// ```
    ///
    /// # Errors
    ///
    /// When string overflows current line, the behaviour is undefined.
    fn print_at<T: Into<String>>(&mut self, y: usize, x: usize, str: T) -> UnitResult {
        self.move_at(y, x)?;
        self.print(str)
    }

    /// Enables / disables the backlight.
    fn set_backlight(&mut self, enabled: bool) -> UnitResult;

    /// Enables / disables blinking the cursor.
    /// `Blinking` means that the whole character box is blinking (a whole 5x8 or 5x10 box),
    fn set_cursor_blinking(&mut self, enabled: bool) -> UnitResult;

    /// Enables / disables the cursor.
    /// `Visible` means that only bottom of the character box is blinking (a single line).
    fn set_cursor_visible(&mut self, enabled: bool) -> UnitResult;

    /// Shows / hides the text.
    fn set_text_visible(&mut self, enabled: bool) -> UnitResult;

    /// Creates a custom character from given bitmap.
    ///
    /// Each array item in given bitmap represents a single line, of which only the last 5 bits are
    /// important - rest is ignored.
    ///
    /// `idx` must be from range `<0, 7>` (that is: only 8 custom characters are possible, that's a
    /// limit imposed by the designers of the HD44780).
    ///
    /// When passed an invalid `idx`, does nothing.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.create_char(1, [
    ///   0b00000000,
    ///   0b10000000,
    ///   0b01000000,
    ///   0b00100000,
    ///   0b00010000,
    ///   0b00001000,
    ///   0b00000100,
    ///   0b00000010,
    /// ]);
    ///
    /// lcd.print_char(1);
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error when passed an invalid index.
    fn create_char(&mut self, idx: u8, lines: [u8; 8]) -> UnitResult;

    /// Returns screen's height (number of lines).
    fn height(&self) -> usize;

    /// Returns screen's width (number of characters per line).
    fn width(&self) -> usize;
}

#[derive(Copy, Clone, PartialEq)]
pub enum Font {
    Font5x8,
    Font5x10,
}

#[derive(Copy, Clone)]
pub struct Properties {
    // number of lines
    pub height: usize,

    // number of characters per line
    pub width: usize,

    // LCD's font
    pub font: Font,
}