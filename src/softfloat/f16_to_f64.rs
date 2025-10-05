use super::{
    expF16UI, float16_t, float64_t, fracF16UI, packToF64, packToF64UI, signF16UI,
    softfloat_commonNaNToF64UI, softfloat_f16UIToCommonNaN, softfloat_normSubnormalF16Sig,
};

#[must_use]
pub const fn f16_to_f64(a: float16_t) -> (float64_t, u8) {
    let uiA = a.v;
    let sign = signF16UI(uiA);
    let mut exp = expF16UI(uiA);
    let mut frac = fracF16UI(uiA);

    if exp == 0x1F {
        if frac != 0 {
            let (commonNaN, flags) = softfloat_f16UIToCommonNaN(uiA);
            return (
                float64_t {
                    v: softfloat_commonNaNToF64UI(&commonNaN),
                },
                flags,
            );
        }
        return (
            float64_t {
                v: packToF64UI(sign, 0x7FF, 0),
            },
            0,
        );
    }

    if exp == 0 {
        if frac == 0 {
            return (packToF64(sign, 0, 0), 0);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(frac);
        exp = normExpSig.exp.wrapping_sub(1);
        frac = normExpSig.sig;
    }

    return (
        packToF64(sign, (exp as i16).wrapping_add(0x3F0), (frac as u64) << 42),
        0,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_to_f64() {
        struct softfloat_f16_to_f64_TestCase {
            a: u16,
            result: u64,
            flags: u8,
        }

        let cases = [
            softfloat_f16_to_f64_TestCase {
                a: 0x3C00,
                result: 0x3FF0000000000000,
                flags: 0,
            },
            softfloat_f16_to_f64_TestCase {
                a: 0x0000,
                result: 0x0000000000000000,
                flags: 0,
            },
            softfloat_f16_to_f64_TestCase {
                a: 0x8000,
                result: 0x8000000000000000,
                flags: 0,
            },
            softfloat_f16_to_f64_TestCase {
                a: 0x4000,
                result: 0x4000000000000000,
                flags: 0,
            },
            softfloat_f16_to_f64_TestCase {
                a: 0x3800,
                result: 0x3FE0000000000000,
                flags: 0,
            },
            softfloat_f16_to_f64_TestCase {
                a: 0xBC00,
                result: 0xBFF0000000000000,
                flags: 0,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_to_f64(float16_t { v: c.a });
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
