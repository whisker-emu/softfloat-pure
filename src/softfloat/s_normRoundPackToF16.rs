use crate::softfloat::{
    float16_t, packToF16, softfloat_countLeadingZeros32,
    softfloat_roundPackToF16,
};

#[must_use]
pub const fn softfloat_normRoundPackToF16(
    sign: bool,
    exp: i16,
    sig: u32,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    let shiftDist = softfloat_countLeadingZeros32(sig).wrapping_sub(1) as i8;
    let exp = exp.wrapping_sub(shiftDist as i16);
    if 4 <= shiftDist && exp < 0x1D {
        return (
            packToF16(
                sign,
                if sig != 0 { exp as i8 } else { 0 },
                (sig << shiftDist.wrapping_sub(4)) as u16,
            ),
            0,
        );
    }
    return softfloat_roundPackToF16(sign, exp, (sig << shiftDist) as u16, roundingMode, detectTininess);
}
