use maid_utils::{
    block::Block,
    sign_extend64,
};

use crate::{
    body::UnconditionalBranch,
    instruction::Instruction,
};

pub const fn decode(block: Block) -> Instruction {
    let op0 = block.take_from_to_u32(29, 31);
    let op1 = block.take_from_to_u32(12, 25);
    let op2 = block.take_from_to_u32(0, 4);

    match op0 {
        // Conditional branch (immediate)
        0b010 => todo!(),

        // Bunch of instructions
        0b110 => todo!(),

        // Unconditional branch (immediate)
        0b100 | 0b000 => {
            let op = block.take_single_bool(31);
            let imm26 = block.take_from_to_u32(0, 25);
            let branch = UnconditionalBranch {
                offset: sign_extend64((imm26 << 2) as _, 27),
            };

            if op {
                Instruction::BlImm(branch)
            } else {
                Instruction::BImm(branch)
            }
        }

        // Compare and branch / test and branch
        0b101 | 0b001 => todo!(),

        _ => todo!(),
    }
}
