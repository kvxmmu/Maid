use maid_declmacro::{
    define_body_structs,
    define_named_conv_enum,
};

define_named_conv_enum! {
    enum DcpsIndex: u8 {
        _1 = 0b01,
        _2 = 0b10,
        _3 = 0b11,
    } range(0b01..=0b11)

    enum RegisterType: u8 {
        W = 0,
        X = 1,
    } range(0..=1)

    enum ConditionBits: u8 {
        ZIsOn = 0b000,
        CIsOn = 0b001,
        NIsOn = 0b010,
        VIsOn = 0b011,

        CIsOnAndZIsOff = 0b100,

        NEqV = 0b101,
        NEqVAndZIsOff = 0b110,

        True = 0b111,
    } range(0b000..=0b111)
}

define_body_structs! {
    #[derive(Copy)]
    struct SysRegMove =
        sys_op0: u8,
        sys_op1: u8,

        sys_crn: u8,
        sys_crm: u8,
        sys_op2: u8,
        rt: u8

    struct ExceptionGenImm =
        imm16: u16

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
