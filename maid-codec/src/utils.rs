pub const fn sign_extend64(value: u64, sign_index: u64) -> u64 {
    let shift = 63 - sign_index;
    (((value << shift) as i64) >> shift) as _
}
