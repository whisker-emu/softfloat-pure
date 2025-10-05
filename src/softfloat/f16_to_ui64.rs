use crate::softfloat::softfloat_roundToUI64;

use super::{
    expF16UI, float16_t, fracF16UI, signF16UI, softfloat_flag_invalid, softfloat_roundToUI32,
    ui64_fromNaN, ui64_fromNegOverflow, ui64_fromPosOverflow,
};

#[must_use]
pub const fn f16_to_ui64(a: float16_t, roundingMode: u8, exact: bool) -> (u64, u8) {
    let uiA = a.v;
    let sign = signF16UI(uiA);
    let exp = expF16UI(uiA);
    let frac = fracF16UI(uiA);

    if exp == 0x1F {
        return (
            if frac != 0 {
                ui64_fromNaN
            } else if sign {
                ui64_fromNegOverflow
            } else {
                ui64_fromPosOverflow
            },
            softfloat_flag_invalid,
        );
    }

    let mut sig32 = frac as u32;
    if exp != 0 {
        sig32 |= 0x0400;
        let shiftDist = exp.wrapping_sub(0x19);
        if (0 <= shiftDist) && !sign {
            return ((sig32.wrapping_shl(shiftDist as u32)) as u64, 0);
        }
        let shiftDist = exp.wrapping_sub(0x0D);
        if 0 < shiftDist {
            sig32 = sig32.wrapping_shl(shiftDist as u32);
        }
    }

    return softfloat_roundToUI64(
        sign,
        (sig32.wrapping_shr(12)) as u64,
        (sig32.wrapping_shr(52)) as u64,
        roundingMode,
        exact,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_to_ui64() {
        struct softfloat_f16_to_ui64_TestCase {
            a: u16,
            roundingMode: u8,
            exact: bool,
            result: u64,
            flags: u8,
        }

        let cases = [
            softfloat_f16_to_ui64_TestCase {
                a: 0x3C00,
                roundingMode: 0,
                exact: false,
                result: 1,
                flags: 0,
            },
            softfloat_f16_to_ui64_TestCase {
                a: 0x0000,
                roundingMode: 0,
                exact: false,
                result: 0,
                flags: 0,
            },
            softfloat_f16_to_ui64_TestCase {
                a: 0x4000,
                roundingMode: 0,
                exact: false,
                result: 2,
                flags: 0,
            },
            softfloat_f16_to_ui64_TestCase {
                a: 0x5640,
                roundingMode: 0,
                exact: false,
                result: 100,
                flags: 0,
            },
            softfloat_f16_to_ui64_TestCase {
                a: 0x7800,
                roundingMode: 0,
                exact: false,
                result: 32768,
                flags: 0,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_to_ui64(float16_t { v: c.a }, c.roundingMode, c.exact);
            assert_eq!((i, res, flags), (i, c.result, c.flags));
        }
    }
}
