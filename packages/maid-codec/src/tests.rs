use maid_declmacro::test_insn;

use crate::{
    arm64::decoder::BufferedDecoder,
    body::UnconditionalBranch,
    instruction::{
        ArithmeticImmOp,
        BitfieldImm,
        ExtractImm,
        Instruction,
        LogicalImmOp,
        MoveWideImm,
        RegisterType,
    },
};

test_insn! {
    test_b_imm(0x19000014) |insn| {
        assert_eq!(insn, Instruction::BImm(UnconditionalBranch {
            offset: 100
        }));
    };

    test_sbfm_imm(0x4A784A93) |insn| {
        assert_eq!(insn, Instruction::SbfmImm(BitfieldImm {
            imms: 30,
            immr: 10,
            rd: 10,
            rn: 2,
            wmask: 18428729675202166783,
            tmask: 31
        }));
    };

    test_extr32_imm(0x20288213) |insn| {
        assert_eq!(insn, Instruction::ExtrImm(ExtractImm {
            rn: 1,
            rd: 0,
            rm: 2,
            lsb: 10,
            register: RegisterType::W
        }));
    };

    test_extr64_imm(0x2078C293) |insn| {
        assert_eq!(insn, Instruction::ExtrImm(ExtractImm {
            rn: 1,
            rd: 0,
            rm: 2,
            lsb: 30,
            register: RegisterType::X
        }));
    };

    test_movk_immediate(0x410180F2) |insn| {
        assert_eq!(insn, Instruction::MovKImmediate(MoveWideImm {
            register: RegisterType::X,
            imm16: 10,
            pos: 0,
            rd: 1
        }));
    };

    test_movn_immediate(0x41018092) |insn| {
        assert_eq!(insn, Instruction::MovNImmediate(MoveWideImm {
            register: RegisterType::X,
            imm16: 10,
            pos: 0,
            rd: 1
        }));
    };

    test_movz_immediate(0x410180D2) |insn| {
        assert_eq!(insn, Instruction::MovZImmediate(MoveWideImm {
            rd: 1,
            imm16: 10,
            pos: 0,
            register: RegisterType::X
        }));
    };

    test_adr_immediate(0x2A030010) |insn| {
        assert_eq!(insn, Instruction::AdrImm { imm: 100, rd: 10 });
    };

    test_adrp_immediate(0x0A0000B0) |insn| {
        assert_eq!(insn, Instruction::AdrpImm { imm: 4096, rd: 10 });
    };

    test_add_immediate(0x4BB10491) |insn| {
        assert_eq!(insn, Instruction::AddImm(ArithmeticImmOp {
            rd: 11,
            rn: 10,
            imm: 300,
            register: RegisterType::X,
            set_flags: false
        }));
    };

    test_add_immediate_from12(0x8BB10491) |insn| {
        assert_eq!(insn, Instruction::AddImm(ArithmeticImmOp {
            rd: 11,
            rn: 12,
            imm: 300,
            register: RegisterType::X,
            set_flags: false
        }));
    };

    test_add_immediate_from10_to4(0x44B10491) |insn| {
        assert_eq!(insn, Instruction::AddImm(ArithmeticImmOp {
            rd: 4,
            rn: 10,
            imm: 300,
            register: RegisterType::X,
            set_flags: false
        }));
    };

    test_ands_immediate(0x400074F2) |insn| {
        assert_eq!(insn, Instruction::AndsImm(LogicalImmOp {
            imm: 4096,
            register: RegisterType::X,
            rn: 2,
            rd: 0
        }));
    };

    test_eor_immediate(0x400074D2) |insn| {
        assert_eq!(insn, Instruction::EorImm(LogicalImmOp {
            imm: 4096,
            register: RegisterType::X,
            rn: 2,
            rd: 0
        }));
    };

    test_orr_immediate_big(0x400074B2) |insn| {
        assert_eq!(insn, Instruction::OrrImm(LogicalImmOp {
            imm: 4096,
            register: RegisterType::X,
            rn: 2,
            rd: 0
        }));
    };

    test_orr_immediate(0x400440B2) |insn| {
        assert_eq!(insn, Instruction::OrrImm(LogicalImmOp {
            imm: 3,
            register: RegisterType::X,
            rn: 2,
            rd: 0
        }));
    };

    test_orr_immediate_32bit(0x40040032) |insn| {
        assert_eq!(insn, Instruction::OrrImm(LogicalImmOp {
            imm: 3,
            register: RegisterType::W,
            rn: 2,
            rd: 0
        }));
    };

    test_and_immediate_32bit(0x00040012) |insn| {
        assert_eq!(insn, Instruction::AndImm(LogicalImmOp {
            imm: 3,
            register: RegisterType::W,
            rn: 0,
            rd: 0
        }));
    };

    test_and_immediate(0x00044092) |insn| {
        assert_eq!(insn, Instruction::AndImm(LogicalImmOp {
            rd: 0,
            rn: 0,
            register: RegisterType::X,
            imm: 3,
        }));
    };

    test_sub_immediate(0x44B104D1) |insn| {
        assert_eq!(insn, Instruction::SubImm(ArithmeticImmOp {
            rd: 4,
            rn: 10,
            imm: 300,
            register: RegisterType::X,
            set_flags: false
        }));
    };

    test_adds_immediate(0x44B104B1) |insn| {
        assert_eq!(insn, Instruction::AddImm(ArithmeticImmOp {
            rd: 4,
            rn: 10,
            imm: 300,
            register: RegisterType::X,
            set_flags: true
        }));
    }
}

#[test]
fn test_register_type_sf() {
    assert_eq!(RegisterType::from_sf(true), RegisterType::X);
    assert_eq!(RegisterType::from_sf(false), RegisterType::W);
}
