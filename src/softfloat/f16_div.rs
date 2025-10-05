use super::{
    defaultNaNF16UI, expF16UI, float16_t, fracF16UI, packToF16, packToF16UI, signF16UI,
    softfloat_approxRecip_1k0s, softfloat_approxRecip_1k1s, softfloat_flag_infinite,
    softfloat_flag_invalid, softfloat_normSubnormalF16Sig, softfloat_propagateNaNF16,
    softfloat_roundPackToF16,
};

#[must_use]
pub const fn f16_div(
    a: float16_t,
    b: float16_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    let signA = signF16UI(a.v);
    let mut expA = expF16UI(a.v);
    let mut sigA = fracF16UI(a.v);
    let signB = signF16UI(b.v);
    let mut expB = expF16UI(b.v);
    let mut sigB = fracF16UI(b.v);
    let signZ = signA ^ signB;
    if expA == 0x1F {
        if sigA != 0 {
            return softfloat_propagateNaNF16(a.v, b.v);
        }
        if expB == 0x1F {
            if sigB != 0 {
                return softfloat_propagateNaNF16(a.v, b.v);
            }
            return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
        }
        return (packToF16(signZ, 0x1F, 0), 0);
    }
    if expB == 0x1F {
        if sigB != 0 {
            return softfloat_propagateNaNF16(a.v, b.v);
        }
        return (packToF16(signZ, 0, 0), 0);
    }
    if expB == 0 {
        if sigB == 0 {
            if (expA as u16) | sigA == 0 {
                return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
            }
            return (packToF16(signZ, 0x1F, 0), softfloat_flag_infinite);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }
    if expA == 0 {
        if sigA == 0 {
            return (packToF16(signZ, 0, 0), 0);
        }
        let normExpSig = softfloat_normSubnormalF16Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }
    let mut expZ = (expA as i16).wrapping_sub(expB as i16).wrapping_add(0xE);
    sigA |= 0x0400;
    sigB |= 0x0400;
    if sigA < sigB {
        expZ = expZ.wrapping_sub(1);
        sigA <<= 5;
    } else {
        sigA <<= 4;
    }
    let index = (sigB >> 6) & 0xF;
    let r0 = softfloat_approxRecip_1k0s[index as usize].wrapping_sub(
        ((softfloat_approxRecip_1k1s[index as usize] as u32).wrapping_mul((sigB & 0x3F) as u32)
            >> 10) as u16,
    );
    let mut sigZ = (((sigA as u32).wrapping_mul(r0 as u32)) >> 16) as u16;
    let mut rem = (sigA << 10).wrapping_sub(sigZ.wrapping_mul(sigB));
    sigZ = sigZ.wrapping_add(((rem as u32).wrapping_mul(r0 as u32) >> 26) as u16);
    sigZ = sigZ.wrapping_add(1);
    if (sigZ & 7) == 0 {
        sigZ &= !1;
        rem = (sigA << 10).wrapping_sub(sigZ.wrapping_mul(sigB));
        if (rem & 0x8000) != 0 {
            sigZ = sigZ.wrapping_sub(2);
        } else if rem != 0 {
            sigZ |= 1;
        }
    }
    return softfloat_roundPackToF16(signZ, expZ, sigZ, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_div() {
        struct softfloat_f16_div_TestCase {
            a: u16,
            b: u16,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f16_div_TestCase {
                a: 0x3C00,
                b: 0x3C00,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x4000,
                b: 0x3C00,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x3C00,
                b: 0x4000,
                result: 0x3800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x4400,
                b: 0x4000,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0xBC00,
                b: 0x3C00,
                result: 0xBC00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x3C00,
                b: 0xBC00,
                result: 0xBC00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0xC000,
                b: 0xBC00,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x4200,
                b: 0x4000,
                result: 0x3E00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x0000,
                b: 0x3C00,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x3C00,
                b: 0x7C00,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x7C00,
                b: 0x3C00,
                result: 0x7C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_div_TestCase {
                a: 0x3C00,
                b: 0x0000,
                result: 0x7C00,
                flags: softfloat_flag_infinite,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_div(
                float16_t { v: c.a },
                float16_t { v: c.b },
                c.roundingMode,
                c.detectTininess,
            );
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
