use crate::utils::sign_extend64;

#[test]
fn test_sext64() {
    let full = ((1 << 63) - 1) | (1 << 63);

    assert_eq!(sign_extend64(0b111, 2), full);
    assert_eq!(sign_extend64(0b111, 3), 0b111);
    assert_eq!(sign_extend64(0b101, 2), full & !(1 << 1));
}
