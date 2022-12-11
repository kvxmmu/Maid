use {
    crate::instruction::{
        ArithmeticOp,
        Instruction,
        RegisterType,
        TaggedArithmeticOp,
    },
    maid_utils::{
        block::*,
        lsl64,
        sign_extend64,
        LOG2_TAG_GRANULE,
    },
};

pub const fn decode(block: Block) -> Instruction {
    let op0 = block.take_from_u32(23, 3);
    let sf = block.take_single_bool(31);
    match op0 {
        // PC-rel. addressing
        0b000 | 0b001 => {
            let immhi = block.take_from_to_u32(5, 23) as u64;
            let immlo = block.take_from_to_u32(29, 30) as u64;
            let imm_unextended = (immhi << 2) | immlo;
            let rd = block.take_from_u32(0, 4) as u8;

            if block.take_single_bool(31) {
                Instruction::AdrpImm {
                    imm: sign_extend64(imm_unextended << 12, 63),
                    rd,
                }
            } else {
                Instruction::AdrImm {
                    imm: sign_extend64(imm_unextended, 20),
                    rd,
                }
            }
        }

        // Add/subtract (immediate)
        0b010 => {
            let is_sub = block.take_single_bool(30);
            let s_suffixed = block.take_single_bool(29);

            let imm12 = block.take_from_to_u32(10, 21);
            let rn = block.take_from_to_u32(5, 9) as u8;
            let rd = block.take_from_to_u32(0, 4) as u8;
            let sh = block.take_single_bool(22);

            let imm = if sh { imm12 << 12 } else { imm12 } as u64;
            let op = ArithmeticOp {
                rd,
                rn,
                imm,
                register: RegisterType::from_sf(sf),
                set_flags: s_suffixed,
            };

            if is_sub {
                Instruction::SubImm(op)
            } else {
                Instruction::AddImm(op)
            }
        }

        // Add/subtract (immediate, with tags)
        0b011 => {
            let is_sub = block.take_single_bool(30);
            let s_val = block.take_single_bool(29);
            let o2 = block.take_single_bool(22);

            if matches!(
                (sf, s_val, o2),
                (_, _, true) | (false, _, false) | (true, true, false)
            ) {
                return Instruction::Unallocated { block };
            }

            let uimm6 = block.take_from_to_u32(16, 21) as u64;
            let uimm4 = block.take_from_to_u32(10, 13) as u64;

            let rd = block.take_from_to_u32(0, 4) as u8;
            let rn = block.take_from_to_u32(5, 9) as u8;

            let offset = lsl64(uimm6, 64, LOG2_TAG_GRANULE);

            let operation = TaggedArithmeticOp {
                rd,
                rn,
                offset,
                uimm4,
            };
            if is_sub {
                Instruction::TaggedSubImm(operation)
            } else {
                Instruction::TaggedAddImm(operation)
            }
        }

        // Logical (immediate)
        0b100 => {
            let n = block.take_single_bool(22);
            if !sf && n {
                return Instruction::Unallocated { block };
            }
            let opc = block.take_from_to_u32(29, 30);

            let immr = block.take_from_to_u32(16, 21);
            let imms = block.take_from_to_u32(10, 15);

            match opc {
                // And
                0b00 => {
                    todo!()
                }

                // Orr
                0b01 => {
                    todo!()
                }

                // Eor
                0b10 => {
                    todo!()
                }

                // Ands
                0b11 => {
                    todo!()
                }

                _ => unreachable!(),
            }
        }

        // Move wide (immediate)
        0b101 => {
            todo!()
        }

        // Bitfield
        0b110 => {
            todo!()
        }

        // Extract
        0b111 => {
            todo!()
        }

        _ => todo!(),
    }
}
