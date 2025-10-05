use super::{
    expF16UI, float16_t, fracF16UI, packToF16UI, signF16UI, softfloat_flag_inexact,
    softfloat_propagateNaNF16UI, softfloat_round_max, softfloat_round_min, softfloat_round_minMag,
    softfloat_round_near_even, softfloat_round_near_maxMag, softfloat_round_odd,
};

#[must_use]
pub const fn f16_roundToInt(a: float16_t, roundingMode: u8, exact: bool) -> (float16_t, u8) {
    let uiA = a.v;
    let exp = expF16UI(uiA);
    let mut flags: u8 = 0;
    if exp <= 0xE {
        if (uiA << 1) == 0 {
            return (a, flags);
        }
        if exact {
            flags |= softfloat_flag_inexact;
        }
        let mut uiZ = uiA & packToF16UI(true, 0, 0);
        match roundingMode {
            softfloat_round_near_even => {
                if fracF16UI(uiA) != 0 && exp == 0xE {
                    uiZ |= packToF16UI(false, 0xF, 0);
                }
            }
            softfloat_round_near_maxMag => {
                if exp == 0xE {
                    uiZ |= packToF16UI(false, 0xF, 0);
                }
            }
            softfloat_round_min => {
                if uiZ != 0 {
                    uiZ = packToF16UI(true, 0xF, 0);
                }
            }
            softfloat_round_max => {
                if uiZ == 0 {
                    uiZ = packToF16UI(false, 0xF, 0);
                }
            }
            softfloat_round_odd => {
                uiZ |= packToF16UI(false, 0xF, 0);
            }
            _ => {}
        }
        return (float16_t { v: uiZ }, flags);
    }

    if 0x19 <= exp {
        if (exp == 0x1F) && (fracF16UI(uiA) != 0) {
            let (uiZ, new_flags) = softfloat_propagateNaNF16UI(uiA, 0);
            return (float16_t { v: uiZ }, flags | new_flags);
        }
        return (a, flags);
    }

    let mut uiZ = uiA;
    let lastBitMask = (1 as u16) << (0x19_i8).wrapping_sub(exp);
    let roundBitsMask = lastBitMask.wrapping_sub(1);

    if roundingMode == softfloat_round_near_maxMag {
        uiZ = uiZ.wrapping_add(lastBitMask >> 1);
    } else if roundingMode == softfloat_round_near_even {
        uiZ = uiZ.wrapping_add(lastBitMask >> 1);
        if (uiZ & roundBitsMask) == 0 {
            uiZ &= !lastBitMask;
        }
    } else if roundingMode
        == (if signF16UI(uiZ) {
            softfloat_round_min
        } else {
            softfloat_round_max
        })
    {
        uiZ = uiZ.wrapping_add(roundBitsMask);
    }

    uiZ &= !roundBitsMask;

    if uiZ != uiA {
        if roundingMode == softfloat_round_odd {
            uiZ |= lastBitMask;
        }
        if exact {
            flags |= softfloat_flag_inexact;
        }
    }

    return (float16_t { v: uiZ }, flags);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_roundToInt() {
        struct softfloat_f16_roundToInt_TestCase {
            a: u16,
            roundingMode: u8,
            exact: bool,
            result: u16,
            flags: u8,
        }

        let cases = [
            softfloat_f16_roundToInt_TestCase {
                a: 0x3E00,
                roundingMode: softfloat_round_near_even,
                exact: true,
                result: 0x4000,
                flags: softfloat_flag_inexact,
            },
            softfloat_f16_roundToInt_TestCase {
                a: 0x4100,
                roundingMode: softfloat_round_near_even,
                exact: true,
                result: 0x4000,
                flags: softfloat_flag_inexact,
            },
            softfloat_f16_roundToInt_TestCase {
                a: 0x4300,
                roundingMode: softfloat_round_near_even,
                exact: true,
                result: 0x4400,
                flags: softfloat_flag_inexact,
            },
            softfloat_f16_roundToInt_TestCase {
                a: 0x3C00,
                roundingMode: softfloat_round_near_even,
                exact: true,
                result: 0x3C00,
                flags: 0,
            },
            softfloat_f16_roundToInt_TestCase {
                a: 0x3800,
                roundingMode: softfloat_round_near_even,
                exact: true,
                result: 0x0000,
                flags: softfloat_flag_inexact,
            },
            softfloat_f16_roundToInt_TestCase {
                a: 0x3E00,
                roundingMode: softfloat_round_minMag,
                exact: true,
                result: 0x3C00,
                flags: softfloat_flag_inexact,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_roundToInt(float16_t { v: c.a }, c.roundingMode, c.exact);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
