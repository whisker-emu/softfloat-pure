use super::{float16_t, isNaNF16UI, softfloat_flag_invalid};

#[must_use]
pub const fn f16_eq_signaling(a: float16_t, b: float16_t) -> (bool, u8) {
    if isNaNF16UI(a.v) || isNaNF16UI(b.v) {
        return (false, softfloat_flag_invalid);
    }
    return ((a.v == b.v) || ((a.v | b.v) << 1) == 0, 0);
}
