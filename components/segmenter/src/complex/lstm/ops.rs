// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/anySE ).

#![allow(dead_code)]
use zerovec::ZeroSlice;

macro_rules! f32c {
    ($ule:expr) => {
        f32::from_unaligned($ule)
    };
}

pub(crate) fn dot_1(xs: &[f32], ys: &ZeroSlice<f32>) -> f32 {
    #[cfg(all(target_feature = "avx", target_feature = "fma"))]
    {
        unsafe { avx::dot_1_avx_fma(xs, ys) }
    }
    #[cfg(all(
        target_arch = "aarch64",
        target_endian = "little",
        target_feature = "neon"
    ))]
    {
        unsafe { neon::dot_1_neon(xs, ys) }
    }
    #[cfg(all(
        not(all(target_feature = "avx", target_feature = "fma")),
        not(all(
            target_arch = "aarch64",
            target_endian = "little",
            target_feature = "neon"
        )),
        not(feature = "std")
    ))]
    {
        unrolled::unrolled_dot_1(xs, ys)
    }
    #[cfg(all(
        not(all(target_feature = "avx", target_feature = "fma")),
        not(all(
            target_arch = "aarch64",
            target_endian = "little",
            target_feature = "neon"
        )),
        feature = "std"
    ))]
    {
        // runtime dispatch
        unsafe { runtime::DOT_1_PTR(xs, ys) }
    }
}

pub(crate) fn dot_2(xs: &ZeroSlice<f32>, ys: &ZeroSlice<f32>) -> f32 {
    #[cfg(all(target_feature = "avx", target_feature = "fma"))]
    {
        unsafe { avx::dot_2_avx_fma(xs, ys) }
    }
    #[cfg(all(
        target_arch = "aarch64",
        target_endian = "little",
        target_feature = "neon"
    ))]
    {
        neon::dot_2_neon(xs, ys)
    }
    #[cfg(all(
        not(all(target_feature = "avx", target_feature = "fma")),
        not(all(
            target_arch = "aarch64",
            target_endian = "little",
            target_feature = "neon"
        )),
        not(feature = "std")
    ))]
    {
        unrolled::unrolled_dot_2(xs, ys)
    }
    #[cfg(all(
        not(all(target_feature = "avx", target_feature = "fma")),
        not(all(
            target_arch = "aarch64",
            target_endian = "little",
            target_feature = "neon"
        )),
        feature = "std"
    ))]
    {
        // runtime dispatch
        unsafe { runtime::DOT_2_PTR(xs, ys) }
    }
}

#[cfg(all(
    not(all(target_feature = "avx", target_feature = "fma")),
    not(all(
        target_arch = "aarch64",
        target_endian = "little",
        target_feature = "neon"
    )),
    feature = "std"
))]
mod runtime {
    use once_cell::sync::Lazy;
    use zerovec::ZeroSlice;

    type Dot1Fn = unsafe fn(&[f32], &ZeroSlice<f32>) -> f32;
    type Dot2Fn = unsafe fn(&ZeroSlice<f32>, &ZeroSlice<f32>) -> f32;

    pub(crate) static DOT_1_PTR: Lazy<Dot1Fn> = Lazy::new(initialize_dot1);
    pub(crate) static DOT_2_PTR: Lazy<Dot2Fn> = Lazy::new(initialize_dot2);

    fn initialize_dot1() -> Dot1Fn {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if std::arch::is_x86_feature_detected!("avx")
                && std::arch::is_x86_feature_detected!("fma")
            {
                return super::avx::dot_1_avx_fma;
            }
        }
        #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
        {
            if std::arch::is_aarch64_feature_detected!("neon") {
                return super::neon::dot_1_neon;
            }
        }
        super::unrolled::unrolled_dot_1
    }

    fn initialize_dot2() -> Dot2Fn {
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        {
            if std::arch::is_x86_feature_detected!("avx")
                && std::arch::is_x86_feature_detected!("fma")
            {
                return super::avx::dot_2_avx_fma;
            }
        }
        #[cfg(all(target_arch = "aarch64", target_endian = "little"))]
        {
            if std::arch::is_aarch64_feature_detected!("neon") {
                return super::neon::dot_2_neon;
            }
        }
        super::unrolled::unrolled_dot_2
    }
}

#[cfg(all(
    not(all(target_feature = "avx", target_feature = "fma")),
    not(all(
        target_arch = "aarch64",
        target_endian = "little",
        target_feature = "neon"
    )),
))]
mod unrolled {
    use zerovec::ule::AsULE;
    use zerovec::ZeroSlice;

