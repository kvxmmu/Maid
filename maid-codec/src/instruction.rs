use {
    maid_utils::block::*,
    static_assertions::const_assert,
};

pub type BufSpan = std::ops::Range<usize>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegisterType {
    W = 0,
    X = 1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArithmeticOp {
    pub rd: u8,
    pub rn: u8,
    pub imm: u64,
    pub register: RegisterType,
    pub set_flags: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaggedArithmeticOp {
    pub rd: u8,
    pub rn: u8,
    pub offset: u64,
    pub uimm4: u64,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Udf,

    AdrpImm { imm: u64, rd: u8 },
    AdrImm { imm: u64, rd: u8 },

    AddImm(ArithmeticOp),
    SubImm(ArithmeticOp),

    TaggedAddImm(TaggedArithmeticOp),
    TaggedSubImm(TaggedArithmeticOp),

    Unallocated { block: Block },
    UnallocatedSpan { span: BufSpan },
}

impl RegisterType {
    pub const fn from_sf(sf: bool) -> Self {
        union TransmuteEnum {
            sf: bool,
            reg: RegisterType,
        }

        // This is required to check that requirements
        // for transmutation are met
        const_assert!((RegisterType::W as u8) == 0);
        const_assert!((RegisterType::X as u8) == 1);

        // SAFETY: this is safe since bool has only 0 or 1 bit
        // pattern, so transmuting between RegisterType &
        // bool is always safe.
        unsafe { TransmuteEnum { sf }.reg }
    }
}
