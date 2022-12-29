use super::registers::CpuRegisters;
use crate::{
    mem::Memory,
    performer::{
        block::Block,
        top_level::table::jump_top_level,
    },
};

pub struct Unit<'a> {
    id: usize,

    pub memory: &'a Memory,
    pub registers: CpuRegisters,
}

impl<'a> Unit<'a> {
    pub fn execute(&mut self) {
        let insn = self
            .memory
            .read_u32_le(self.memory.abs_to_rel(self.registers.pc));
        jump_top_level(self, Block::new(insn));

        self.registers.pc += 4;
        self.registers.general[31] = 0;
    }
}

impl<'a> Unit<'a> {
    pub const fn id(&self) -> usize {
        self.id
    }

    pub fn jump_to(&mut self, to: u64) {
        self.registers.pc = to;
    }

    pub const fn new(id: usize, memory: &'a Memory) -> Self {
        Self {
            id,
            registers: CpuRegisters {
                general: [0; 32],
                sp: 0,
                pc: 0,
            },
            memory,
        }
    }
}
