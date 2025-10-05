use super::{float16_t, softfloat_isSigNaNF16UI};

#[inline]
#[must_use]
pub const fn f16_isSignalingNaN(a: float16_t) -> bool {
    return softfloat_isSigNaNF16UI(a.v);
}
