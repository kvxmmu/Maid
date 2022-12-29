use crate::error::MemOpError;

#[inline]
#[cold]
pub(crate) fn out_of_bounds<T>() -> Result<T, MemOpError> {
    Err(MemOpError::OutOfBounds)
}
