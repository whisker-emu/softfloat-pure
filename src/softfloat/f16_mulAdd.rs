use super::{float16_t, softfloat_mulAddF16};

#[inline]
#[must_use]
pub const fn f16_mulAdd(
    a: float16_t,
    b: float16_t,
    c: float16_t,
    roundingMode: u8,
    detectTininess: u8,
) -> (float16_t, u8) {
    return softfloat_mulAddF16(a.v, b.v, c.v, 0, roundingMode, detectTininess);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f16_mulAdd() {
        struct softfloat_f16_mulAdd_TestCase {
            a: u16,
            b: u16,
            c: u16,
            result: u16,
            flags: u8,
            roundingMode: u8,
            detectTininess: u8,
        }

        let cases = [
            softfloat_f16_mulAdd_TestCase {
                a: 0x3C00,
                b: 0x3C00,
                c: 0x0000,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_mulAdd_TestCase {
                a: 0x4000,
                b: 0x4200,
                c: 0x3C00,
                result: 0x4700,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_mulAdd_TestCase {
                a: 0x3800,
                b: 0x3800,
                c: 0x3800,
                result: 0x3A00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
            softfloat_f16_mulAdd_TestCase {
                a: 0x3C00,
                b: 0x0000,
                c: 0x3C00,
                result: 0x3C00,
                flags: 0,
                roundingMode: 0,
                detectTininess: 1,
            },
        ];

        for (i, c) in cases.iter().enumerate() {
            let (res, flags) = f16_mulAdd(
                float16_t { v: c.a },
                float16_t { v: c.b },
                float16_t { v: c.c },
                c.roundingMode,
                c.detectTininess,
            );
            assert_eq!((i, res.v, flags), (i, c.result, c.flags));
        }
    }
}
