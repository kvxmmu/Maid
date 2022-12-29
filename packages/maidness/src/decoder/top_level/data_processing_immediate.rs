use crate::{
    cpu::unit::Unit,
    decoder::{
        block::*,
        fns::{
            stub,
            TableFn,
        },
        utils::sign_extend64,
    },
};

#[rustfmt::skip]
static DP_IMMEDIATE_TABLE: [TableFn; 8] = [
    pcrel_addr,   // 0b000 (PC-rel.addressing)
    pcrel_addr,   // 0b001 (PC-rel.addressing)
    stub,         // 0b010 (Add/subtract - immediate)
    stub,         // 0b011 (Add/subtract - immediate, with tags)
    stub,         // 0b100 (Logical - immediate)
    stub,         // 0b101 (Move wide - immediate)
    stub,         // 0b110 (Bitfield)
    stub,         // 0b111 (Extract)
];

fn pcrel_addr(unit: &mut Unit, block: Block) {
    let is_adrp = block.take_single_bool(31);
    let dest = block.take_from_to_u32(0, 4) as u8;

    let immlo = block.take_from_to_u32(29, 30); // bits(2)
    let immhi = block.take_from_to_u32(5, 23); // bits(19)
    let joined = ((immhi << 2) | immlo) as u64; // bits(21)

    if is_adrp {
        let imm = sign_extend64(joined << 12, 32);
        let base = unit.registers.pc & !((1 << 12) - 1);
        unit.registers.write_general(dest, base + imm);
    } else {
        let imm = sign_extend64(joined, 20);
        unit.registers
            .write_general(dest, unit.registers.pc + imm);
    }
}

pub(crate) fn jump_dp_immediate(unit: &mut Unit, block: Block) {
    let op0 = block.take_from_to_u32(23, 25);

    DP_IMMEDIATE_TABLE[op0 as usize](unit, block)
}
