use maid_utils::block::*;

pub type BufSpan = std::ops::Range<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterType {
    W,
    X,
}

#[derive(Debug)]
pub enum Instruction {
    Udf,

    AdrpImm { imm: u64, rd: u8 },
    AdrImm { imm: u64, rd: u8 },

    Unallocated { block: Block },
    UnallocatedSpan { span: BufSpan },
}
