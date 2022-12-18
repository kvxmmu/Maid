use {
    crate::instruction::{
        ArithmeticImmOp,
        BitfieldImm,
        ExtractImm,
        Instruction,
        LogicalImmOp,
        MoveWideImm,
        RegisterType,
        TaggedArithmeticOp,
    },
    maid_utils::{
        block::*,
        decode_bit_masks,
        lsl64,
        sign_extend64,
        PossiblyUndefined,
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

            let imm = if sh { imm12 << 12 } else { imm12 };
            let op = ArithmeticImmOp {
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
            let uimm4 = block.take_from_to_u32(10, 13) as u8;

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

            let rn = block.take_from_to_u32(5, 9) as u8;
            let rd = block.take_from_to_u32(0, 4) as u8;

            let immr = block.take_from_to_u32(16, 21) as u64;
            let imms = block.take_from_to_u32(10, 15) as u64;

            let register_type = RegisterType::from_sf(sf);
            let PossiblyUndefined::Defined((imm, _)) = decode_bit_masks(
                n as u64,
                imms,
                immr,
                true,
                register_type.as_data_size(),
            ) else {
                return Instruction::Udf;
            };

            let logical = LogicalImmOp {
                imm,
                register_type,
                rn,
                rd,
            };

            match opc {
                // And
                0b00 => Instruction::AndImm(logical),

                // Orr
                0b01 => Instruction::OrrImm(logical),

                // Eor
                0b10 => Instruction::EorImm(logical),

                // Ands
                0b11 => Instruction::AndsImm(logical),

                _ => unreachable!(),
            }
        }

        // Move wide (immediate)
        0b101 => {
            let opc = block.take_from_to_u32(29, 30);
            let hw = block.take_from_to_u32(21, 22);

            if (opc == 0b01) || matches!(hw, 0b10 | 0b11) {
                return Instruction::Unallocated { block };
            }

            let imm16 = block.take_from_to_u32(5, 20);
            let rd = block.take_from_to_u32(0, 4) as u8;
            let register_type = RegisterType::from_sf(sf);

            if matches!(register_type, RegisterType::W) && ((hw >> 1) == 1)
            {
                return Instruction::Udf;
            }

            let pos = hw << 4;
            let mov = MoveWideImm {
                rd,
                register: register_type,
                imm16: imm16 as u16,
                pos: pos as _,
            };

            match opc {
                // movn
                0b00 => Instruction::MovNImmediate(mov),

                // movz
                0b10 => Instruction::MovZImmediate(mov),

                // movk
                0b11 => Instruction::MovKImmediate(mov),

                _ => unreachable!(),
            }
        }

        // Bitfield
        0b110 => {
            let opc = block.take_from_to_u32(29, 30);
            let immr = block.take_from_to_u32(16, 21);
            let imms = block.take_from_to_u32(10, 15);

            let rn = block.take_from_to_u32(5, 9) as u8;
            let rd = block.take_from_to_u32(0, 4) as u8;
            let n = block.take_single_bool(22);
            let register = RegisterType::from_sf(sf);

            let PossiblyUndefined::Defined((wmask, tmask)) = decode_bit_masks(
                n as _,
                imms as _,
                immr as _,
                false,
                register.as_data_size(),
            ) else {
                return Instruction::Udf;
            };

            let bitfield = BitfieldImm {
                wmask,
                tmask,
                rd,
                rn,
                immr: immr as _,
                imms: imms as _,
            };

            match opc {
                _ if !sf && n => Instruction::Unallocated { block },

                0b00 => Instruction::SbfmImm(bitfield),
                0b01 => Instruction::BfmImm(bitfield),
                0b10 => Instruction::UbfmImm(bitfield),

                _ => Instruction::Unallocated { block },
            }
        }

        // Extract
        0b111 => {
            let op21 = block.take_from_to_u32(29, 30);
            let (rn, rd, rm) = (
                block.take_from_to_u32(5, 9) as u8,
                block.take_from_to_u32(0, 4) as u8,
                block.take_from_to_u32(16, 20) as u8,
            );
            let imms = block.take_from_to_u32(10, 15) as u8;
            let o0 = block.take_single_bool(21);

            if (op21 == 0) && !o0 {
                Instruction::ExtrImm(ExtractImm {
                    rd,
                    rn,
                    rm,
                    lsb: imms as _,
                    register: RegisterType::from_sf(sf),
                })
            } else {
                Instruction::Unallocated { block }
            }
        }

        _ => Instruction::Udf,
    }
}
