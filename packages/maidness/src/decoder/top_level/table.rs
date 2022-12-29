use super::data_processing_immediate::*;
use crate::{
    cpu::unit::Unit,
    decoder::{
        block::Block,
        fns::*,
    },
};

#[rustfmt::skip]
static TOP_LEVEL_TABLE: [TableFn; 16] = [
    stub,              // 0b0000 (Reserved)
    unallocated,       // 0b0001 (Unallocated)
    stub,              // 0b0010 (SVE encodings)
    stub,              // 0b0011 (Unallocated)
    stub,              // 0b0100 (Loads and Stores)
    stub,              // 0b0101 (Data Processing - Register)
    stub,              // 0b0110 (Loads and Stores)
    stub,              // 0b0111 (Data Processing - Scalar Floating-Point and Advanced SIMD)

    jump_dp_immediate, // 0b1000 (Data Processing - Immediate)
    jump_dp_immediate, // 0b1001 (Data Processing - Immediate)

    stub,              /* 0b1010 (Branches, Exception Generating and System instructions) */
    stub,              /* 0b1011 (Branches, Exception Generating and System instructions) */

    stub,              // 0b1100 (Loads and Stores)
    stub,              // 0b1101 (Data Processing - Register)
    stub,              // 0b1110 (Loads and Stores)
    stub,              // 0b1111 (Data Processing - Scalar Floating-Point and Advanced SIMD)
];

pub fn jump_top_level(unit: &mut Unit, block: Block) {
    let op0 = block.take_from_to_u32(25, 28);

    TOP_LEVEL_TABLE[op0 as usize](unit, block)
}
