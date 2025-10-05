use crate::softfloat::{commonNaN, defaultNaNF16UI};

/*----------------------------------------------------------------------------
| Converts the common NaN pointed to by 'aPtr' into a 16-bit floating-point
| NaN, and returns the bit pattern of this value as an unsigned integer.
*----------------------------------------------------------------------------*/
//#define softfloat_commonNaNToF16UI( aPtr ) ((uint_fast16_t) defaultNaNF16UI)
#[inline]
#[must_use]
pub const fn softfloat_commonNaNToF16UI(_aPtr: commonNaN) -> u16 {
    defaultNaNF16UI
}
