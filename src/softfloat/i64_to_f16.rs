use super::{
    float16_t, packToF16UI, softfloat_countLeadingZeros64, softfloat_roundPackToF16,
    softfloat_shortShiftRightJam64,
};

#[must_use]
pub const fn i64_to_f16(a: i64, roundingMode: u8, detectTininess: u8) -> (float16_t, u8) {
    let sign = a < 0;
    let absA = if sign {
        (a as u64).wrapping_neg()
    } else {
        a as u64
    };

    let mut shiftDist = (softfloat_countLeadingZeros64(absA) as i8).wrapping_sub(53);
    if shiftDist >= 0 {
        let uiZ = if a != 0 {
            packToF16UI(
                sign,
                (0x18i8).wrapping_sub(shiftDist),
                (absA << shiftDist) as u16,
            )
        } else {
            0
        };
        return (float16_t { v: uiZ }, 0);
    }
    shiftDist = shiftDist.wrapping_add(4);
    let sig = if shiftDist < 0 {
        softfloat_shortShiftRightJam64(absA, (-shiftDist) as u8) as u16
    } else {
        (absA << shiftDist) as u16
    };
    return softfloat_roundPackToF16(
        sign,
        (0x1Ci8).wrapping_sub(shiftDist) as i16,
        sig,
        roundingMode,
        detectTininess,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i64_to_f16() {
        struct softfloat_i64_to_f16_TestCase {
            a: i64,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_i64_to_f16_TestCase {
                a: 0,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: 1,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: -1,
                result: 0xBC00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: 2,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: 100,
                result: 0x5640,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: -100,
                result: 0xD640,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: 32768,
                result: 0x7800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: -32768,
                result: 0xF800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: 0x7FFFFFFFFFFFFFFF,
                result: 0x7C00,
                flags: 5,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i64_to_f16_TestCase {
                a: -0x8000000000000000i64,
                result: 0xFC00,
                flags: 5,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = i64_to_f16(c.a, c.roundingMode, c.detectTininess);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
