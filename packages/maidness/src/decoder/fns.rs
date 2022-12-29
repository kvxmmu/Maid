use super::block::Block;
use crate::cpu::unit::Unit;

pub type TableFn = for<'a, 'b> fn(unit: &'a mut Unit<'b>, insn: Block);

pub fn unallocated(_: &mut Unit, _: Block) {}

#[track_caller]
pub fn stub(_: &mut Unit, insn: Block) {
    unimplemented!("Stub at {}", insn);
}

static _ASSERT_MATCHES1: TableFn = unallocated;
static _ASSERT_MATCHES2: TableFn = stub;
