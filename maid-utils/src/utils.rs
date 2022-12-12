pub const LOG2_TAG_GRANULE: u64 = 4;
pub const TAG_GRANULE: u64 = 1 << LOG2_TAG_GRANULE;

/// Mark branch as cold path. May help compiler with proper
/// branch optimizations. Actually does nothing.
#[cold]
pub const fn cold_path() {}

/// Same as `cold_path`, but short-hand for something like
/// cold_path() + return
#[cold]
pub const fn cold_value<T>(v: T) -> T {
    v
}

/// Short-hand for returning unlikely-happen errors
pub const fn cold_err<T, E>(e: E) -> Result<T, E> {
    cold_value(Err(e))
}

/// Mark that condition is likely to be false
pub const fn unlikely(cond: bool) -> bool {
    if cond {
        cold_value(true)
    } else {
        false
    }
}

/// Mark that condition is likely to be true
pub const fn likely(cond: bool) -> bool {
    if cond {
        true
    } else {
        cold_value(false)
    }
}

/// Sign extend unsigned integer
pub const fn sign_extend64(value: u64, sign_index: u64) -> u64 {
    let shift = 63 - sign_index;
    (((value << shift) as i64) >> shift) as _
}

pub const fn highest_set_bit(v: u64) -> u64 {
    64 - v.leading_zeros() as u64 - 1
}

pub const fn replicate1_32(bit: u64) -> u64 {
    (!0) * bit
}

pub const fn replicate_pow2_64(bits: u64, exp: u64) -> u64 {
    // result = bits + bits * 2^exp + ... + bits * 2^(64 / exp)
    // result = bits * (1 + 2^exp + ... + 2^(64 / exp))
    // t = 64 / exp
    // result = bits * (1 + 2^exp + ... + 2^t)
    //
    // b1 = 1
    // q = 2^exp
    // b(t) = b1 * q^(t - 1) = (2^exp)^(t - 1) = 2^(exp * (t -
    // 1)) b(t) = 2 ^ (exp * (t - 1))
    //
    // sum(t) = (b(t) * 2^exp - 1) / (2^exp - 1)
    // result = bits * sum(t)

    let q = 1 << exp;

    // variables are trivially reducible, just try
    // it in your workbook
    let sumt = 0xFF_FF_FF_FF_FF_FF_FF_FF / (q - 1);

    bits * sumt
}

// TODO: write more generalized version with power of two
// since 32 is power of two it is only divisible by 2^n
// where `n` is natural or zero
pub const fn replicate2_32(bits: u64) -> u64 {
    // Actually we have geometric progression, since we need to
    // replicate 2bits to 32bit, this will look like:
    // bits + bits * 2^exp + bits * 2^(2 * exp) + ... + bits *
    // 2^16 = result so, we can take out the b:
    // bits * (2^0 + 2^2 + 2^4 + ... + 2^16) = result
    //
    // b1 = 1
    // q = 2^2
    // b(16) = b1 * q^15 = 2^30
    // sum(16) = (b(16)*4 - 1) / 3
    // then we have:
    // result = bits * ((2^32 - 1) / 3)

    bits * (((1 << 32) - 1) / 3)
}

// -> (bits(M) bits(N))
pub const fn decode_bit_masks<const M: u64>(
    imm_n: u64,
    imms: u64,
    immr: u64,
    immediate: bool,
) -> (u64, u64) {
    let len = highest_set_bit((imm_n << 6) | !imms);

    assert!(len >= 1);
    assert!(M >= (1 << len));

    let levels: u64 = (1 << len) - 1; // bits(6)

    if immediate && ((imms & levels) == levels) {
        panic!("undefined");
    }

    let (s, r) = (imms & levels, immr & levels);
    let diff = s.wrapping_sub(r);

    let esize = 1 << len;
    let d = diff & (len - 1);

    let (welem, telem) = (ones(s + 1), ones(d + 1));
    // bits(esize)

    let wmask = welem.rotate_right(r as u32);
    let tmask = telem;

    (wmask, tmask)
}

pub const fn ones(n: u64) -> u64 {
    let mask = 1 << (n - 1);
    let lhs = mask - 1;
    lhs | mask
}

pub const fn lsl64(x: u64, bits: u64, shift: u64) -> u64 {
    if shift == 0 {
        x
    } else {
        lsl64_c(x, bits, shift).0
    }
}

pub const fn lsl64_c(x: u64, bits: u64, shift: u64) -> (u64, bool) {
    assert!(shift > 0, "shift <= 0");

    let extended = x << shift;

    let (end, start) = (shift + bits - 1, shift);
    let mask = (1 << (end - start)) - 1;

    let result = (extended >> start) & mask;
    let carry_out = extended & ((1 << shift) - 1);

    (result, carry_out != 0)
}
