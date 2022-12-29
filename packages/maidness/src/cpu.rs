pub struct Cpu {
    registers: [u64; 31],
}

impl Cpu {
    pub const fn new() -> Self {
        Self { registers: [0; 31] }
    }
}
