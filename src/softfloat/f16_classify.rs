use super::{expF16UI, float16_t, fracF16UI, isNaNF16UI, signF16UI, softfloat_isSigNaNF16UI};

#[must_use]
pub const fn f16_classify(a: float16_t) -> u16 {
    let infOrNaN = expF16UI(a.v) == 0x1F;
    let subnormalOrZero = expF16UI(a.v) == 0;
    let sign = signF16UI(a.v);
    let fracZero = fracF16UI(a.v) == 0;
    let isNaN = isNaNF16UI(a.v);
    let isSNaN = softfloat_isSigNaNF16UI(a.v);

    return ((sign && infOrNaN && fracZero) as u16)
        | (((sign && !infOrNaN && !subnormalOrZero) as u16) << 1)
        | (((sign && subnormalOrZero && !fracZero) as u16) << 2)
        | (((sign && subnormalOrZero && fracZero) as u16) << 3)
        | (((!sign && infOrNaN && fracZero) as u16) << 7)
        | (((!sign && !infOrNaN && !subnormalOrZero) as u16) << 6)
        | (((!sign && subnormalOrZero && !fracZero) as u16) << 5)
        | (((!sign && subnormalOrZero && fracZero) as u16) << 4)
        | (((isNaN && isSNaN) as u16) << 8)
        | (((isNaN && !isSNaN) as u16) << 9);
}
