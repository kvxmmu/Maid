use {
    crate::arm64::block::Block,
    std::ops::Range,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterType {
    W,
    X,
}

#[derive(Debug)]
pub enum Instruction {
    Udf,
    Unallocated { span: Range<usize> },
}
