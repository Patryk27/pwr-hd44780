/// Hand-made Rust driver for the brilliant HD44780 LCDs.
///
/// # License
///
/// Copyright (c) 2018-2019, Patryk Wychowaniec <wychowaniec.patryk@gmail.com>.
/// Licensed under the MIT license.
#[feature(i2c)]
extern crate i2cdev;
#[feature(gpio)]
extern crate rppal;

pub use self::{
    bus::*,
    driver::*,
    error::*,
    font::*,
    properties::*,
    result::*,
};

mod bus;
mod driver;
mod error;
mod font;
mod properties;
mod result;

// @todo "/// - Returns an error if printing this character would overflow current line."
pub trait Hd44780 {
    /// Clears screen's contents and moves cursor at `(0, 0)` (top-left corner).
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
    /// lcd.move_at(2, 4); // Moves cursor at 5th character in the 3rd line
    /// ```
    ///
    /// # Errors
    ///
    /// - When given invalid coordinates (beyond the screen), returns an error and does not touch
    ///   the cursor at all.
    ///
    /// - Returns an error if communication with the LCD fails.
    fn move_at(&mut self, y: usize, x: usize) -> Result<()>;

    /// Prints a single ASCII character at current cursor's position and moves the cursor.
    /// Can be used to print custom-made characters (the ones created with `create_char()`).
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.print_char(2);
    /// ```
    ///
    /// # Safety
    ///
    /// - If given invalid character code, prints junk on the screen.
    ///
    /// - If printing this character would overflow current line, behavior is unspecified (up to the
    ///   LCD itself).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn print_char(&mut self, ch: u8) -> Result<()>;

    /// Moves cursor at given position and prints given ASCII character there.
    /// Can be used to print custom-made characters (ie. the ones created by `create_char()`).
    /// Does not restore cursor to the original position.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.print_char_at(1, 0, 2);
    /// ```
    ///
    /// # Safety
    ///
    /// - If given invalid character code, prints junk on the screen.
    ///
    /// - If printing this character would overflow current line, behavior is unspecified (up to the
    ///   LCD itself).
    ///
    /// # Errors
    ///
    /// - When given invalid coordinates (beyond the screen), returns an error and does not touch
    ///   or print anything at all.
    ///
    /// - Returns an error if communication with the LCD fails.
    fn print_char_at(&mut self, y: usize, x: usize, ch: u8) -> Result<()> {
        self.move_at(y, x)?;
        self.print_char(ch)
    }

    /// Prints an ASCII text at current cursor's position and moves the cursor.
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
    /// - Returns an error if given string contains at least one non-ASCII character.
    ///
    /// - Returns an error if communication with the LCD fails.
    fn print(&mut self, text: &str) -> Result<()> {
        // Extract characters from string
        let chars: Vec<_> = text.chars().collect();

        // Ensure all characters are ASCII
        // @todo

        // Print them
        for ch in chars {
            self.print_char(ch as u8)?;
        }

        Ok(())
    }

    /// Moves cursor at given position and prints given ASCII text there.
    /// Does not restore cursor to the original position.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.print_at(1, 0, "Hello World!");
    /// lcd.print_at(2, 0, format!("Hello, {}!", someone));
    /// ```
    ///
    /// # Safety
    ///
    /// - If printing this text would overflow current line, behavior is unspecified (up to the LCD
    ///   itself).
    ///
    /// # Errors
    ///
    /// - Returns an error if given string contains at least one non-ASCII character.
    ///
    /// - Returns an error if communication with the LCD fails.
    fn print_at(&mut self, y: usize, x: usize, text: &str) -> Result<()> {
        self.move_at(y, x)?;
        self.print(text)
    }

    /// Toggles LCD's backlight.
    ///
    /// # Safety
    ///
    /// - If LCD does not support changing the backlight, nothing happens (this command is a no-op).
    ///
    /// # Errors
    ///
    /// - Returns an error if communication with the LCD fails.
    fn enable_backlight(&mut self, enabled: bool) -> Result<()>;

    /// Toggles cursor box's blinking mode.
    ///
    /// When enabled, the entire character box (5x8 / 5x10 pixels) at current cursor's position will
    /// be blinking with constant speed.
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
    fn enable_text(&mut self, enabled: bool) -> Result<()>;

    /// Creates a custom 5x8/5x10-pixel character from given bitmap.
    ///
    /// @todo
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
    fn create_char(&mut self, idx: u8, lines: [u8; 8]) -> Result<()>;

    /// Returns number of characters (per line) this screen can display.
    fn width(&self) -> usize;

    /// Returns number of lines this screen can display.
    fn height(&self) -> usize;
}



