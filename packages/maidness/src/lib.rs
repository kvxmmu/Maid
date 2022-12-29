#![cfg_attr(feature = "no-std", no_std)]
#[cfg(feature = "no-std")]
extern crate alloc;

pub mod mem;

pub mod error;
pub mod utils;

pub mod cpu;

pub mod decoder;

mod std;
