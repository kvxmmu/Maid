use maid_utils::block::*;
use static_assertions::const_assert;

pub use crate::body::*;

pub type BufSpan = std::ops::Range<usize>;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Udf,

    AdrpImm {
        imm: u64,
        rd: u8,
    },
    AdrImm {
        imm: u64,
        rd: u8,
    },

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

    SbfmImm(BitfieldImm),
    BfmImm(BitfieldImm),
    UbfmImm(BitfieldImm),

    ExtrImm(ExtractImm),

    BImm(UnconditionalBranch),
    BlImm(UnconditionalBranch),

    Cbz(CompareAndBranchImm),
    Cbnz(CompareAndBranchImm),

    BCond(CondBranchImm),
    BCCond(CondBranchImm),

    Brk {
        imm16: u16,
    },
    Hlt {
        imm16: u16,
    },
    Dcps {
        index: DcpsIndex,
        imm16: u16,
        ll: u8,
    },

    TestZeroImmediate {
        offset: u64,
        rt: u8,
        bit_pos: u8,
        not: bool,
    },

    Svc(ExceptionGenImm),
    Hvc(ExceptionGenImm),
    Smc(ExceptionGenImm),

    MsrReg(SysRegMove),
    Mrs(SysRegMove),

    Ret {
        rn: u8,
    },

    Sys(SysInsn),
    Sysl(SysInsn),

    Unallocated {
        block: Block,
    },
    UnallocatedSpan {
        span: BufSpan,
    },
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
