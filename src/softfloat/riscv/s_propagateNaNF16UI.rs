use crate::softfloat::{
    defaultNaNF16UI, defaultNaNF32UI, float16_t, softfloat_flag_invalid, softfloat_isSigNaNF16UI,
};

/*----------------------------------------------------------------------------
| Interpreting `uiA' and `uiB' as the bit patterns of two 16-bit floating-
| point values, at least one of which is a NaN, returns the bit pattern of
| the combined NaN result.  If either `uiA' or `uiB' has the pattern of a
| signaling NaN, the invalid exception is raised.
*----------------------------------------------------------------------------*/
#[inline]
#[must_use]
pub const fn softfloat_propagateNaNF16UI(uiA: u16, uiB: u16) -> (u16, u8) {
    let mut flags = 0;
    if softfloat_isSigNaNF16UI(uiA) || softfloat_isSigNaNF16UI(uiB) {
        flags |= softfloat_flag_invalid;
    }
    return (defaultNaNF16UI, flags);
}

#[inline]
#[must_use]
pub const fn softfloat_propagateNaNF16(uiA: u16, uiB: u16) -> (float16_t, u8) {
    let (ret, flags) = softfloat_propagateNaNF16UI(uiA, uiB);

    (float16_t { v: ret }, flags)
}
