use crate::softfloat::{
    float16_t, packToF16, packToF16UI, softfloat_flag_inexact, softfloat_flag_overflow,
    softfloat_flag_underflow, softfloat_round_max, softfloat_round_min, softfloat_round_near_even,
    softfloat_round_near_maxMag, softfloat_round_odd, softfloat_shiftRightJam32,
    softfloat_tininess_beforeRounding,
};

#[must_use]
pub const fn softfloat_roundPackToF16(
    sign: bool,
    mut exp: i16,
    mut sig: u16,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    let mut flags = 0;
    let roundNearEven = roundingMode == softfloat_round_near_even;
    let mut roundIncrement: u8 = 0x8;

    if !roundNearEven && roundingMode != softfloat_round_near_maxMag {
        let x = if sign {
            softfloat_round_min
        } else {
            softfloat_round_max
        };

        roundIncrement = if roundingMode == x { 0xF } else { 0 };
    }
    let roundBits = (sig & 0xF) as u8;

    if 0x1D <= (exp as u32) {
        if exp < 0 {
            let isTiny = (detectTininess == softfloat_tininess_beforeRounding)
                || (exp < -1)
                || (sig.wrapping_add(roundIncrement as u16) < 0x8000);

            sig = softfloat_shiftRightJam32(sig as u32, exp.wrapping_neg() as u16) as u16;
            exp = 0;
            let roundBits = (sig & 0xF) as u8;

            if isTiny && roundBits != 0 {
                flags |= softfloat_flag_underflow;
            }
        } else if (0x1D < exp) || (0x8000 <= sig.wrapping_add(roundIncrement as u16)) {
            flags |= softfloat_flag_overflow | softfloat_flag_inexact;
            return (
                float16_t {
                    v: packToF16UI(sign, 0x1F, 0).wrapping_sub((roundIncrement == 0) as u16),
                },
                flags,
            );
        }
    }

    sig = sig.wrapping_add(roundIncrement as u16) >> 4;
    if roundBits != 0 {
        flags |= softfloat_flag_inexact;
        if roundingMode == softfloat_round_odd {
            sig |= 1;
            return (packToF16(sign, exp as i8, sig as u16), flags);
        }
    }
    sig &= !(((roundBits ^ 0x8) == 0) as u16 & (roundNearEven as u16));
    if sig == 0 {
        exp = 0;
    }

    return (packToF16(sign, exp as i8, sig as u16), flags);
}
