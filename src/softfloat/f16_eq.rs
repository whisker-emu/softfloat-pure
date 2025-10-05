use super::{float16_t, isNaNF16UI, softfloat_flag_invalid, softfloat_isSigNaNF16UI};

#[must_use]
pub const fn f16_eq(a: float16_t, b: float16_t) -> (bool, u8) {
    if isNaNF16UI(a.v) || isNaNF16UI(b.v) {
        if softfloat_isSigNaNF16UI(a.v) || softfloat_isSigNaNF16UI(b.v) {
            return (false, softfloat_flag_invalid);
        }
        return (false, 0);
    }
    return (a.v == b.v || ((a.v | b.v) << 1) == 0, 0);
}
