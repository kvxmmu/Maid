use crate::{
    cpu::unit::Unit,
    decoder::{
        block::*,
        fns::{
            stub,
            TableFn,
        },
    },
};

#[rustfmt::skip]
static DP_IMMEDIATE_TABLE: [TableFn; 8] = [
    stub,   // 0b000 (PC-rel.addressing)
    stub,   // 0b001 (PC-rel.addressing)
    stub,   // 0b010 (Add/subtract - immediate)
    stub,   // 0b011 (Add/subtract - immediate, with tags)
    stub,   // 0b100 (Logical - immediate)
    stub,   // 0b101 (Move wide - immediate)
    stub,   // 0b110 (Bitfield)
    stub,   // 0b111 (Extract)
];

pub(crate) fn jump_dp_immediate(unit: &mut Unit, block: Block) {
    let op0 = block.take_from_to_u32(23, 25);

    DP_IMMEDIATE_TABLE[op0 as usize](unit, block);
}
