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
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    pub const fn base_addr(&self) -> u64 {
        self.start
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Memory {
    pub fn read_u32_le(&self, rel: usize) -> u32 {
        let slice = &self.data[rel..rel + 4];

        ((slice[3] as u32) << 24)
            | ((slice[2] as u32) << 16)
            | ((slice[1] as u32) << 8)
            | (slice[0] as u32)
    }
}

impl Memory {
    pub fn new(start: u64, size: usize) -> Self {
        Self {
            data: vec![0; size],
            start,
        }
    }
}

impl Memory {
    pub const fn abs_to_rel(&self, abs: u64) -> usize {
        (self.start - abs) as _
    }

    pub const fn rel_to_abs(&self, rel: usize) -> u64 {
        self.start + rel as u64
    }
}
