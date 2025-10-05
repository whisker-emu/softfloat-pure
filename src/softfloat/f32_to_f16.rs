use super::{
    expF32UI, float16_t, float32_t, fracF32UI, packToF16, packToF16UI, signF32UI,
    softfloat_commonNaNToF16UI, softfloat_f32UIToCommonNaN, softfloat_roundPackToF16,
};

#[must_use]
pub const fn f32_to_f16(a: float32_t, roundingMode: u8, detectTininess: u8) -> (float16_t, u8) {
    let uiA = a.v;
    let sign = signF32UI(uiA);
    let exp = expF32UI(uiA);
    let frac = fracF32UI(uiA);

    if exp == 0xFF {
        if frac != 0 {
            let (commonNaN, flags) = softfloat_f32UIToCommonNaN(uiA);
            return (
                float16_t {
                    v: softfloat_commonNaNToF16UI(commonNaN),
                },
                flags,
            );
        }
        return (
            float16_t {
                v: packToF16UI(sign, 0x1F, 0),
            },
            0,
        );
    }

    let frac16 = (frac >> 9) | ((frac & 0x1FF) != 0) as u32;
    if ((exp as u32) | frac16) == 0 {
        return (packToF16(sign, 0, 0), 0);
    }

    return softfloat_roundPackToF16(
        sign,
        (exp as i16).wrapping_sub(0x71),
        (frac16 | 0x4000) as u16,
        roundingMode,
        detectTininess,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f32_to_f16() {
        struct softfloat_f32_to_f16_TestCase {
            a: u32,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f32_to_f16_TestCase {
                a: 0x3F800000,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_to_f16_TestCase {
                a: 0x00000000,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_to_f16_TestCase {
                a: 0x80000000,
                result: 0x8000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_to_f16_TestCase {
                a: 0x40000000,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_to_f16_TestCase {
                a: 0x3F000000,
                result: 0x3800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f32_to_f16_TestCase {
                a: 0xBF800000,
                result: 0xBC00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f32_to_f16(float32_t { v: c.a }, c.roundingMode, c.detectTininess);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