    /// Compute the dot product of an aligned and an unaligned f32 slice.
    ///
    /// `xs` and `ys` must be the same length
    ///
    /// (Based on ndarray 0.15.6)
    pub(crate) fn unrolled_dot_1(xs: &[f32], ys: &ZeroSlice<f32>) -> f32 {
        debug_assert_eq!(xs.len(), ys.len());
        // eightfold unrolled so that floating point can be vectorized
        // (even with strict floating point accuracy semantics)
        let mut p = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let xit = xs.chunks_exact(8);
        let yit = ys.as_ule_slice().chunks_exact(8);
        let sum = xit
            .remainder()
            .iter()
            .zip(yit.remainder().iter())
            .map(|(x, y)| x * f32c!(*y))
            .sum::<f32>();
        for (xx, yy) in xit.zip(yit) {
            // TODO: Use array_chunks once stable to avoid the unwrap.
            // <https://github.com/rust-lang/rust/issues/74985>
            #[allow(clippy::unwrap_used)]
            let [x0, x1, x2, x3, x4, x5, x6, x7] = *<&[f32; 8]>::try_from(xx).unwrap();
            #[allow(clippy::unwrap_used)]
            let [y0, y1, y2, y3, y4, y5, y6, y7] =
                *<&[<f32 as AsULE>::ULE; 8]>::try_from(yy).unwrap();
            p.0 += x0 * f32c!(y0);
            p.1 += x1 * f32c!(y1);
            p.2 += x2 * f32c!(y2);
            p.3 += x3 * f32c!(y3);
            p.4 += x4 * f32c!(y4);
            p.5 += x5 * f32c!(y5);
            p.6 += x6 * f32c!(y6);
            p.7 += x7 * f32c!(y7);
        }
        sum + (p.0 + p.4) + (p.1 + p.5) + (p.2 + p.6) + (p.3 + p.7)
    }

