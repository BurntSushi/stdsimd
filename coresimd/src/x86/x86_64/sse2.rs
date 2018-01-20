//! `x86_64`'s Streaming SIMD Extensions 2 (SSE2)

use x86::*;
use simd_llvm::*;

#[cfg(test)]
use stdsimd_test::assert_instr;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.sse2.cvtsd2si64"]
    fn cvtsd2si64(a: __m128d) -> i64;
    #[link_name = "llvm.x86.sse2.cvttsd2si64"]
    fn cvttsd2si64(a: __m128d) -> i64;
}

/// Convert the lower double-precision (64-bit) floating-point element in a to
/// a 64-bit integer.
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsd2si))]
pub unsafe fn _mm_cvtsd_si64(a: __m128d) -> i64 {
    cvtsd2si64(a)
}

/// Alias for [`_mm_cvtsd_si64`](fn._mm_cvtsd_si64_ss.html).
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsd2si))]
pub unsafe fn _mm_cvtsd_si64x(a: __m128d) -> i64 {
    _mm_cvtsd_si64(a)
}

/// Convert the lower double-precision (64-bit) floating-point element in `a`
/// to a 64-bit integer with truncation.
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvttsd2si))]
pub unsafe fn _mm_cvttsd_si64(a: __m128d) -> i64 {
    cvttsd2si64(a)
}

/// Alias for [`_mm_cvttsd_si64`](fn._mm_cvttsd_si64_ss.html).
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvttsd2si))]
pub unsafe fn _mm_cvttsd_si64x(a: __m128d) -> i64 {
    _mm_cvttsd_si64(a)
}

/// Stores a 64-bit integer value in the specified memory location.
/// To minimize caching, the data is flagged as non-temporal (unlikely to be
/// used again soon).
#[inline(always)]
#[target_feature(enable = "sse2")]
// FIXME movnti on windows and linux x86_64
//#[cfg_attr(test, assert_instr(movntiq))]
pub unsafe fn _mm_stream_si64(mem_addr: *mut i64, a: i64) {
    ::core::intrinsics::nontemporal_store(mem_addr, a);
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, not(windows)), assert_instr(movq))]
pub unsafe fn _mm_cvtsi64_si128(a: i64) -> __m128i {
    _mm_set_epi64x(0, a)
}

/// Return a vector whose lowest element is `a` and all higher elements are
/// `0`.
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, not(windows)), assert_instr(movq))]
pub unsafe fn _mm_cvtsi64x_si128(a: i64) -> __m128i {
    _mm_cvtsi64_si128(a)
}

/// Return the lowest element of `a`.
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, not(windows)), assert_instr(movq))]
pub unsafe fn _mm_cvtsi128_si64(a: __m128i) -> i64 {
    simd_extract(a.as_i64x2(), 0)
}

/// Return the lowest element of `a`.
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(all(test, not(windows)), assert_instr(movq))]
pub unsafe fn _mm_cvtsi128_si64x(a: __m128i) -> i64 {
    _mm_cvtsi128_si64(a)
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsi2sd))]
pub unsafe fn _mm_cvtsi64_sd(a: __m128d, b: i64) -> __m128d {
    simd_insert(a, 0, b as f64)
}

/// Return `a` with its lower element replaced by `b` after converting it to
/// an `f64`.
#[inline(always)]
#[target_feature(enable = "sse2")]
#[cfg_attr(test, assert_instr(cvtsi2sd))]
pub unsafe fn _mm_cvtsi64x_sd(a: __m128d, b: i64) -> __m128d {
    _mm_cvtsi64_sd(a, b)
}

#[cfg(test)]
mod tests {
    use std::{f64, i64};

    use stdsimd_test::simd_test;

    use x86::*;

    #[simd_test = "sse2"]
    unsafe fn test_mm_cvtsd_si64() {

        let r = _mm_cvtsd_si64(_mm_setr_pd(-2.0, 5.0));
        assert_eq!(r, -2_i64);

        let r = _mm_cvtsd_si64(_mm_setr_pd(f64::MAX, f64::MIN));
        assert_eq!(r, i64::MIN);
    }

    #[simd_test = "sse2"]
    unsafe fn test_mm_cvtsd_si64x() {
        let r = _mm_cvtsd_si64x(_mm_setr_pd(f64::NAN, f64::NAN));
        assert_eq!(r, i64::MIN);
    }

    #[simd_test = "sse2"]
    unsafe fn test_mm_cvttsd_si64() {
        let a = _mm_setr_pd(-1.1, 2.2);
        let r = _mm_cvttsd_si64(a);
        assert_eq!(r, -1_i64);
    }

    #[simd_test = "sse2"]
    unsafe fn test_mm_cvttsd_si64x() {
        let a = _mm_setr_pd(f64::NEG_INFINITY, f64::NAN);
        let r = _mm_cvttsd_si64x(a);
        assert_eq!(r, i64::MIN);
    }

    #[simd_test = "sse2"]
    unsafe fn test_mm_stream_si64() {
        let a: i64 = 7;
        let mut mem = ::std::boxed::Box::<i64>::new(-1);
        _mm_stream_si64(&mut *mem as *mut i64, a);
        assert_eq!(a, *mem);
    }

    #[simd_test = "sse2"]
    unsafe fn test_mm_cvtsi64_si128() {
        let r = _mm_cvtsi64_si128(5);
        assert_eq_m128i(r, _mm_setr_epi64x(5, 0));
    }

    #[simd_test = "sse2"]
    unsafe fn test_mm_cvtsi128_si64() {
        let r = _mm_cvtsi128_si64(_mm_setr_epi64x(5, 0));
        assert_eq!(r, 5);
    }

    #[simd_test = "sse2"]
    unsafe fn test_mm_cvtsi64_sd() {
        let a = _mm_set1_pd(3.5);
        let r = _mm_cvtsi64_sd(a, 5);
        assert_eq_m128d(r, _mm_setr_pd(5.0, 3.5));
    }
}
