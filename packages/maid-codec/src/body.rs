use maid_declmacro::define_body_structs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegisterType {
    W = 0,
    X = 1,
}

define_body_structs! {
    #[derive(Copy)]
    struct UnconditionalBranch = offset: u64

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
