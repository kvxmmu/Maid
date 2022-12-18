#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    EndOfData,
    InvalidLengthOfData { length: usize },
}
