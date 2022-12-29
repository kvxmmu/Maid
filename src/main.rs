use maidness::{
    cpu::unit::Unit,
    mem::Memory,
};

fn main() {
    let binary: &[u8] = &0x21030010u32.to_be_bytes(); //include_bytes!("../misc/bins/add2.bintest");

    let mut memory = Memory::new(0x80000000, 4096);
    memory.data_mut()[..binary.len()].copy_from_slice(binary);

    let mut unit = Unit::new(0, &memory);

    unit.jump_to(0x80000000);
    unit.execute();

    assert_eq!(unit.registers.read_general(1), 0x80000064);
}
