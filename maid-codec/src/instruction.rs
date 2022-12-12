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
pub struct ArithmeticImmOp {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogicalImmOp {
    pub imm: u64,
    pub register_type: RegisterType,

    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveWideImm {
    pub register_type: RegisterType,
    pub imm16: u64,
    pub pos: u64,
    pub rd: u8,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Udf,

    AdrpImm { imm: u64, rd: u8 },
    AdrImm { imm: u64, rd: u8 },

    AddImm(ArithmeticImmOp),
    SubImm(ArithmeticImmOp),

    TaggedAddImm(TaggedArithmeticOp),
    TaggedSubImm(TaggedArithmeticOp),

    AndImm(LogicalImmOp),
    OrrImm(LogicalImmOp),
    EorImm(LogicalImmOp),
    AndsImm(LogicalImmOp),

    MovNImmediate(MoveWideImm),
    MovZImmediate(MoveWideImm),
    MovKImmediate(MoveWideImm),

    Unallocated { block: Block },
    UnallocatedSpan { span: BufSpan },
}

impl RegisterType {
    pub const fn as_data_size(self) -> u64 {
        32 * ((self as u64) + 1)
    }

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
