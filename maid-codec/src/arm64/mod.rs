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

pub mod block;
pub mod decoder;
pub mod encoder;

pub mod error;

#[cfg(test)]
mod tests;