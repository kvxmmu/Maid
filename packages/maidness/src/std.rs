macro_rules! uses {
    (
        $prefix:ident: $(
            $path:path
        ),*
    ) => {
        $(
            #[cfg(feature = "no-std")]
            pub use ::$prefix::{ $path };

            #[cfg(not(feature = "no-std"))]
            pub use ::std::{ $path };
        )*
    };
}

uses!(alloc: vec::Vec, vec);
uses!(core: sync::atomic::AtomicU8, fmt, slice);
