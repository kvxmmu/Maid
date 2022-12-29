use super::unit::Unit;
use crate::{
    mem::Memory,
    std::Vec,
};

pub struct Cpu<'a> {
    pub units: Vec<Unit<'a>>,
    pub memory: &'a Memory,
}

impl<'a> Cpu<'a> {
    pub fn new(number_of_units: usize, memory: &'a Memory) -> Self {
        let units: Vec<_> = (0..number_of_units)
            .map(|id| Unit::new(id, memory))
            .collect();

        Self { units, memory }
    }
}
