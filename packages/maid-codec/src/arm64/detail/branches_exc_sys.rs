use maid_utils::{
    block::Block,
    sign_extend64,
};

use crate::{
    body::{
        CompareAndBranchImm,
        CondBranchImm,
        ConditionBits,
        RegisterType,
        UnconditionalBranch,
    },
    instruction::Instruction,
};

pub const fn decode(block: Block) -> Instruction {
    let op0 = block.take_from_to_u32(29, 31);
    let op1 = block.take_from_to_u32(12, 25);
    let op2 = block.take_from_to_u32(0, 4);

    match op0 {
        // Conditional branch (immediate)
        0b010 => {
            if (op1 & (1 << 13)) != 0 {
                return Instruction::Udf;
            }

            let (false, o1) =
                (block.take_single_bool(4), block.take_single_bool(24)) else {
                return Instruction::Unallocated { block };
            };

            let imm19 = block.take_from_to_u32(5, 23);
            let offset = sign_extend64((imm19 << 2) as u64, 20);
            let Some(cond) = ConditionBits::try_from_u8(
                block.take_from_to_u32(0, 3) as _
            ) else {
                return Instruction::Udf;
            };
            let cond_branch = CondBranchImm { offset, cond };

            if o1 {
                Instruction::BCCond(cond_branch)
            } else {
                Instruction::BCond(cond_branch)
            }
        }

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
        0b101 | 0b001 => {
            if (op1 & (1 << 13)) == 0 {
                // Compare and branch
                let register =
                    RegisterType::from_sf(block.take_single_bool(31));
                let imm19 = block.take_from_to_u32(5, 23);
                let rt = block.take_from_to_u32(0, 4) as u8;

                let cmp_branch = CompareAndBranchImm {
                    rt,
                    offset: sign_extend64((imm19 << 2) as u64, 20),
                    register,
                };

                if block.take_single_bool(24) {
                    Instruction::Cbz(cmp_branch)
                } else {
                    Instruction::Cbnz(cmp_branch)
                }
            } else {
                // Test and branch
                let imm14 = block.take_from_to_u32(5, 18);
                let rt = block.take_from_to_u32(0, 4) as u8;
                let b40 = block.take_from_to_u32(19, 23);
                let b5 = block.take_from_to_u32(31, 31);

                let bit_pos = ((b5 << 5) | b40) as u8;
                let offset = sign_extend64((imm14 << 2) as u64, 15);

                Instruction::TestZeroImmediate {
                    offset,
                    rt,
                    bit_pos,
                    not: block.take_single_bool(24),
                }
            }
        }

        _ => todo!(),
    }
}
