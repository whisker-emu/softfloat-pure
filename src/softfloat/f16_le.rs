use super::{float16_t, isNaNF16UI, signF16UI, softfloat_flag_invalid};

#[inline]
#[must_use]
pub const fn f16_le(a: float16_t, b: float16_t) -> (bool, u8) {
    if isNaNF16UI(a.v) || isNaNF16UI(b.v) {
        return (false, softfloat_flag_invalid);
    }
    let signA = signF16UI(a.v);
    let signB = signF16UI(b.v);
    (
        if signA != signB {
            signA || ((a.v | b.v) << 1) == 0
        } else {
            a.v == b.v || signA ^ (a.v < b.v)
        },
        0,
    )
}
