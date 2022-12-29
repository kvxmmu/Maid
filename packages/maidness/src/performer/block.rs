use static_assertions::{
    assert_eq_align,
    assert_eq_size,
};

use crate::std::{
    fmt::{
        self,
        Display,
        Formatter,
    },
    slice,
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    inner: u32,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{:0<8x} | {:0<8x}",
            self.inner,
            self.inner.to_be()
        ))
    }
}

impl Block {
    pub const fn take_single_as_is(self, index: u32) -> u32 {
        self.inner & (1 << index)
    }

    pub const fn take_single_bool(self, index: u32) -> bool {
        (self.inner & (1 << index)) != 0
    }

    pub const fn take_single(self, index: u32) -> Self {
        Self::new(self.take_single_as_is(index))
    }

    pub const fn appended(self, with: u32, no: u32) -> Self {
        Block::new((self.inner << no) | with)
    }

    pub const fn take_from(self, from: u32, count: u32) -> Self {
        Self::new(self.take_from_u32(from, count))
    }

    pub const fn take_from_to_u32(self, from: u32, to: u32) -> u32 {
        self.take_from_u32(from, to - from + 1)
    }

    pub const fn take_from_to(self, from: u32, to: u32) -> Self {
        Self::new(self.take_from_to_u32(from, to))
    }

    pub const fn take_from_u32(self, from: u32, count: u32) -> u32 {
        if count == 0 {
            0
        } else {
            let mask = 1 << (count - 1);
            (self.inner >> from) & ((mask - 1) | mask)
        }
    }
}

impl Block {
    pub const fn from_u32_slice(u32_slice: &[u32]) -> &[Block] {
        // SAFETY: since `Block` is declared as transparent to u32,
        // so, casting from underlying type to wrapped `Block` is
        // legal.

        // This is necessary because we **must** ensure that
        // `Block` and `u32` has same layout.
        assert_eq_align!(Block, u32);
        assert_eq_size!(Block, u32);

        unsafe {
            slice::from_raw_parts(
                u32_slice.as_ptr() as *const Block,
                u32_slice.len(),
            )
        }
    }
}

impl Block {
    pub const fn into_inner(self) -> u32 {
        self.inner
    }

    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }
}

impl From<u32> for Block {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<Block> for u32 {
    fn from(value: Block) -> Self {
        value.inner
    }
}
