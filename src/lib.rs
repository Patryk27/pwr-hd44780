/// A convenient, high-level driver for the Hd44780 display.
/// Supports both the `I2C` and `GPIO` interfaces + has a buffered implementation.
///
/// # Example
///
/// ```rust
/// let mut lcd_dev = LinuxI2CDevice::new("/dev/i2c-1", 0x27).unwrap();
/// let mut lcd_interface = hd44780::interface::i2c::I2CInterface::new(&mut lcd_dev);
/// let mut lcd = hd44780::DirectHd44780::new(&mut lcd_interface);
///
/// lcd.print("Hello World! :-)");
/// ```
///
/// # Copyright
/// 2018 by Patryk Wychowaniec, MIT.

extern crate i2cdev;

pub mod frontend;
pub mod interface;

pub trait Hd44780 {
    /// Clears the screen and moves cursor at (0, 0).
    fn clear(&mut self);

    /// Moves the cursor at (0, 0).
    fn home(&mut self);

    /// Moves the cursor at given position.
    /// When passed an invalid coordinates (eg. beyond the screen), does nothing.
    fn move_at(&mut self, y: usize, x: usize);

    /// Prints text at current cursor's position.
    fn print(&mut self, str: String);

    /// Enables / disables the backlight.
    fn set_backlight(&mut self, enabled: bool);

    /// Enables / disables blinking the cursor.
    /// Blinking = whole 5x8 / 5x10 character is blinking,
    fn set_cursor_blinking(&mut self, enabled: bool);

    /// Enables / disables the cursor.
    /// Visible = only bottom of the character is blinking.
    fn set_cursor_visible(&mut self, enabled: bool);

    /// Shows / hides the text.
    fn set_text_visible(&mut self, enabled: bool);

    /// Creates a custom character from given bitmap.
    ///
    /// Each array item in given bitmap represents a single line, of which only the last 5 bits are
    /// important - rest is ignored.
    ///
    /// `idx` must be from range `<0, 7>` (that is: only 8 custom characters are possible, that's a
    /// limit imposed by the designers of the Hd44780).
    ///
    /// When passed an invalid `idx`, does nothing.
    ///
    /// # Example
    ///
    /// ```rust
    /// lcd.set_char(0, [
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
    /// lcd.print(String::from("\x00"));
    /// ```
    fn set_char(&mut self, idx: u8, lines: [u8; 8]);

    /// Returns screen's width (number of characters per line).
    fn get_width(&mut self) -> usize;

    /// Returns screen's height (number of lines).
    fn get_height(&mut self) -> usize;
}

#[derive(Copy, Clone, PartialEq)]
pub enum Font {
    Font5x8,
    Font5x10,
}

#[derive(Copy, Clone)]
pub struct Properties {
    // number of characters per line
    pub width: usize,

    // number of lines
    pub height: usize,

    // LCD's font
    pub font: Font,
}