use maidness::{
    cpu::unit::Unit,
    mem::Memory,
};

fn main() {
    let add2 = include_bytes!("../misc/bins/add2.bintest");

    let mut memory = Memory::new(0x80000000, 4096);
    memory.data_mut()[..add2.len()].copy_from_slice(add2);

    let mut unit = Unit::new(0, &memory);
    unit.jump_to(0x80000000);

    println!("Execute: {}", unit.execute());
}
