// SPDX-License-Identifier: MIT OR Apache-2.0

use core::borrow::Borrow;

use super::{
    softfloat::{
        float16_t, float32_t, float64_t, i32_to_f16, i32_to_f32, i32_to_f64, i64_to_f16,
        i64_to_f32, i64_to_f64, init_detectTininess, ui32_to_f16, ui32_to_f32, ui32_to_f64,
        ui64_to_f16, ui64_to_f32, ui64_to_f64,
    },
    wrapper::{ExceptionFlags, Float, RoundingMode, TininessMode},
};

#[derive(Copy, Clone, Debug)]
pub struct FPU {
    pub flags: ExceptionFlags,
    detect_tininess: u8,
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn new(tininess: TininessMode) -> Self {
        Self {
            flags: ExceptionFlags::default(),
            detect_tininess: tininess.to_softfloat(),
        }
    }
}

impl Default for FPU {
    #[inline]
    fn default() -> Self {
        Self {
            flags: ExceptionFlags::default(),
            detect_tininess: init_detectTininess,
        }
    }
}

impl FPU {
    #[inline]
    const fn flagged_f64(&mut self, args: (float64_t, u8)) -> float64_t {
        self.flags.merge(args.1);
        args.0
    }

    #[inline]
    const fn flagged_f32(&mut self, args: (float32_t, u8)) -> float32_t {
        self.flags.merge(args.1);
        args.0
    }

    #[inline]
    fn flagged<X>(&mut self, args: (X, u8)) -> X {
        self.flags.merge(args.1);
        args.0
    }
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn to_i32<F, T>(&mut self, x: T, rnd: RoundingMode, exact: bool) -> i32
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(x.borrow().to_i32(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn to_i64<F, T>(&mut self, a: T, rnd: RoundingMode, exact: bool) -> i64
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_i64(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn to_u64<F, T>(&mut self, a: T, rnd: RoundingMode, exact: bool) -> u64
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_u64(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn to_u32<F, T>(&mut self, a: T, rnd: RoundingMode, exact: bool) -> u32
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_u32(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn to_f64<F, T>(&mut self, a: T, rnd: RoundingMode) -> float64_t
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_f64(rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn to_f16<F, T>(&mut self, a: T, rnd: RoundingMode) -> float16_t
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_f16(rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn to_f32<F, T>(&mut self, a: T, rnd: RoundingMode) -> float32_t
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().to_f32(rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn is_signaling_nan<F, T>(&mut self, a: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        a.borrow().is_signaling_nan()
    }

    #[inline]
    #[must_use]
    pub fn lt<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().lt(b.borrow()))
    }

    #[inline]
    #[must_use]
    pub fn le<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().le(b.borrow()))
    }

    #[inline]
    #[must_use]
    pub fn eq<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().eq(b.borrow()))
    }

    #[inline]
    #[must_use]
    pub fn eq_signaling<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().eq_signaling(b.borrow()))
    }

