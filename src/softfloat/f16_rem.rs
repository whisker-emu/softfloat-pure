use crate::softfloat::{
    defaultNaNF16UI, expF16UI, float16_t, fracF16UI, signF16UI, softfloat_approxRecip32_1,
    softfloat_flag_invalid, softfloat_normRoundPackToF16, softfloat_normSubnormalF16Sig,
    softfloat_propagateNaNF16,
};

#[must_use]
pub const fn f16_rem(
    a: float16_t,
    b: float16_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    let signA = signF16UI(a.v);
    let mut expA = expF16UI(a.v);
    let mut sigA = fracF16UI(a.v);

    let mut expB = expF16UI(b.v);
    let mut sigB = fracF16UI(b.v);

    if expA == 0x1F {
        if (sigA != 0) || ((expB == 0x1F) && (sigB != 0)) {
            return softfloat_propagateNaNF16(a.v, b.v);
        }
        return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
    }
    if expB == 0x1F {
        if sigB != 0 {
            return softfloat_propagateNaNF16(a.v, b.v);
        }
        return (a, 0);
    }

    if expB == 0 {
        if sigB == 0 {
            return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }
    if expA == 0 {
        if sigA == 0 {
            return (a, 0);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }

    let mut rem = sigA | 0x0400;
    let mut q: u16;
    sigB |= 0x0400;
    let mut expDiff = expA.wrapping_sub(expB);
    if expDiff < 1 {
        if expDiff < -1 {
            return (a, 0);
        }

        sigB <<= 3;
        if expDiff != 0 {
            rem <<= 2;
            q = 0;
        } else {
            rem <<= 3;
            q = (sigB <= rem) as u16;
            if q != 0 {
                rem = rem.wrapping_sub(sigB);
            }
        }
    } else {
        let recip32 = softfloat_approxRecip32_1((sigB as u32) << 21);
        rem <<= 4;
        expDiff = expDiff.wrapping_sub(31);
        sigB <<= 3;

        let mut q32: u32;
        loop {
            q32 = ((rem as u64).wrapping_mul(recip32 as u64) >> 16) as u32;
            if expDiff < 0 {
                break;
            }
            rem = (q32 as u16).wrapping_mul(sigB).wrapping_neg();
            expDiff -= 29;
        }

        q32 >>= !(expDiff as u16) & 31;
        q = q32 as u16;
        rem = (rem.wrapping_shl((expDiff as u32).wrapping_add(30)))
            .wrapping_sub(q.wrapping_mul(sigB));
    }

    // ------------------------------------------------------------------------

    let mut altRem: u16;
    loop {
        altRem = rem;
        q = q.wrapping_add(1);
        rem = rem.wrapping_sub(sigB);

        if (rem & 0x8000) != 0 {
            break;
        }
    }
    let meanRem = rem.wrapping_add(altRem);
    if (meanRem & 0x8000) != 0 || (meanRem == 0 && ((q & 1) != 0)) {
        rem = altRem;
    }
    let mut signRem = signA;
    if 0x8000 <= rem {
        signRem = !signRem;
        rem = rem.wrapping_neg();
    }

    return softfloat_normRoundPackToF16(signRem, expB as i16, rem, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_rem() {
        struct softfloat_f16_rem_TestCase {
            a: u16,
            b: u16,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f16_rem_TestCase {
                a: 0x4500,
                b: 0x4000,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_rem_TestCase {
                a: 0x3C00,
                b: 0x3C00,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_rem_TestCase {
                a: 0x4300,
                b: 0x4000,
                result: 0xB800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_rem_TestCase {
                a: 0x4700,
                b: 0x4200,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_rem(
                float16_t { v: c.a },
                float16_t { v: c.b },
                c.roundingMode,
                c.detectTininess,
            );
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
