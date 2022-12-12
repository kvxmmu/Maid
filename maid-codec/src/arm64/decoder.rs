use {
    super::{
        detail::*,
        error::DecodeError,
    },
    crate::instruction::Instruction,
    maid_utils::{
        block::*,
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
        let block = Block::new(match self.try_peek_u32() {
            Ok(b) => b,
            Err(DecodeError::InvalidLengthOfData { length }) => {
                return Ok(Instruction::UnallocatedSpan {
                    span: self.advanced..(self.advanced + length),
                });
            }

            Err(e) => return Err(e),
        });

        Ok(match block.take_from_to_u32(25, 28) {
            0b0000 => reserved::decode(block),
            0b0010 => sve_encodings::decode(block),

            0b1000 | 0b1001 => data_processing_imm::decode(block),
            0b1010 | 0b1011 => branches_exc_sys::decode(block),

            0b0100 | 0b0110 | 0b1100 | 0b1110 => {
                loads_and_stores::decode(block)
            }

            0b0101 | 0b1101 => data_processing_register::decode(block),
            0b0111 | 0b1111 => data_processing_fp_simd::decode(block),

            0b0001 | 0b011 => Instruction::Unallocated { block },
            _ => todo!(),
        })
    }
}

impl<'a> BufferedDecoder<'a> {
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

impl BufferedDecoder<'_> {
    pub const fn try_peek_u32(&self) -> DecodeResult<u32> {
        match self.buffer.len() {
            0 => cold_err(DecodeError::EndOfData),
            length @ 1..=3 => {
                cold_err(DecodeError::InvalidLengthOfData { length })
            }
            4.. => {
                let res = ((self.buffer[0] as u32) << 24)
                    | ((self.buffer[1] as u32) << 16)
                    | ((self.buffer[2] as u32) << 8)
                    | (self.buffer[3] as u32);
                Ok(res)
            }

            // SAFETY: since there's no gap between 0 | 1..=3, 4.. patterns
            // this branch is actually will never happen even if
            // size_of::<usize>() == 1
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }
}
