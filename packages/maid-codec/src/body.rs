use maid_declmacro::define_body_structs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegisterType {
    W = 0,
    X = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ConditionBits {
    ZIsOn = 0b000,
    CIsOn = 0b001,
    NIsOn = 0b010,
    VIsOn = 0b011,

    CIsOnAndZIsOff = 0b100,

    NEqV = 0b101,
    NEqVAndZIsOff = 0b110,

    True = 0b111,
}

define_body_structs! {
    struct CondBranchImm =
        offset: u64,
        cond: ConditionBits

    #[derive(Copy)]
    struct UnconditionalBranch = offset: u64

    struct CompareAndBranchImm =
        rt: u8,
        register: RegisterType,
        offset: u64

    struct BitfieldImm =
        imms: u8,
        immr: u8,
        rd: u8,
        rn: u8,

        wmask: u64,
        tmask: u64

    struct ArithmeticImmOp =
        rd: u8,
        rn: u8,
        imm: u32,
        register: RegisterType,
        set_flags: bool

    struct TaggedArithmeticOp =
        rd: u8,
        rn: u8,

        uimm4: u8,
        offset: u64

    #[derive(Copy)]
    struct ExtractImm =
        rn: u8,
        rd: u8,
        rm: u8,
        lsb: u8,

        register: RegisterType

    struct LogicalImmOp =
        imm: u64,
        register: RegisterType,

        rn: u8,
        rd: u8

    struct MoveWideImm =
        register: RegisterType,
        imm16: u16,
        pos: u64,
        rd: u8
}

impl ConditionBits {
    pub const fn try_from_u8(u: u8) -> Option<Self> {
        union U {
            i: u8,
            e: ConditionBits,
        }

        match u {
            0..=0b111 => {
                // SAFETY: this is safe since we're checked value range
                // above
                Some(unsafe { U { i: u }.e })
            }

            _ => None,
        }
    }
}
