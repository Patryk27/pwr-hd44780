use crate::{Error, Lcd, Result};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn validate(&self, lcd: &dyn Lcd) -> Result<()> {
        let dimensions = lcd.dimensions();

        if self.x < dimensions.x && self.y < dimensions.y {
            Ok(())
        } else {
            Err(Error::CursorOutOfBounds {
                cursor: *self,
                screen_dimensions: dimensions,
            })
        }
    }
}