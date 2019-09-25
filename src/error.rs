use rppal::i2c::Error as I2cError;

use crate::Point;

pub enum Error {
    CommunicationError(Box<dyn std::error::Error>),

    CharOutOfBounds {
        char: u8,
    },

    CursorOutOfBounds {
        cursor: Point,
        screen_dimensions: Point,
    },
}

impl From<I2cError> for Error {
    fn from(err: I2cError) -> Self {
        Error::CommunicationError(Box::new(err))
    }
}