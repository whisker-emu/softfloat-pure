use super::{
    float16_t, packToF16UI, softfloat_countLeadingZeros64, softfloat_roundPackToF16,
    softfloat_shortShiftRightJam64,
};

#[must_use]
pub const fn ui64_to_f16(a: u64, roundingMode: u8, detectTininess: u8) -> (float16_t, u8) {
    let mut shiftDist = (softfloat_countLeadingZeros64(a) as i8).wrapping_sub(53);
    if shiftDist >= 0 {
        let uiZ = if a != 0 {
            packToF16UI(
                false,
                (0x18i8).wrapping_sub(shiftDist),
                (a << shiftDist) as u16,
            )
        } else {
            0
        };
        return (float16_t { v: uiZ }, 0);
    }

    shiftDist = shiftDist.wrapping_add(4);
    let sig = if shiftDist < 0 {
        softfloat_shortShiftRightJam64(a, (-shiftDist) as u8) as u16
    } else {
        (a << shiftDist) as u16
    };

    return softfloat_roundPackToF16(
        false,
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
    fn test_ui64_to_f16() {
        struct softfloat_ui64_to_f16_TestCase {
            a: u64,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_ui64_to_f16_TestCase {
                a: 0,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_ui64_to_f16_TestCase {
                a: 1,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_ui64_to_f16_TestCase {
                a: 2,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_ui64_to_f16_TestCase {
                a: 100,
                result: 0x5640,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_ui64_to_f16_TestCase {
                a: 32768,
                result: 0x7800,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = ui64_to_f16(c.a, c.roundingMode, c.detectTininess);
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
