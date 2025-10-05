use crate::softfloat::{commonNaN, softfloat_flag_invalid};

/*----------------------------------------------------------------------------
| Assuming 'uiA' has the bit pattern of a 16-bit floating-point NaN, converts
| this NaN to the common NaN form, and stores the resulting common NaN at the
| location pointed to by 'zPtr'.  If the NaN is a signaling NaN, the invalid
| exception is raised.
*----------------------------------------------------------------------------*/
//#define softfloat_f16UIToCommonNaN( uiA, zPtr ) if ( ! ((uiA) & 0x0200) ) softfloat_raiseFlags( softfloat_flag_invalid )

#[inline]
#[must_use]
pub const fn softfloat_f16UIToCommonNaN(uiA: u16) -> (commonNaN, u8) {
    if (uiA & 0x0200) == 0 {
        return (commonNaN::default(), softfloat_flag_invalid);
    }
    (commonNaN::default(), 0)
}
