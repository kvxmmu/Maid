#[macro_export]
macro_rules! define_body_structs {
    ($(
        $(
            #[derive( $($derive_path:path),* )]
        )?
        struct $name:ident = $(
            $field:ident: $type:ty
        ),*
    )*) => {
        $(
            #[derive(
                Debug, Clone, PartialEq, Eq,
                $(
                    $($derive_path),*
                )?
            )]
            pub struct $name {
                $(
                    pub $field: $type
                ),*
            }
        )*
    };
}

#[macro_export]
macro_rules! test_insn {
    ($($name:ident($insn:expr) |$insn_name:ident| $e:expr);* $(;)?) => {
        $(
            #[test]
            fn $name() {
                let mut insn: u32 = $insn;
                insn = insn.to_be();
                let data: [u8; 4] = [
                    (insn & 0xff) as _,
                    ((insn >> 8 ) & 0xff) as _,
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

#[macro_export]
macro_rules! try_const {
    ($condition:expr) => {
        match $condition {
            Ok(o) => o,
            Err(e) => {
                return Err(e);
            }
        }
    };
}