#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterType {
    W,
    X,
}

#[derive(Debug)]
pub enum Instruction {
    Udf,
}
