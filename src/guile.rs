use crate::floc::Floc;

/// Guile is not supported in this port; setup is a no-op that reports success.
///
/// Safe: the `flocp` argument is ignored, so no pointer is ever dereferenced —
/// the function carries no safety preconditions despite its C-ABI signature.
pub fn guile_gmake_setup(_flocp: *const Floc) -> ::core::ffi::c_int {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_is_a_safe_noop_returning_one() {
        // Callable from safe code (no `unsafe`), ignores its arg, reports success.
        assert_eq!(guile_gmake_setup(::core::ptr::null()), 1);
    }
}
