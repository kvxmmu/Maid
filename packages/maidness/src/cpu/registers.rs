pub struct CpuRegisters {
    pub general: [u64; 32],

    pub sp: u64,
    pub pc: u64,
}

impl CpuRegisters {
    pub fn read_general(&mut self, index: u8) -> u64 {
        self.general[index as usize]
    }

    pub fn write_general(&mut self, index: u8, value: u64) {
        self.general[index as usize] = value;
    }
}