    #[inline]
    #[must_use]
    pub fn lt_quiet<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().lt_quiet(b))
    }

    #[inline]
    #[must_use]
    pub fn le_quiet<F, T>(&mut self, a: T, b: T) -> bool
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().le_quiet(b))
    }

    #[inline]
    #[must_use]
    pub fn add<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().add(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn mul_add<F, T>(&mut self, a: T, b: T, c: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().fused_mul_add(b, c, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn div<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().div(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn mul<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().mul(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn rem<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().rem(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn sub<F, T>(&mut self, a: T, b: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().sub(b, rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn round_to_int<F, T>(&mut self, a: T, rnd: RoundingMode, exact: bool) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().round_to_integral(rnd, exact))
    }

    #[inline]
    #[must_use]
    pub fn sqrt<F, T>(&mut self, a: T, rnd: RoundingMode) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        self.flagged(a.borrow().sqrt(rnd, self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn max<F, T>(&mut self, a: T, b: T) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        let fs1_val = a.borrow();
        let fs2_val = b.borrow();
        let fs2_nan = fs2_val.is_nan();
        let fs1_nan = fs1_val.is_nan();
        if fs1_nan && fs2_nan {
            F::from_bits(F::DEFAULT_NAN)
        } else {
            let is_lt = self.lt_quiet::<F, &F>(fs2_val, fs1_val);
            let is_eq = self.eq::<F, &F>(fs2_val, fs1_val);
            let greater = is_lt || (is_eq && (fs1_val.is_positive() || !fs2_val.is_positive()));
            if greater || fs2_nan {
                F::from_bits(fs1_val.to_bits())
            } else {
                F::from_bits(fs2_val.to_bits())
            }
        }
    }

    #[inline]
    #[must_use]
    pub fn min<F, T>(&mut self, a: T, b: T) -> F
    where
        F: Float,
        T: Borrow<F>,
    {
        let fs1_val = a.borrow();
        let fs2_val = b.borrow();
        let fs2_nan = fs2_val.is_nan();
        let fs1_nan = fs1_val.is_nan();
        if fs1_nan && fs2_nan {
            F::from_bits(F::DEFAULT_NAN)
        } else {
            let is_lt = self.lt_quiet::<F, &F>(fs1_val, fs2_val);
            let is_eq = self.eq::<F, &F>(fs1_val, fs2_val);
            let less = is_lt || (is_eq && !fs1_val.is_positive());
            if less || fs2_nan {
                F::from_bits(fs1_val.to_bits())
            } else {
                F::from_bits(fs2_val.to_bits())
            }
        }
    }
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn f16_from_i64(&mut self, a: i64, rnd: RoundingMode) -> float16_t {
        self.flagged(i64_to_f16(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f16_from_i32(&mut self, a: i32, rnd: RoundingMode) -> float16_t {
        self.flagged(i32_to_f16(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f16_from_u64(&mut self, a: u64, rnd: RoundingMode) -> float16_t {
        self.flagged(ui64_to_f16(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f16_from_u32(&mut self, a: u32, rnd: RoundingMode) -> float16_t {
        self.flagged(ui32_to_f16(a, rnd.to_softfloat(), self.detect_tininess))
    }
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn f32_from_i64(&mut self, a: i64, rnd: RoundingMode) -> float32_t {
        self.flagged_f32(i64_to_f32(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f32_from_i32(&mut self, a: i32, rnd: RoundingMode) -> float32_t {
        self.flagged_f32(i32_to_f32(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f32_from_u64(&mut self, a: u64, rnd: RoundingMode) -> float32_t {
        self.flagged_f32(ui64_to_f32(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f32_from_u32(&mut self, a: u32, rnd: RoundingMode) -> float32_t {
        self.flagged_f32(ui32_to_f32(a, rnd.to_softfloat(), self.detect_tininess))
    }
}

impl FPU {
    #[inline]
    #[must_use]
    pub fn f64_from_i64(&mut self, a: i64, rnd: RoundingMode) -> float64_t {
        self.flagged_f64(i64_to_f64(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f64_from_i32(&mut self, a: i32) -> float64_t {
        i32_to_f64(a)
    }

    #[inline]
    #[must_use]
    pub fn f64_from_u64(&mut self, a: u64, rnd: RoundingMode) -> float64_t {
        self.flagged_f64(ui64_to_f64(a, rnd.to_softfloat(), self.detect_tininess))
    }

    #[inline]
    #[must_use]
    pub fn f64_from_u32(&mut self, a: u32) -> float64_t {
        ui32_to_f64(a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper functions to create specific float values
    fn make_f32(v: f32) -> float32_t {
        float32_t::from_bits(v.to_bits())
    }

    fn make_f64(v: f64) -> float64_t {
        float64_t::from_bits(v.to_bits())
    }

    fn get_f32(v: float32_t) -> f32 {
        f32::from_bits(v.to_bits())
    }

    fn get_f64(v: float64_t) -> f64 {
        f64::from_bits(v.to_bits())
    }

    #[test]
    fn test_max_f32_normal_values() {
        let mut fpu = FPU::default();

        // Test regular positive values
        assert_eq!(get_f32(fpu.max(make_f32(1.0), make_f32(2.0))), 2.0);
        assert_eq!(get_f32(fpu.max(make_f32(2.0), make_f32(1.0))), 2.0);

        // Test regular negative values
        assert_eq!(get_f32(fpu.max(make_f32(-1.0), make_f32(-2.0))), -1.0);
        assert_eq!(get_f32(fpu.max(make_f32(-2.0), make_f32(-1.0))), -1.0);

        // Test mixed sign values
        assert_eq!(get_f32(fpu.max(make_f32(-1.0), make_f32(1.0))), 1.0);
        assert_eq!(get_f32(fpu.max(make_f32(1.0), make_f32(-1.0))), 1.0);
    }

    #[test]
    fn test_max_f32_special_values() {
        let mut fpu = FPU::default();

        // Test with infinities
        assert_eq!(
            get_f32(fpu.max(make_f32(f32::INFINITY), make_f32(1.0))),
            f32::INFINITY
        );
        assert_eq!(
            get_f32(fpu.max(make_f32(1.0), make_f32(f32::INFINITY))),
            f32::INFINITY
        );
        assert_eq!(
            get_f32(fpu.max(make_f32(f32::NEG_INFINITY), make_f32(1.0))),
            1.0
        );
        assert_eq!(
            get_f32(fpu.max(make_f32(1.0), make_f32(f32::NEG_INFINITY))),
            1.0
        );
        assert_eq!(
            get_f32(fpu.max(make_f32(f32::INFINITY), make_f32(f32::NEG_INFINITY))),
            f32::INFINITY
        );

        // Test with NaN
        let nan = f32::NAN;
        let result1 = get_f32(fpu.max(make_f32(nan), make_f32(1.0)));
        let result2 = get_f32(fpu.max(make_f32(1.0), make_f32(nan)));

        assert_eq!(result1, 1.0); // NaN + non-NaN should return non-NaN
        assert_eq!(result2, 1.0); // non-NaN + NaN should return non-NaN

        // Both NaN should return a canonical NaN
        let result_both_nan = fpu.max(make_f32(nan), make_f32(nan));
        assert!(get_f32(result_both_nan).is_nan());

        // Test with zero
        let plus_zero = 0.0f32;
        let minus_zero = -0.0f32;

        // Max should prefer +0 over -0
        let zero_result = get_f32(fpu.max(make_f32(plus_zero), make_f32(minus_zero)));
        assert!(zero_result == 0.0 && zero_result.is_sign_positive());

        let zero_result = get_f32(fpu.max(make_f32(minus_zero), make_f32(plus_zero)));
        assert!(zero_result == 0.0 && zero_result.is_sign_positive());
    }

    #[test]
    fn test_min_f32_normal_values() {
        let mut fpu = FPU::default();

        // Test regular positive values
        assert_eq!(get_f32(fpu.min(make_f32(1.0), make_f32(2.0))), 1.0);
        assert_eq!(get_f32(fpu.min(make_f32(2.0), make_f32(1.0))), 1.0);

        // Test regular negative values
        assert_eq!(get_f32(fpu.min(make_f32(-1.0), make_f32(-2.0))), -2.0);
        assert_eq!(get_f32(fpu.min(make_f32(-2.0), make_f32(-1.0))), -2.0);

        // Test mixed sign values
        assert_eq!(get_f32(fpu.min(make_f32(-1.0), make_f32(1.0))), -1.0);
        assert_eq!(get_f32(fpu.min(make_f32(1.0), make_f32(-1.0))), -1.0);
    }

    #[test]
    fn test_min_f32_special_values() {
        let mut fpu = FPU::default();

        // Test with infinities
        assert_eq!(
            get_f32(fpu.min(make_f32(f32::INFINITY), make_f32(1.0))),
            1.0
        );
        assert_eq!(
            get_f32(fpu.min(make_f32(1.0), make_f32(f32::INFINITY))),
            1.0
        );
        assert_eq!(
            get_f32(fpu.min(make_f32(f32::NEG_INFINITY), make_f32(1.0))),
            f32::NEG_INFINITY
        );
        assert_eq!(
            get_f32(fpu.min(make_f32(1.0), make_f32(f32::NEG_INFINITY))),
            f32::NEG_INFINITY
        );
        assert_eq!(
            get_f32(fpu.min(make_f32(f32::INFINITY), make_f32(f32::NEG_INFINITY))),
            f32::NEG_INFINITY
        );

        // Test with NaN
        let nan = f32::NAN;
        let result1 = get_f32(fpu.min(make_f32(nan), make_f32(1.0)));
        let result2 = get_f32(fpu.min(make_f32(1.0), make_f32(nan)));

        assert_eq!(result1, 1.0); // NaN + non-NaN should return non-NaN
        assert_eq!(result2, 1.0); // non-NaN + NaN should return non-NaN

        // Both NaN should return a canonical NaN
        let result_both_nan = fpu.min(make_f32(nan), make_f32(nan));
        assert!(get_f32(result_both_nan).is_nan());

        // Test with zero
        let plus_zero = 0.0f32;
        let minus_zero = -0.0f32;

        // Min should prefer -0 over +0
        let zero_result = get_f32(fpu.min(make_f32(plus_zero), make_f32(minus_zero)));
        assert!(zero_result == 0.0 && zero_result.is_sign_negative());

        let zero_result = get_f32(fpu.min(make_f32(minus_zero), make_f32(plus_zero)));
        assert!(zero_result == 0.0 && zero_result.is_sign_negative());

        let zero_result = get_f32(fpu.min(make_f32(plus_zero), make_f32(minus_zero)));
        assert!(zero_result == 0.0 && zero_result.is_sign_negative());
    }

    #[test]
    fn test_max_f64_normal_values() {
        let mut fpu = FPU::default();

        // Test regular positive values
        assert_eq!(get_f64(fpu.max(make_f64(1.0), make_f64(2.0))), 2.0);
        assert_eq!(get_f64(fpu.max(make_f64(2.0), make_f64(1.0))), 2.0);

        // Test regular negative values
        assert_eq!(get_f64(fpu.max(make_f64(-1.0), make_f64(-2.0))), -1.0);
        assert_eq!(get_f64(fpu.max(make_f64(-2.0), make_f64(-1.0))), -1.0);

        // Test mixed sign values
        assert_eq!(get_f64(fpu.max(make_f64(-1.0), make_f64(1.0))), 1.0);
        assert_eq!(get_f64(fpu.max(make_f64(1.0), make_f64(-1.0))), 1.0);
    }

    #[test]
    fn test_min_f64_special_values() {
        let mut fpu = FPU::default();

        // Test with infinities
        assert_eq!(
            get_f64(fpu.min(make_f64(f64::INFINITY), make_f64(1.0))),
            1.0
        );
        assert_eq!(
            get_f64(fpu.min(make_f64(1.0), make_f64(f64::INFINITY))),
            1.0
        );
        assert_eq!(
            get_f64(fpu.min(make_f64(f64::NEG_INFINITY), make_f64(1.0))),
            f64::NEG_INFINITY
        );
        assert_eq!(
            get_f64(fpu.min(make_f64(1.0), make_f64(f64::NEG_INFINITY))),
            f64::NEG_INFINITY
        );

        // Test with NaN
        let nan = f64::NAN;
        let result1 = get_f64(fpu.min(make_f64(nan), make_f64(1.0)));
        let result2 = get_f64(fpu.min(make_f64(1.0), make_f64(nan)));

        assert_eq!(result1, 1.0); // NaN + non-NaN should return non-NaN
        assert_eq!(result2, 1.0); // non-NaN + NaN should return non-NaN

        // Test with zero
        let plus_zero = 0.0f64;
        let minus_zero = -0.0f64;

        // Min should prefer -0 over +0
        let zero_result = get_f64(fpu.min(make_f64(plus_zero), make_f64(minus_zero)));
        assert!(zero_result == 0.0 && zero_result.is_sign_negative());
    }
}
