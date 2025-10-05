use super::{
    defaultNaNF16UI, exp8_sig16, expF16UI, float16_t, fracF16UI, packToF16, signF16UI,
    softfloat_flag_invalid, softfloat_normSubnormalF16Sig, softfloat_propagateNaNF16,
    softfloat_roundPackToF16,
};

#[must_use]
pub const fn f16_mul(
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
        if sigA != 0 || ((expB == 0x1F) && sigB != 0) {
            return softfloat_propagateNaNF16(a.v, b.v);
        }
        let magBits = (expB as u16) | sigB;
        if magBits == 0 {
            return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
        }
        return (packToF16(signZ, 0x1F, 0), 0);
    }

    let mut normExpSig = exp8_sig16 { exp: 0, sig: 0 };

    if expB == 0x1F {
        if sigB != 0 {
            return softfloat_propagateNaNF16(a.v, b.v);
        }
        let magBits = (expA as u16) | sigA;
        if magBits == 0 {
            return (float16_t { v: defaultNaNF16UI }, softfloat_flag_invalid);
        }
        return (packToF16(signZ, 0x1F, 0), 0);
    }

    if expA == 0 {
        if sigA == 0 {
            return (packToF16(signZ, 0, 0), 0);
        }
        normExpSig = softfloat_normSubnormalF16Sig(sigA);
        expA = normExpSig.exp;
        sigA = normExpSig.sig;
    }

    if expB == 0 {
        if sigB == 0 {
            return (packToF16(signZ, 0, 0), 0);
        }
        normExpSig = softfloat_normSubnormalF16Sig(sigB);
        expB = normExpSig.exp;
        sigB = normExpSig.sig;
    }

    let mut expZ = (expA as i16).wrapping_add(expB as i16).wrapping_sub(0xF);
    sigA = (sigA | 0x0400) << 4;
    sigB = (sigB | 0x0400) << 5;
    let sig32Z: u32 = (sigA as u32).wrapping_mul(sigB as u32);
    let mut sigZ: u16 = (sig32Z >> 16) as u16;
    if (sig32Z & 0xFFFF) != 0 {
        sigZ |= 1;
    }
    if sigZ < 0x4000 {
        expZ -= 1;
        sigZ <<= 1;
    }

    return softfloat_roundPackToF16(signZ, expZ, sigZ, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_mul() {
        struct softfloat_f16_mul_TestCase {
            a: u16,
            b: u16,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f16_mul_TestCase {
                a: 0x3C00,
                b: 0x3C00,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_mul_TestCase {
                a: 0x4000,
                b: 0x4200,
                result: 0x4600,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_mul_TestCase {
                a: 0x3800,
                b: 0x3800,
                result: 0x3400,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_mul_TestCase {
                a: 0xBC00,
                b: 0x3C00,
                result: 0xBC00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_mul_TestCase {
                a: 0x0000,
                b: 0x3C00,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_mul_TestCase {
                a: 0xBC00,
                b: 0xBC00,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_mul(
                float16_t { v: c.a },
                float16_t { v: c.b },
                c.roundingMode,
                c.detectTininess,
            );
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
