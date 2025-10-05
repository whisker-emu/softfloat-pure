use super::{exp8_sig16, softfloat_countLeadingZeros16};

#[inline]
#[must_use]
pub const fn softfloat_normSubnormalF16Sig(sig: u16) -> exp8_sig16 {
    let shiftDist = softfloat_countLeadingZeros16(sig).wrapping_sub(5) as i8;
    exp8_sig16 {
        exp: (1 as i8).wrapping_sub(shiftDist),
        sig: sig << shiftDist,
    }
}
