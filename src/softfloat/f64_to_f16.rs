use super::{
    expF64UI, float16_t, float64_t, fracF64UI, packToF16, packToF16UI, signF64UI,
    softfloat_commonNaNToF16UI, softfloat_f64UIToCommonNaN, softfloat_roundPackToF16,
    softfloat_shortShiftRightJam64,
};

#[must_use]
pub const fn f64_to_f16(a: float64_t, roundingMode: u8, detectTininess: u8) -> (float16_t, u8) {
    let uiA = a.v;
    let sign = signF64UI(uiA);
    let exp = expF64UI(uiA);
    let frac = fracF64UI(uiA);

    if exp == 0x7FF {
        if frac != 0 {
            let (commonNaN, flags) = softfloat_f64UIToCommonNaN(uiA);
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

    let frac16 = softfloat_shortShiftRightJam64(frac, 38) as u16;
    if ((exp as u16) | frac16) == 0 {
        return (packToF16(sign, 0, 0), 0);
    }

    return softfloat_roundPackToF16(
        sign,
        exp.wrapping_sub(0x3F1),
        frac16 | 0x4000,
        roundingMode,
        detectTininess,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_to_f16() {
        struct softfloat_f64_to_f16_TestCase {
            a: u64,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f64_to_f16_TestCase {
                a: 0x3FF0000000000000,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f64_to_f16_TestCase {
                a: 0x0000000000000000,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f64_to_f16_TestCase {
                a: 0x8000000000000000,
                result: 0x8000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f64_to_f16_TestCase {
                a: 0x4000000000000000,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f64_to_f16_TestCase {
                a: 0x3FE0000000000000,
                result: 0x3800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f64_to_f16_TestCase {
                a: 0xBFF0000000000000,
                result: 0xBC00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f64_to_f16(float64_t { v: c.a }, c.roundingMode, c.detectTininess);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
