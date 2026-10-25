// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{Float, RoundingMode};
use crate::softfloat::{
    defaultNaNF16UI, f16_add, f16_div, f16_eq, f16_eq_signaling, f16_isSignalingNaN, f16_le,
    f16_le_quiet, f16_lt, f16_lt_quiet, f16_mul, f16_mulAdd, f16_rem, f16_roundToInt, f16_sqrt,
    f16_sub, f16_to_f32, f16_to_f64, f16_to_i32, f16_to_i64, f16_to_ui32, f16_to_ui64, float16_t,
    float32_t, float64_t, i32_to_f16, i64_to_f16, ui32_to_f16, ui64_to_f16,
};
use core::borrow::Borrow;

impl float16_t {
    /// Converts primitive `f32` to `float16_t`.
    #[inline]
    #[must_use]
    pub fn from_f32(v: f32, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        float32_t::from_bits(v.to_bits()).to_f16(rnd, detect_tininess)
    }

    /// Converts primitive `f64` to `float16_t`.
    #[inline]
    #[must_use]
    pub fn from_f64(v: f64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        float64_t::from_bits(v.to_bits()).to_f16(rnd, detect_tininess)
    }
}

impl Float for float16_t {
    type Payload = u16;

    const EXPONENT_BIT: Self::Payload = 0x1F;
    const FRACTION_BIT: Self::Payload = 0x03FF;
    const DEFAULT_NAN: Self::Payload = defaultNaNF16UI;
    const SIGN_POS: usize = 15;
    const EXPONENT_POS: usize = 10;

    #[inline]
    fn set_payload(&mut self, x: Self::Payload) {
        self.v = x;
    }

    #[inline]
    fn from_bits(v: Self::Payload) -> Self {
        Self { v }
    }

    #[inline]
    fn to_bits(&self) -> Self::Payload {
        self.v
    }

    #[inline]
    fn add<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f16_add(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn sub<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f16_sub(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn mul<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f16_mul(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn fused_mul_add<T: Borrow<Self>>(
        &self,
        x: T,
        y: T,
        rnd: RoundingMode,
        detect_tininess: u8,
    ) -> (Self, u8) {
        f16_mulAdd(
            *self,
            *x.borrow(),
            *y.borrow(),
            rnd.to_softfloat(),
            detect_tininess,
        )
    }

    #[inline]
    fn div<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f16_div(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn rem<T: Borrow<Self>>(&self, x: T, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f16_rem(*self, *x.borrow(), rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn sqrt(&self, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        f16_sqrt(*self, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn eq<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f16_eq(*self, *x.borrow())
    }

    #[inline]
    fn lt<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f16_lt(*self, *x.borrow())
    }

    #[inline]
    fn le<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f16_le(*self, *x.borrow())
    }

    #[inline]
    fn lt_quiet<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f16_lt_quiet(*self, *x.borrow())
    }

    #[inline]
    fn le_quiet<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f16_le_quiet(*self, *x.borrow())
    }

    #[inline]
    fn eq_signaling<T: Borrow<Self>>(&self, x: T) -> (bool, u8) {
        f16_eq_signaling(*self, *x.borrow())
    }

    #[inline]
    fn is_signaling_nan(&self) -> bool {
        f16_isSignalingNaN(*self)
    }

    #[inline]
    fn from_u32(x: u32, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        ui32_to_f16(x, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn from_u64(x: u64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        ui64_to_f16(x, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn from_i32(x: i32, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        i32_to_f16(x, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn from_i64(x: i64, rnd: RoundingMode, detect_tininess: u8) -> (Self, u8) {
        i64_to_f16(x, rnd.to_softfloat(), detect_tininess)
    }

    #[inline]
    fn to_u32(&self, rnd: RoundingMode, exact: bool) -> (u32, u8) {
        f16_to_ui32(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn to_u64(&self, rnd: RoundingMode, exact: bool) -> (u64, u8) {
        f16_to_ui64(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn to_i32(&self, rnd: RoundingMode, exact: bool) -> (i32, u8) {
        f16_to_i32(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn to_i64(&self, rnd: RoundingMode, exact: bool) -> (i64, u8) {
        f16_to_i64(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn round_to_integral(&self, rnd: RoundingMode, exact: bool) -> (Self, u8) {
        f16_roundToInt(*self, rnd.to_softfloat(), exact)
    }

    #[inline]
    fn to_f16(&self, _rnd: RoundingMode, _detect_tininess: u8) -> (float16_t, u8) {
        (*self, 0)
    }

    #[inline]
    fn to_f32(&self, _rnd: RoundingMode, _detect_tininess: u8) -> (float32_t, u8) {
        f16_to_f32(*self)
    }

    #[inline]
    fn to_f64(&self, _rnd: RoundingMode, _detect_tininess: u8) -> (float64_t, u8) {
        f16_to_f64(*self)
    }
}
