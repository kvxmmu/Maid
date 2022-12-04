use {
    crate::instruction::Instruction,
    maid_utils::block::Block,
};

pub const fn decode(block: Block) -> Instruction {
    let op0 = block.take_from_u32(31, 3);
    let op1 = block.take_from_u32(24, 9);

    match (op0, op1) {
        (0, 0) => Instruction::Udf,
        _ => Instruction::Unallocated { block },
    }
}
