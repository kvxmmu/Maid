use crate::{
    block::Block,
    replicate2_32,
    utils::sign_extend64,
};

#[test]
fn test_replicate2_32() {
    assert_eq!(replicate2_32(0b10), 0xAA_AA_AA_AA);
    assert_eq!(replicate2_32(0b11), 0xFF_FF_FF_FF);
}

#[test]
fn test_sext64() {
    let full = ((1 << 63) - 1) | (1 << 63);

    assert_eq!(sign_extend64(0b111, 2), full);
    assert_eq!(sign_extend64(0b111, 3), 0b111);
    assert_eq!(sign_extend64(0b101, 2), full & !(1 << 1));
}

#[test]
fn test_block_appended() {
    let block = Block::new(10);
    assert_eq!(block.appended(2, 2), Block::new((10 << 2) | 2));
}

#[test]
fn test_u32_slice_to_block_slice_cast() {
    let u32_slice: &[u32] = &[10, 20, 30, 40];
    let block_slice = Block::from_u32_slice(u32_slice);

    assert_eq!(
        block_slice,
        &[
            Block::new(10),
            Block::new(20),
            Block::new(30),
            Block::new(40)
        ]
    );
}
