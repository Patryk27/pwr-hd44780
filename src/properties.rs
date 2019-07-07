#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct LcdProperties {
    pub width: usize,
    pub height: usize,
    pub font: LcdFont,
}