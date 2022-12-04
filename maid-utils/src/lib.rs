/// Mark branch as cold path. May help compiler with proper
/// branch optimizations. Actually does nothing.
#[cold]
pub const fn cold_path() {}

/// Same as `cold_path`, but short-hand for something like
/// cold_path() + return
#[cold]
pub const fn cold_value<T>(v: T) -> T {
    v
}

/// Short-hand for returning unlikely-happen errors
pub const fn cold_err<T, E>(e: E) -> Result<T, E> {
    cold_value(Err(e))
}

/// Mark that condition is likely to be false
pub const fn unlikely(cond: bool) -> bool {
    if cond {
        cold_value(true)
    } else {
        false
    }
}

/// Mark that condition is likely to be true
pub const fn likely(cond: bool) -> bool {
    if cond {
        true
    } else {
        cold_value(false)
    }
}
