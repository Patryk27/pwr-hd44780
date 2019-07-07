pub enum CommandValue {
    Clear = 0x01,
    Home = 0x02,
    SetEntryMode = 0x04,
    SetDisplayFlags = 0x08,
    // SetCursorShift = 0x10, // @todo not implemented
    SetFunctions = 0x20,
    SetCGRamAddress = 0x40,
    SetDDRamAddress = 0x80,
}

pub enum Command {
    Clear,
    Home,

    SetEntryMode {
        enable_shift: bool,
        increment_counter: bool,
    },

    SetDisplayFlags {
        cursor_blinking: bool,
        cursor_visible: bool,
        text_visible: bool,
    },

    SetFunctions {
        font_5x10: bool,
        height: usize,
        eight_bit_bus: bool,
    },

    SetCGRamAddress {
        address: u8,
    },

    SetDDRamAddress {
        address: u8,
    },
}