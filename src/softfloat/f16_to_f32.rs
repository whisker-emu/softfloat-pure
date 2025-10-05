use super::{
    expF16UI, float16_t, float32_t, fracF16UI, packToF32, packToF32UI, signF16UI,
    softfloat_commonNaNToF32UI, softfloat_f16UIToCommonNaN, softfloat_normSubnormalF16Sig,
};

#[must_use]
pub const fn f16_to_f32(a: float16_t) -> (float32_t, u8) {
    let uiA = a.v;
    let sign = signF16UI(uiA);
    let mut exp = expF16UI(uiA);
    let mut frac = fracF16UI(uiA);

    if exp == 0x1F {
        if frac != 0 {
            let (commonNaN, flags) = softfloat_f16UIToCommonNaN(uiA);
            return (
                float32_t {
                    v: softfloat_commonNaNToF32UI(&commonNaN),
                },
                flags,
            );
        }
        return (
            float32_t {
                v: packToF32UI(sign, 0xFF, 0),
            },
            0,
        );
    }

    if exp == 0 {
        if frac == 0 {
            return (packToF32(sign, 0, 0), 0);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(frac);
        exp = normExpSig.exp.wrapping_sub(1);
        frac = normExpSig.sig;
    }

    return (
        packToF32(sign, (exp as i16).wrapping_add(0x70), (frac as u32) << 13),
        0,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_to_f32() {
        struct softfloat_f16_to_f32_TestCase {
            a: u16,
            result: u32,
            flags: u8,
        }

        let cases = [
            softfloat_f16_to_f32_TestCase {
                a: 0x3C00,
                result: 0x3F800000,
                flags: 0,
            },
            softfloat_f16_to_f32_TestCase {
                a: 0x0000,
                result: 0x00000000,
                flags: 0,
            },
            softfloat_f16_to_f32_TestCase {
                a: 0x8000,
                result: 0x80000000,
                flags: 0,
            },
            softfloat_f16_to_f32_TestCase {
                a: 0x4000,
                result: 0x40000000,
                flags: 0,
            },
            softfloat_f16_to_f32_TestCase {
                a: 0x3800,
                result: 0x3F000000,
                flags: 0,
            },
            softfloat_f16_to_f32_TestCase {
                a: 0xBC00,
                result: 0xBF800000,
                flags: 0,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_to_f32(float16_t { v: c.a });
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
