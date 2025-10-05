use super::{float16_t, signF16UI, softfloat_addMagsF16, softfloat_subMagsF16};

#[must_use]
pub const fn f16_sub(
    a: float16_t,
    b: float16_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    if signF16UI(a.v ^ b.v) {
        softfloat_addMagsF16(a.v, b.v, roundingMode, detectTininess)
    } else {
        softfloat_subMagsF16(a.v, b.v, roundingMode, detectTininess)
    }
}
