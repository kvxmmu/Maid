pub struct CpuRegisters {
    pub general: [u64; 32],

    pub sp: u64,
    pub pc: u64,
}
