use crate::Result;

pub trait Bus {
    fn write_command(&mut self, byte: u8) -> Result<()>;
    fn write_data(&mut self, byte: u8) -> Result<()>;
    fn enable_backlight(&mut self, enabled: bool) -> Result<()>;
    fn size(&self) -> BusSize;
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BusSize {
    FourBit,
    EightBit,
}