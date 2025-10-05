#[inline]
#[must_use]
pub const fn softfloat_countLeadingZeros16(a: u16) -> u8 {
    (if a != 0 { a.leading_zeros() } else { 16 }) as u8
}
