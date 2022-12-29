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
    stub,   // 0b000
    stub,   // 0b001
    stub,   // 0b010
    stub,   // 0b011
    stub,   // 0b100
    stub,   // 0b101
    stub,   // 0b110
    stub,   // 0b111
];

pub(crate) fn jump_dp_immediate(unit: &mut Unit, block: Block) {
    let op0 = block.take_from_to_u32(23, 25);

    DP_IMMEDIATE_TABLE[op0 as usize](unit, block);
}