    /// Compute the dot product of two unaligned f32 slices.
    ///
    /// `xs` and `ys` must be the same length
    ///
    /// (Based on ndarray 0.15.6)
    pub(crate) fn unrolled_dot_2(xs: &ZeroSlice<f32>, ys: &ZeroSlice<f32>) -> f32 {
        debug_assert_eq!(xs.len(), ys.len());
        // eightfold unrolled so that floating point can be vectorized
        // (even with strict floating point accuracy semantics)
        let mut p = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let xit = xs.as_ule_slice().chunks_exact(8);
        let yit = ys.as_ule_slice().chunks_exact(8);
        let sum = xit
            .remainder()
            .iter()
            .zip(yit.remainder().iter())
            .map(|(x, y)| f32c!(*x) * f32c!(*y))
            .sum::<f32>();
        for (xx, yy) in xit.zip(yit) {
            // TODO: Use array_chunks once stable to avoid the unwrap.
            // <https://github.com/rust-lang/rust/issues/74985>
            #[allow(clippy::unwrap_used)]
            let [x0, x1, x2, x3, x4, x5, x6, x7] =
                *<&[<f32 as AsULE>::ULE; 8]>::try_from(xx).unwrap();
            #[allow(clippy::unwrap_used)]
            let [y0, y1, y2, y3, y4, y5, y6, y7] =
                *<&[<f32 as AsULE>::ULE; 8]>::try_from(yy).unwrap();
            p.0 += f32c!(x0) * f32c!(y0);
            p.1 += f32c!(x1) * f32c!(y1);
            p.2 += f32c!(x2) * f32c!(y2);
            p.3 += f32c!(x3) * f32c!(y3);
            p.4 += f32c!(x4) * f32c!(y4);
            p.5 += f32c!(x5) * f32c!(y5);
            p.6 += f32c!(x6) * f32c!(y6);
            p.7 += f32c!(x7) * f32c!(y7);
        }
        sum + (p.0 + p.4) + (p.1 + p.5) + (p.2 + p.6) + (p.3 + p.7)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod avx {
    use zerovec::ule::AsULE;
    use zerovec::ZeroSlice;

    #[target_feature(enable = "avx,fma")]
    pub(crate) unsafe fn dot_1_avx_fma(xs: &[f32], ys: &ZeroSlice<f32>) -> f32 {
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        debug_assert_eq!(xs.len(), ys.len());

        let xc = xs.chunks_exact(8);
        let yc = ys.as_ule_slice().chunks_exact(8);

        let remainder = xc
            .remainder()
            .iter()
            .zip(yc.remainder().iter())
            .map(|(x, y)| x * f32c!(*y))
            .sum::<f32>();

        // TODO: Use array_chunks once stable to avoid the unwrap.
        // <https://github.com/rust-lang/rust/issues/74985>
        #[allow(clippy::unwrap_used)]
        let xc = xc.map(|xx| *<&[_; 8]>::try_from(xx).unwrap());
        #[allow(clippy::unwrap_used)]
        let yc = yc.map(|yy| *<&[_; 8]>::try_from(yy).unwrap());
        use core::arch::x86_64::*;

        // SAFETY: No safety requirement
        let mut sum = unsafe { _mm256_setzero_ps() };

        for (x, y) in xc.zip(yc) {
            // We should be able to use _mm256_load_ps here, as x is f32-aligned, and f32-aligment
            // is the safety requirement of that function. However, it segfaults.
            let xv = unsafe { _mm256_loadu_ps(x.as_ptr()) };
            // SAFETY: _mm256_loadu_ps does not require its argument to be aligned
            let yv = unsafe { _mm256_loadu_ps(y.as_ptr() as *const f32) };
            // SAFETY: No safety requirement
            sum = unsafe { _mm256_fmadd_ps(xv, yv, sum) };
        }

        // Using hacks in
        // https://stackoverflow.com/questions/6996764/fastest-way-to-do-horizontal-sse-vector-sum-or-other-reduction
        // SAFETY: No safety requirement
        let mut lo = unsafe { _mm256_castps256_ps128(sum) };
        // SAFETY: No safety requirement
        let hi = unsafe { _mm256_extractf128_ps(sum, 1) };
        // SAFETY: No safety requirement
        lo = unsafe { _mm_add_ps(lo, hi) };

        // SAFETY: No safety requirement
        let mut shuf = unsafe { _mm_movehdup_ps(lo) };
        // SAFETY: No safety requirement
        let mut sums = unsafe { _mm_add_ps(lo, shuf) };
        // SAFETY: No safety requirement
        shuf = unsafe { _mm_movehl_ps(shuf, sums) };
        // SAFETY: No safety requirement
        sums = unsafe { _mm_add_ss(sums, shuf) };
        // SAFETY: No safety requirement
        unsafe { _mm_cvtss_f32(sums) + remainder }
    }

    #[target_feature(enable = "avx,fma")]
    pub(crate) unsafe fn dot_2_avx_fma(xs: &ZeroSlice<f32>, ys: &ZeroSlice<f32>) -> f32 {
        debug_assert_eq!(xs.len(), ys.len());
        #[cfg(target_arch = "x86")]
        use core::arch::x86::*;
        #[cfg(target_arch = "x86_64")]
        use core::arch::x86_64::*;

        let xc = xs.as_ule_slice().chunks_exact(8);
        let yc = ys.as_ule_slice().chunks_exact(8);

        let remainder = xc
            .remainder()
            .iter()
            .zip(yc.remainder().iter())
            .map(|(x, y)| f32c!(*x) * f32c!(*y))
            .sum::<f32>();

        // TODO: Use array_chunks once stable to avoid the unwrap.
        // <https://github.com/rust-lang/rust/issues/74985>
        #[allow(clippy::unwrap_used)]
        let xc = xc.map(|xx| *<&[_; 8]>::try_from(xx).unwrap());
        #[allow(clippy::unwrap_used)]
        let yc = yc.map(|yy| *<&[_; 8]>::try_from(yy).unwrap());

        // SAFETY: No safety requirement
        let mut sum = unsafe { _mm256_setzero_ps() };

        for (x, y) in xc.zip(yc) {
            // SAFETY: _mm256_loadu_ps does not require its argument to be aligned
            let xv = unsafe { _mm256_loadu_ps(x.as_ptr() as *const f32) };
            // SAFETY: _mm256_loadu_ps does not require its argument to be aligned
            let yv = unsafe { _mm256_loadu_ps(y.as_ptr() as *const f32) };
            // SAFETY: No safety requirement
            sum = unsafe { _mm256_fmadd_ps(xv, yv, sum) };
        }

        // Using hacks in
        // https://stackoverflow.com/questions/6996764/fastest-way-to-do-horizontal-sse-vector-sum-or-other-reduction
        // SAFETY: No safety requirement
        let mut lo = unsafe { _mm256_castps256_ps128(sum) };
        // SAFETY: No safety requirement
        let hi = unsafe { _mm256_extractf128_ps(sum, 1) };
        // SAFETY: No safety requirement
        lo = unsafe { _mm_add_ps(lo, hi) };

        // SAFETY: No safety requirement
        let mut shuf = unsafe { _mm_movehdup_ps(lo) };
        // SAFETY: No safety requirement
        let mut sums = unsafe { _mm_add_ps(lo, shuf) };
        // SAFETY: No safety requirement
        shuf = unsafe { _mm_movehl_ps(shuf, sums) };
        // SAFETY: No safety requirement
        sums = unsafe { _mm_add_ss(sums, shuf) };
        // SAFETY: No safety requirement
        unsafe { _mm_cvtss_f32(sums) + remainder }
    }
}

#[cfg(target_arch = "aarch64")]
#[cfg(target_endian = "little")]
mod neon {
    use zerovec::ule::AsULE;

    #[target_feature(enable = "neon")]
    pub(crate) unsafe fn dot_1_neon(xs: &[f32], ys: &ZeroSlice<f32>) -> f32 {
        use core::arch::aarch64::*;

        debug_assert_eq!(xs.len(), ys.len());

        let xc = xs.chunks_exact(8);
        let yc = ys.as_ule_slice().chunks_exact(8);

        let remainder = xc
            .remainder()
            .iter()
            .zip(yc.remainder().iter())
            .map(|(x, y)| x * f32c!(*y))
            .sum::<f32>();

        // TODO: Use array_chunks once stable to avoid the unwrap.
        // <https://github.com/rust-lang/rust/issues/74985>
        #[allow(clippy::unwrap_used)]
        let xc = xc.map(|xx| *<&[_; 8]>::try_from(xx).unwrap());
        #[allow(clippy::unwrap_used)]
        let yc = yc.map(|yy| *<&[_; 8]>::try_from(yy).unwrap());

        // https://developer.arm.com/documentation/102197/0100/Calculating-dot-products-using-Neon-Intrinsics
        let mut sum0 = unsafe { vdupq_n_f32(0.0) };
        let mut sum1 = unsafe { vdupq_n_f32(0.0) };

        for (x, y) in xc.zip(yc) {
            let xv0 = unsafe { vld1q_f32(x.as_ptr()) };
            let yv0 = unsafe { vld1q_f32(y.as_ptr() as *const f32) };

            sum0 = unsafe { vfmaq_f32(sum0, xv0, yv0) };

            let xv1 = unsafe { vld1q_f32(x[4..].as_ptr()) };
            let yv1 = unsafe { vld1q_f32(y[4..].as_ptr() as *const f32) };

            sum1 = unsafe { vfmaq_f32(sum1, xv1, yv1) };
        }
        unsafe { vaddvq_f32(sum0) + vaddvq_f32(sum1) + remainder }
    }

    #[target_feature(enable = "neon")]
    pub(crate) unsafe fn dot_2_neon(xs: &ZeroSlice<f32>, ys: &ZeroSlice<f32>) -> f32 {
        use core::arch::aarch64::*;

        debug_assert_eq!(xs.len(), ys.len());

        let xc = xs.as_ule_slice().chunks_exact(8);
        let yc = ys.as_ule_slice().chunks_exact(8);

        let remainder = xc
            .remainder()
            .iter()
            .zip(yc.remainder().iter())
            .map(|(x, y)| f32c!(*x) * f32c!(*y))
            .sum::<f32>();

        // TODO: Use array_chunks once stable to avoid the unwrap.
        // <https://github.com/rust-lang/rust/issues/74985>
        #[allow(clippy::unwrap_used)]
        let xc = xc.map(|xx| *<&[_; 8]>::try_from(xx).unwrap());
        #[allow(clippy::unwrap_used)]
        let yc = yc.map(|yy| *<&[_; 8]>::try_from(yy).unwrap());

        // https://developer.arm.com/documentation/102197/0100/Calculating-dot-products-using-Neon-Intrinsics
        let mut sum0 = unsafe { vdupq_n_f32(0.0) };
        let mut sum1 = unsafe { vdupq_n_f32(0.0) };

        for (x, y) in xc.zip(yc) {
            let xv0 = unsafe { vld1q_f32(x.as_ptr() as *const f32) };
            let yv0 = unsafe { vld1q_f32(y.as_ptr() as *const f32) };

            sum0 = unsafe { vfmaq_f32(sum0, xv0, yv0) };

            let xv1 = unsafe { vld1q_f32(x[4..].as_ptr() as *const f32) };
            let yv1 = unsafe { vld1q_f32(y[4..].as_ptr() as *const f32) };

            sum1 = unsafe { vfmaq_f32(sum1, xv1, yv1) };
        }
        unsafe { vaddvq_f32(sum0) + vaddvq_f32(sum1) + remainder }
    }
}
