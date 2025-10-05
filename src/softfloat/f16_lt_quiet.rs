use super::{float16_t, isNaNF16UI, signF16UI, softfloat_flag_invalid, softfloat_isSigNaNF16UI};

#[must_use]
pub const fn f16_lt_quiet(a: float16_t, b: float16_t) -> (bool, u8) {
    if isNaNF16UI(a.v) || isNaNF16UI(b.v) {
        if softfloat_isSigNaNF16UI(a.v) || softfloat_isSigNaNF16UI(b.v) {
            return (false, softfloat_flag_invalid);
        }
        return (false, 0);
    }
    let signA = signF16UI(a.v);
    let signB = signF16UI(b.v);
    return (
        if signA != signB {
            signA && ((a.v | b.v) << 1) != 0
        } else {
            a.v != b.v && signA ^ (a.v < b.v)
        },
        0,
    );
}
