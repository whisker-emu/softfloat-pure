use super::{float16_t, signF16UI, softfloat_addMagsF16, softfloat_subMagsF16};

#[inline]
#[must_use]
pub const fn f16_add(
    a: float16_t,
    b: float16_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    if signF16UI(a.v ^ b.v) {
        softfloat_subMagsF16(a.v, b.v, roundingMode, detectTininess)
    } else {
        softfloat_addMagsF16(a.v, b.v, roundingMode, detectTininess)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_add() {
        struct softfloat_f16_add_TestCase {
            a: u16,
            b: u16,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f16_add_TestCase {
                a: 0x0000,
                b: 0x3C00,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_add_TestCase {
                a: 0x3C00,
                b: 0x3C00,
                result: 0x4000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_add_TestCase {
                a: 0xBC00,
                b: 0x3C00,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_add_TestCase {
                a: 0x0000,
                b: 0x0000,
                result: 0x0000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_add_TestCase {
                a: 0x3800,
                b: 0x3800,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_add_TestCase {
                a: 0xBC00,
                b: 0xBC00,
                result: 0xC000,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_add(
                float16_t { v: c.a },
                float16_t { v: c.b },
                c.roundingMode,
                c.detectTininess,
            );
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
