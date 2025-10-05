use super::{float16_t, packToF16UI, softfloat_countLeadingZeros32, softfloat_roundPackToF16};

#[must_use]
pub const fn i32_to_f16(a: i32, roundingMode: u8, detectTininess: u8) -> (float16_t, u8) {
    let sign = a < 0;
    let absA = if sign {
        (a as u32).wrapping_neg()
    } else {
        a as u32
    };

    let mut shiftDist = (softfloat_countLeadingZeros32(absA) as i8).wrapping_sub(21);
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
        (absA >> -shiftDist) as u16 | ((absA << (shiftDist & 31)) != 0) as u16
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
    fn test_i32_to_f16() {
        struct softfloat_i32_to_f16_TestCase {
            a: i32,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_i32_to_f16_TestCase {
                a: 0,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i32_to_f16_TestCase {
                a: 1,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i32_to_f16_TestCase {
                a: -1,
                result: 0xBC00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i32_to_f16_TestCase {
                a: 2,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i32_to_f16_TestCase {
                a: 100,
                result: 0x5640,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i32_to_f16_TestCase {
                a: -100,
                result: 0xD640,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i32_to_f16_TestCase {
                a: 32768,
                result: 0x7800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_i32_to_f16_TestCase {
                a: -32768,
                result: 0xF800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = i32_to_f16(c.a, c.roundingMode, c.detectTininess);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
