use std::{
    mem,
    slice,
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    inner: u32,
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
        self.take_from_u32(from, to + 1 - from)
    }

    pub const fn take_from_to(self, from: u32, to: u32) -> Self {
        Self::new(self.take_from_to_u32(from, to))
    }

    pub const fn take_from_u32(self, from: u32, count: u32) -> u32 {
        if count == 0 {
            0
        } else {
            let mask = 1 << count;
            (self.inner >> from) & ((mask - 1) | mask)
        }
    }

    pub const fn from_u32_slice(u32_slice: &[u32]) -> &[Block] {
        // SAFETY: since `Block` is declared as transparent to u32,
        // so, casting from underlying type to wrapped `Block` is
        // legal.
        //
        // And one not obvious thing about this cast is that we need
        // to fully reconstruct slice to avoid transmuting between
        // slice types which exact layout is not specified.

        // This is necessary because we **must** ensure that
        // `Block` and `u32` has same layout. Silently stealed from
        // `static_assertions` crate will be replaced with
        // macroses from it if I will write code like that more than
        // once
        const _: [(); 0] =
            [(); mem::size_of::<Block>() - mem::size_of::<u32>()];
        const _: [(); 0] =
            [(); mem::align_of::<Block>() - mem::align_of::<u32>()];

        unsafe {
            slice::from_raw_parts(
                u32_slice.as_ptr() as *const Block,
                u32_slice.len(),
            )
        }
    }

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
