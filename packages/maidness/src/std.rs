macro_rules! uses_alloc {
    (
        $(
            $path:path
        ),*
        $(,)?
    ) => {
        $(
            #[cfg(feature = "no-std")]
            pub use ::alloc::{ $path };

            #[cfg(not(feature = "no-std"))]
            pub use ::std::{ $path };
        )*
    };
}

uses_alloc!(vec::Vec, vec);
