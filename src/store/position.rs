#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub file_index: u64,
    pub start_position: u64,
    pub length: u64,
}

impl From<(u64, u64, u64)> for Position {
    fn from(
        (file_index, start_position, end_position): (u64, u64, u64),
    ) -> Self {
        Position {
            file_index,
            start_position,
            length: end_position - start_position,
        }
    }
}
