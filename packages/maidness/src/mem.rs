use crate::std::{
    vec,
    Vec,
};

#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
    start: u64,
}

impl Memory {
    pub fn new(start: u64, size: usize) -> Self {
        Self {
            data: vec![0; size],
            start,
        }
    }
}
