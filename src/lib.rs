/// Hand-made driver for HD44780 LCDs.
///
/// # License
///
/// Copyright (c) 2018-2019, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
/// Licensed under the MIT license.

pub use error::Error;
pub use font::Font;
pub use point::Point;
pub use properties::Properties;
pub use result::Result;
pub(crate) use utils::{wait_ms, wait_ns, wait_us};

mod error;
mod font;
pub mod lcds;
mod point;
mod properties;
mod result;
mod utils;

pub trait Lcd {
    /// Clears screen's contents and moves cursor at `(0, 0)` (i.e. top-left corner).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn clear(&mut self) -> Result<()>;

    /// Moves cursor at `(0, 0)` (top-left corner).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn home(&mut self) -> Result<()>;

    /// Moves the cursor at given (zero-based) position.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use pwr_hd44780::Point;
    ///
    /// lcd.goto(Point { x: 2, y: 4 });
    /// ```
    ///
    /// # Errors
    ///
    /// - Returns an error when given invalid coordinates (beyond the screen).
    ///
    /// - Returns an error if communication with the LCD fails.
    fn goto(&mut self, p: Point) -> Result<()>;

    /// Prints a text at current cursor's position and moves the cursor.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.print("Hello World!");
    /// lcd.print(format!("Hello, {}!", someone));
    /// ```
    ///
    /// # Safety
    ///
    /// - If printing this text would overflow current line, behavior is unspecified (up to the LCD
    ///   itself).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn print(&mut self, text: &str) -> Result<()> {
        for ch in text.chars() {
            self.print_char(ch as u8)?;
        }

        Ok(())
    }

    /// Prints a character at current cursor's position and moves the cursor by one unit.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.print_char(100)?; // prints ASCII 'd'
    /// lcd.print_char(2)?; // prints custom character; see: Lcd::create_char()
    /// ```
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn print_char(&mut self, ch: u8) -> Result<()>;

    /// Enables / disables LCD's backlight.
    ///
    /// # Safety
    ///
    /// - If LCD does not support changing the backlight, nothing happens (this command is a no-op).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn enable_backlight(&mut self, enabled: bool) -> Result<()>;

    /// Enables / disables blinking of the cursor's box.
    ///
    /// When enabled, the entire character box (i.e. 5x8 / 5x10 pixels) at current cursor's position
    /// will start blinking with constant speed.
    ///
    /// # Safety
    ///
    /// - If LCD does not support changing the blinking mode, nothing happens (this command is a
    ///   no-op).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn enable_cursor_box_blinking(&mut self, enabled: bool) -> Result<()>;

    /// Toggles cursor line's blinking mode.
    ///
    /// When enabled, bottom of the character box at current cursor's position will be blinking with
    /// constant speed.
    ///
    /// There's no way to change the blinking speed.
    ///
    /// # Safety
    ///
    /// - If LCD does not support changing the blinking mode, nothing happens (this command is a
    ///   no-op).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn enable_cursor_line_blinking(&mut self, enabled: bool) -> Result<()>;

    /// Toggles text visibility.
    ///
    /// When disabled, the entire screen will stop displaying text at once.
    ///
    /// # Safety
    ///
    /// - If LCD does not support changing the text visibility, nothing happens (this command is a
    ///   no-op).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn enable_text_visibility(&mut self, enabled: bool) -> Result<()>;

    /// Creates a custom 5x8/5x10-pixel character from given bitmap.
    ///
    /// Only 8 chars can be created (that's a hardware limit), from indexes 0 up to 7.
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
    /// ])?;
    ///
    /// lcd.print_char(1)?;
    /// ```
    fn create_char(&mut self, idx: u8, bitmap: [u8; 8]) -> Result<()>;

    /// Returns LCD's dimensions:
    /// - `x` coordinate determines number of characters (per line) this screen can display,
    /// - `y` coordinate determines number of lines this screen can display.
    fn dimensions(&self) -> Point;
}



