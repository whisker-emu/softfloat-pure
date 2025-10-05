use super::{
    expF16UI, float16_t, fracF16UI, i32_fromNaN, i32_fromNegOverflow, i32_fromPosOverflow,
    signF16UI, softfloat_flag_invalid, softfloat_roundToI32,
};

#[must_use]
pub const fn f16_to_i32(a: float16_t, roundingMode: u8, exact: bool) -> (i32, u8) {
    let uiA = a.v;
    let sign = signF16UI(uiA);
    let exp = expF16UI(uiA);
    let frac = fracF16UI(uiA);

    if exp == 0x1F {
        return (
            if frac != 0 {
                i32_fromNaN
            } else if sign {
                i32_fromNegOverflow
            } else {
                i32_fromPosOverflow
            },
            softfloat_flag_invalid,
        );
    }

    let mut sig32 = frac as i32;
    if exp != 0 {
        sig32 |= 0x0400;
        let shiftDist = exp.wrapping_sub(0x19);
        if 0 <= shiftDist {
            sig32 <<= shiftDist;
            return (if sign { -sig32 } else { sig32 }, 0);
        }
        let shiftDist = exp.wrapping_sub(0x0D);
        if 0 < shiftDist {
            sig32 <<= shiftDist;
        }
    }

    return softfloat_roundToI32(sign, sig32 as u64, roundingMode, exact);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_to_i32() {
        struct softfloat_f16_to_i32_TestCase {
            a: u16,
            roundingMode: u8,
            exact: bool,
            result: i32,
            flags: u8,
        }

        let cases = [
            softfloat_f16_to_i32_TestCase {
                a: 0x3C00,
                roundingMode: 0,
                exact: false,
                result: 1,
                flags: 0,
            },
            softfloat_f16_to_i32_TestCase {
                a: 0x0000,
                roundingMode: 0,
                exact: false,
                result: 0,
                flags: 0,
            },
            softfloat_f16_to_i32_TestCase {
                a: 0xBC00,
                roundingMode: 0,
                exact: false,
                result: -1,
                flags: 0,
            },
            softfloat_f16_to_i32_TestCase {
                a: 0x4000,
                roundingMode: 0,
                exact: false,
                result: 2,
                flags: 0,
            },
            softfloat_f16_to_i32_TestCase {
                a: 0x5640,
                roundingMode: 0,
                exact: false,
                result: 100,
                flags: 0,
            },
            softfloat_f16_to_i32_TestCase {
                a: 0xD640,
                roundingMode: 0,
                exact: false,
                result: -100,
                flags: 0,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_to_i32(float16_t { v: c.a }, c.roundingMode, c.exact);
            assert_eq!((i, res, flags), (i, c.result, c.flags));
        }
    }
}
