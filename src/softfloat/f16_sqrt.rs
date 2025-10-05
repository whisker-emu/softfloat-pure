use super::{
    defaultNaNF16UI, expF16UI, float16_t, fracF16UI, signF16UI, softfloat_approxRecipSqrt_1k0s,
    softfloat_approxRecipSqrt_1k1s, softfloat_flag_invalid, softfloat_normSubnormalF16Sig,
    softfloat_propagateNaNF16UI, softfloat_roundPackToF16,
};

#[must_use]
pub const fn f16_sqrt(a: float16_t, roundingMode: u8, detectTininess: u8) -> (float16_t, u8) {
    let uiA = a.v;
    let signA = signF16UI(uiA);
    let mut expA = expF16UI(uiA);
    let mut sigA = fracF16UI(uiA);
    // ------------------------------------------------------------------------
    if expA == 0x1F {
        if sigA != 0 {
            let (uiZ, flags) = softfloat_propagateNaNF16UI(uiA, 0);
            return (float16_t { v: uiZ }, flags);
        }
        if !signA {
            return (a, 0);
        }
        return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
    }
    // ------------------------------------------------------------------------
    if signA {
        if 0 == ((expA as u16) | sigA) {
            return (a, 0);
        }
        return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
    }
    // ------------------------------------------------------------------------
    if expA == 0 {
        if sigA == 0 {
            return (a, 0);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    // ------------------------------------------------------------------------
    let expZ = ((expA.wrapping_sub(0xF)) >> 1).wrapping_add(0xE);
    expA &= 1;
    sigA |= 0x0400;
    let index = ((sigA >> 6) & 0xE) + (expA as u16);
    let r0 = (softfloat_approxRecipSqrt_1k0s[index as usize] as u16).wrapping_sub(
        (((softfloat_approxRecipSqrt_1k1s[index as usize] as u32)
            .wrapping_mul((sigA & 0x7F) as u32))
            >> 11) as u16,
    );
    let mut ESqrR0 = ((r0 as u32).wrapping_mul(r0 as u32)) >> 1;
    if expA != 0 {
        ESqrR0 >>= 1;
    }
    let sigma0 = (!(ESqrR0.wrapping_mul(sigA as u32) >> 16)) as u16;
    let mut recipSqrt16 = r0.wrapping_add((((r0 as u32).wrapping_mul(sigma0 as u32)) >> 25) as u16);
    if (recipSqrt16 & 0x8000) == 0 {
        recipSqrt16 = 0x8000;
    }
    let mut sigZ = (((sigA as u32) << 5).wrapping_mul(recipSqrt16 as u32) >> 16) as u16;
    if expA != 0 {
        sigZ >>= 1;
    }
    // ------------------------------------------------------------------------
    sigZ = sigZ.wrapping_add(1);
    if (sigZ & 7) == 0 {
        let shiftedSigZ = sigZ >> 1;
        let negRem = shiftedSigZ.wrapping_mul(shiftedSigZ);
        sigZ &= !1;
        if (negRem & 0x8000) != 0 {
            sigZ |= 1;
        } else if negRem != 0 {
            sigZ = sigZ.wrapping_sub(1);
        }
    }
    return softfloat_roundPackToF16(false, expZ as i16, sigZ, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_sqrt() {
        struct softfloat_f16_sqrt_TestCase {
            a: u16,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f16_sqrt_TestCase {
                a: 0x4400,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_sqrt_TestCase {
                a: 0x3C00,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_sqrt_TestCase {
                a: 0x0000,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_sqrt_TestCase {
                a: 0x4880,
                result: 0x4200,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_sqrt_TestCase {
                a: 0x3400,
                result: 0x3800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_sqrt(float16_t { v: c.a }, c.roundingMode, c.detectTininess);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
