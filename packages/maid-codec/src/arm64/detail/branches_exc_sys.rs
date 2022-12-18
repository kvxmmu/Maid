use maid_utils::{
    block::Block,
    sign_extend64,
};

use crate::{
    body::{
        CompareAndBranchImm,
        CondBranchImm,
        ConditionBits,
        DcpsIndex,
        ExceptionGenImm,
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
        // Bunch of instructions
        0b110 => {
            if (op1 & 0x3000) == 0 {
                // exception generation
                let opc = block.take_from_to_u32(5, 20);
                let op2 = block.take_from_to_u32(2, 4);
                let ll = block.take_from_to_u32(0, 1);

                let imm16 = block.take_from_to_u32(5, 20) as u16;

                if op2 != 0 {
                    return Instruction::Unallocated { block };
                }

                match opc {
                    0b000 => {
                        let exc = ExceptionGenImm { imm16 };
                        return match ll {
                            0b00 => Instruction::Unallocated { block },

                            0b01 => Instruction::Svc(exc),
                            0b10 => Instruction::Hvc(exc),
                            0b11 => Instruction::Smc(exc),

                            _ => unreachable!(),
                        };
                    }

                    0b001 => {
                        return match ll {
                            0b10 | 0b01 | 0b11 => {
                                Instruction::Unallocated { block }
                            }

                            0b00 => Instruction::Brk { imm16 },

                            _ => unreachable!(),
                        }
                    }

                    0b010 => {
                        return match ll {
                            0b10 | 0b01 | 0b11 => {
                                Instruction::Unallocated { block }
                            }

                            0b00 => Instruction::Hlt { imm16 },

                            _ => unreachable!(),
                        }
                    }

                    0b011 | 0b100 | 0b110 | 0b111 => {
                        return Instruction::Unallocated { block }
                    }

                    0b101 => {
                        let Some(index) = DcpsIndex::try_from_u8(ll as u8) else {
                            return Instruction::Unallocated { block }
                        };

                        return Instruction::Dcps {
                            index,
                            imm16,
                            ll: ll as _,
                        };
                    }

                    _ => unreachable!(),
                }
            } else if op1 == 0b01000000110001 {
                // System instructions with register argument
                todo!()
            } else if op1 == 0b01000000110010 && op2 == 0b11111 {
                // Hints
                todo!()
            } else if op1 == 0b01000000110011 {
                // Barriers
                todo!()
            }

            let (pstate_lhs, pstate_rhs) = (op1 >> 7, op1 & 0x0F);
            if (pstate_lhs == 0b0100000) && (pstate_rhs == 0b0100) {
                // PSTATE
                todo!()
            }

            let (sysinf_lhs, sysinf_rhs) =
                ((op1 >> 11), (op1 >> 8) & 0b11);
            if (sysinf_lhs == 0b0100) && (sysinf_rhs == 0b01) {
                // System instructions
                todo!()
            }

            let (sysregmv_lhs, sysregmv_rhs) =
                ((op1 >> 10), (op1 >> 8) & 0b1);
            if (sysregmv_lhs == 0b0100) && (sysregmv_rhs == 1) {
                // System register move
                todo!()
            }

            if (op1 >> 13) == 1 {
                // Unconditional branch (register)
                todo!()
            }

            Instruction::Udf
        }

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
