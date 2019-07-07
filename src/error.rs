pub enum Error {
    CommunicationFailed,

    InvalidCoordinates {
        current_yx: (usize, usize),
        max_yx: (usize, usize),
    },

    InvalidString,
}