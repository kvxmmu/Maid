pub use paste as __paste;
pub use static_assertions as __sa;

#[macro_export]
macro_rules! define_named_conv_enum {
    (
        $(
            enum $name:ident: $repr:ty {
                $(
                    $variant:ident = $value:expr
                ),*
                $(,)?
            } range($pat:pat)
        )*
    ) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            #[repr($repr)]
            pub enum $name {
                $(
                    $variant = $value
                ),*
            }

            impl $name {
                $crate::__paste::paste! {
                    pub const fn [<try_from_ $repr>](from: $repr) -> Option<$name> {
                        $crate::__sa::assert_eq_align!($name, $repr);
                        $crate::__sa::assert_eq_size!($name, $repr);

                        // TODO: Pattern irrefutability check?
                        if !matches!(from, $pat) {
                            return None;
                        }

                        union U {
                            i: $repr,
                            e: $name
                        }

                        // SAFETY: this is safe since we're sure about enum representation
                        // and checked range above
                        // but possibly unsound because range can contain invalid pattern
                        Some(unsafe { U { i: from }.e })
                    }
                }
            }
        )*
    };
}

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
                let (_, $insn_name) = decoder.decode_next().unwrap();

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
