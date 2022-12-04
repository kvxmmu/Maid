use {
    super::error::DecodeError,
    crate::instruction::Instruction,
    maid_utils::{
        cold_err,
        cold_path,
    },
};

pub type DecodeResult<T> = Result<T, DecodeError>;

pub struct BufferedDecoder<'a> {
    buffer: &'a [u8],
    advanced: usize,
}

impl<'a> BufferedDecoder<'a> {
    pub const fn try_peek(&self) -> DecodeResult<Instruction> {
        let block = match self.try_peek_u32() {
            Ok(b) => b,
            Err(DecodeError::InvalidLengthOfData { length }) => {
                return Ok(Instruction::Unallocated {
                    span: self.advanced..(self.advanced + length),
                });
            }

            Err(e) => return Err(e),
        };

        todo!()
    }

    pub fn decode_next(&mut self) -> DecodeResult<Instruction> {
        let result = self.try_peek();
        self.advance_by(4);

        result
    }

    pub fn advance_by(&mut self, mut by: usize) {
        if by > self.buffer.len() {
            cold_path();
            by = self.buffer.len();
        }

        self.buffer = &self.buffer[by..];
        self.advanced += by;
    }

    pub const fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            advanced: 0,
        }
    }
}

impl<'a> BufferedDecoder<'a> {
    pub const fn try_peek_u32(&self) -> DecodeResult<u32> {
        match self.buffer.len() {
            0 => cold_err(DecodeError::EndOfData),
            length @ 1..=3 => {
                cold_err(DecodeError::InvalidLengthOfData { length })
            }
            4.. => {
                let res = ((self.buffer[0] as u32) << 24)
                    | ((self.buffer[1] as u32) << 16);
                Ok(res)
            }

            // SAFETY: since there's no gap between 0 | 1..=3, 4.. patterns
            // this branch is actually will never happen even if
            // size_of::<usize>() == 1
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
