use crate::softfloat::{
    float16_t, packToF16UI, softfloat_countLeadingZeros16, softfloat_roundPackToF16,
};

#[must_use]
pub const fn softfloat_normRoundPackToF16(
    sign: bool,
    exp: i16,
    sig: u16,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    let shiftDist = softfloat_countLeadingZeros16(sig).wrapping_sub(1) as i8;
    let exp = exp.wrapping_sub(shiftDist as i16);
    if (4 <= shiftDist) && ((exp as u32) < 0x1D) {
        return (
            float16_t {
                v: packToF16UI(
                    sign,
                    if sig != 0 { exp as i8 } else { 0 },
                    sig << (shiftDist - 4),
                ),
            },
            0,
        );
    }
    return softfloat_roundPackToF16(sign, exp, sig << shiftDist, roundingMode, detectTininess);
}
