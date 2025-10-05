use super::{float16_t, softfloat_isSigNaNF16UI};

#[must_use]
#[inline]
pub const fn f16_isSignalingNaN(a: float16_t) -> bool {
    return softfloat_isSigNaNF16UI(a.v);
}
