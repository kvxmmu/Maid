use crate::{
    arm64::decoder::BufferedDecoder,
    instruction::{
        ArithmeticOp,
        Instruction,
        RegisterType,
    },
};

macro_rules! test_insn {
    ($($name:ident($insn:expr) |$insn_name:ident| $e:expr);* $(;)?) => {
        $(
            #[test]
            fn $name() {
                let insn: u32 = $insn;
                let data: [u8; 4] = [
                    (insn & 0xff) as _,
                    ((insn >> 8) & 0xff) as _,
                    ((insn >> 16) & 0xff) as _,
                    ((insn >> 24) & 0xff) as _,
                ];
                let mut decoder = BufferedDecoder::new(&data);
                let $insn_name = decoder.decode_next().unwrap();

                $e
            }
        )*
    };
}

test_insn! {
    test_adr_immediate(0x2A030010) |insn| {
        assert_eq!(insn, Instruction::AdrImm { imm: 100, rd: 10 });
    };

    test_adrp_immediate(0x0A0000B0) |insn| {
        assert_eq!(insn, Instruction::AdrpImm { imm: 4096, rd: 10 });
    };

    test_add_immediate(0x4BB10491) |insn| {
        assert_eq!(insn, Instruction::AddImm(ArithmeticOp {
            rd: 11,
            rn: 10,
            imm: 300,
            register: RegisterType::X,
            set_flags: false
        }));
    };

    test_add_immediate_from12(0x8BB10491) |insn| {
        assert_eq!(insn, Instruction::AddImm(ArithmeticOp {
            rd: 11,
            rn: 12,
            imm: 300,
            register: RegisterType::X,
            set_flags: false
        }));
    };

    test_add_immediate_from10_to4(0x44B10491) |insn| {
        assert_eq!(insn, Instruction::AddImm(ArithmeticOp {
            rd: 4,
            rn: 10,
            imm: 300,
            register: RegisterType::X,
            set_flags: false
        }));
    };

    test_sub_immediate(0x44B104D1) |insn| {
        assert_eq!(insn, Instruction::SubImm(ArithmeticOp {
            rd: 4,
            rn: 10,
            imm: 300,
            register: RegisterType::X,
            set_flags: false
        }));
    };

    test_adds_immediate(0x44B104B1) |insn| {
        assert_eq!(insn, Instruction::AddImm(ArithmeticOp {
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
