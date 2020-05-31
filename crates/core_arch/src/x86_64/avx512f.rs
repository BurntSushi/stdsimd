use crate::{
    core_arch::{simd::*, x86::*},
    mem::transmute,
};

/// Sets packed 64-bit integers in `dst` with the supplied values.
///
/// [Intel's documentation]( https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,4909&text=_mm512_set_pd)
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_set_pd(
    e0: f64,
    e1: f64,
    e2: f64,
    e3: f64,
    e4: f64,
    e5: f64,
    e6: f64,
    e7: f64,
) -> __m512d {
    _mm512_setr_pd(e7, e6, e5, e4, e3, e2, e1, e0)
}

/// Sets packed 64-bit integers in `dst` with the supplied values in
/// reverse order.
///
/// [Intel's documentation]( https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,4909&text=_mm512_set_pd)
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_setr_pd(
    e0: f64,
    e1: f64,
    e2: f64,
    e3: f64,
    e4: f64,
    e5: f64,
    e6: f64,
    e7: f64,
) -> __m512d {
    let r = f64x8::new(e0, e1, e2, e3, e4, e5, e6, e7);
    transmute(r)
}

/// Sets packed 64-bit integers in `dst` with the supplied values.
///
/// [Intel's documentation]( https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,4909&text=_mm512_set_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_set_epi64(
    e0: i64,
    e1: i64,
    e2: i64,
    e3: i64,
    e4: i64,
    e5: i64,
    e6: i64,
    e7: i64,
) -> __m512i {
    _mm512_setr_epi64(e7, e6, e5, e4, e3, e2, e1, e0)
}

/// Sets packed 64-bit integers in `dst` with the supplied values in
/// reverse order.
///
/// [Intel's documentation]( https://software.intel.com/sites/landingpage/IntrinsicsGuide/#expand=727,1063,4909,1062,1062,4909&text=_mm512_set_epi64)
#[inline]
#[target_feature(enable = "avx512f")]
pub unsafe fn _mm512_setr_epi64(
    e0: i64,
    e1: i64,
    e2: i64,
    e3: i64,
    e4: i64,
    e5: i64,
    e6: i64,
    e7: i64,
) -> __m512i {
    let r = i64x8::new(e0, e1, e2, e3, e4, e5, e6, e7);
    transmute(r)
}

#[cfg(test)]
mod tests {
    use std;
    use stdarch_test::simd_test;

    use crate::core_arch::x86::*;
    use crate::core_arch::x86_64::*;

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_setzero_pd() {
        assert_eq_m512d(_mm512_setzero_pd(), _mm512_set1_pd(0.));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_set1_pd() {
        let expected = _mm512_set_pd(2., 2., 2., 2., 2., 2., 2., 2.);
        assert_eq_m512d(expected, _mm512_set1_pd(2.));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmplt_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let m = _mm512_cmplt_epu64_mask(a, b);
        assert_eq!(m, 0b11001111);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmplt_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        let r = _mm512_mask_cmplt_epu64_mask(mask, a, b);
        assert_eq!(r, 0b01001010);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpgt_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let m = _mm512_cmpgt_epu64_mask(b, a);
        assert_eq!(m, 0b11001111);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpgt_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        let r = _mm512_mask_cmpgt_epu64_mask(mask, b, a);
        assert_eq!(r, 0b01001010);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmple_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        assert_eq!(_mm512_cmple_epu64_mask(a, b), _mm512_cmpgt_epu64_mask(b, a))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmple_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        assert_eq!(
            _mm512_mask_cmple_epu64_mask(mask, a, b),
            _mm512_mask_cmpgt_epu64_mask(mask, b, a)
        );
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpge_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        assert_eq!(_mm512_cmpge_epu64_mask(a, b), _mm512_cmplt_epu64_mask(b, a))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpge_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        assert_eq!(
            _mm512_mask_cmpge_epu64_mask(mask, a, b),
            _mm512_mask_cmplt_epu64_mask(mask, b, a)
        );
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpeq_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set_epi64(0, 1, 13, 42, i64::MAX, i64::MIN, 100, -100);
        let m = _mm512_cmpeq_epu64_mask(b, a);
        assert_eq!(m, 0b11001111);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpeq_epu64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set_epi64(0, 1, 13, 42, i64::MAX, i64::MIN, 100, -100);
        let mask = 0b01111010;
        let r = _mm512_mask_cmpeq_epu64_mask(mask, b, a);
        assert_eq!(r, 0b01001010);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmplt_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let m = _mm512_cmplt_epi64_mask(a, b);
        assert_eq!(m, 0b00000101);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmplt_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01100110;
        let r = _mm512_mask_cmplt_epi64_mask(mask, a, b);
        assert_eq!(r, 0b00000100);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpgt_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let m = _mm512_cmpgt_epi64_mask(b, a);
        assert_eq!(m, 0b00000101);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpgt_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01100110;
        let r = _mm512_mask_cmpgt_epi64_mask(mask, b, a);
        assert_eq!(r, 0b00000100);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmple_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        assert_eq!(_mm512_cmple_epi64_mask(a, b), _mm512_cmpgt_epi64_mask(b, a))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmple_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        assert_eq!(
            _mm512_mask_cmple_epi64_mask(mask, a, b),
            _mm512_mask_cmpgt_epi64_mask(mask, b, a)
        );
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpge_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        assert_eq!(_mm512_cmpge_epi64_mask(a, b), _mm512_cmplt_epi64_mask(b, a))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpge_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, u64::MAX as i64, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set1_epi64(-1);
        let mask = 0b01111010;
        assert_eq!(
            _mm512_mask_cmpge_epi64_mask(mask, a, b),
            _mm512_mask_cmplt_epi64_mask(mask, b, a)
        );
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_cmpeq_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set_epi64(0, 1, 13, 42, i64::MAX, i64::MIN, 100, -100);
        let m = _mm512_cmpeq_epi64_mask(b, a);
        assert_eq!(m, 0b11001111);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_cmpeq_epi64_mask() {
        let a = _mm512_set_epi64(0, 1, -1, 13, i64::MAX, i64::MIN, 100, -100);
        let b = _mm512_set_epi64(0, 1, 13, 42, i64::MAX, i64::MIN, 100, -100);
        let mask = 0b01111010;
        let r = _mm512_mask_cmpeq_epi64_mask(mask, b, a);
        assert_eq!(r, 0b01001010);
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_set_pd() {
        let r = _mm512_setr_pd(0., 1., 2., 3., 4., 5., 6., 7.);
        assert_eq_m512d(r, _mm512_set_pd(7., 6., 5., 4., 3., 2., 1., 0.));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_setr_pd() {
        let r = _mm512_set_pd(0., 1., 2., 3., 4., 5., 6., 7.);
        assert_eq_m512d(r, _mm512_setr_pd(7., 6., 5., 4., 3., 2., 1., 0.));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_set_epi64() {
        let r = _mm512_setr_epi64(0, 1, 2, 3, 4, 5, 6, 7);
        assert_eq_m512i(r, _mm512_set_epi64(7, 6, 5, 4, 3, 2, 1, 0))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_setr_epi64() {
        let r = _mm512_set_epi64(0, 1, 2, 3, 4, 5, 6, 7);
        assert_eq_m512i(r, _mm512_setr_epi64(7, 6, 5, 4, 3, 2, 1, 0))
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_i32gather_pd() {
        let mut arr = [0f64; 128];
        for i in 0..128 {
            arr[i] = i as f64;
        }
        // A multiplier of 8 is word-addressing
        #[rustfmt::skip]
        let index = _mm256_setr_epi32(0, 16, 32, 48, 64, 80, 96, 112);
        let r = _mm512_i32gather_pd(index, arr.as_ptr() as *const u8, 8);
        assert_eq_m512d(r, _mm512_setr_pd(0., 16., 32., 48., 64., 80., 96., 112.));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_i32gather_pd() {
        let mut arr = [0f64; 128];
        for i in 0..128 {
            arr[i] = i as f64;
        }
        let src = _mm512_set1_pd(2.);
        let mask = 0b10101010;
        #[rustfmt::skip]
        let index = _mm256_setr_epi32(0, 16, 32, 48, 64, 80, 96, 112);
        // A multiplier of 8 is word-addressing
        let r = _mm512_mask_i32gather_pd(src, mask, index, arr.as_ptr() as *const u8, 8);
        assert_eq_m512d(r, _mm512_setr_pd(2., 16., 2., 48., 2., 80., 2., 112.));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_i64gather_pd() {
        let mut arr = [0f64; 128];
        for i in 0..128 {
            arr[i] = i as f64;
        }
        // A multiplier of 8 is word-addressing
        #[rustfmt::skip]
        let index = _mm512_setr_epi64(0, 16, 32, 48, 64, 80, 96, 112);
        let r = _mm512_i64gather_pd(index, arr.as_ptr() as *const u8, 8);
        assert_eq_m512d(r, _mm512_setr_pd(0., 16., 32., 48., 64., 80., 96., 112.));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_i64gather_pd() {
        let mut arr = [0f64; 128];
        for i in 0..128 {
            arr[i] = i as f64;
        }
        let src = _mm512_set1_pd(2.);
        let mask = 0b10101010;
        #[rustfmt::skip]
        let index = _mm512_setr_epi64(0, 16, 32, 48, 64, 80, 96, 112);
        // A multiplier of 8 is word-addressing
        let r = _mm512_mask_i64gather_pd(src, mask, index, arr.as_ptr() as *const u8, 8);
        assert_eq_m512d(r, _mm512_setr_pd(2., 16., 2., 48., 2., 80., 2., 112.));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_i32gather_epi64() {
        let mut arr = [0i64; 128];
        for i in 0..128i64 {
            arr[i as usize] = i;
        }
        // A multiplier of 8 is word-addressing
        #[rustfmt::skip]
        let index = _mm256_setr_epi32(0, 16, 32, 48, 64, 80, 96, 112);
        let r = _mm512_i32gather_epi64(index, arr.as_ptr() as *const u8, 8);
        assert_eq_m512i(r, _mm512_setr_epi64(0, 16, 32, 48, 64, 80, 96, 112));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_i32gather_epi64() {
        let mut arr = [0i64; 128];
        for i in 0..128i64 {
            arr[i as usize] = i;
        }
        let src = _mm512_set1_epi64(2);
        let mask = 0b10101010;
        #[rustfmt::skip]
        let index = _mm256_setr_epi32(0, 16, 32, 48, 64, 80, 96, 112);
        // A multiplier of 8 is word-addressing
        let r = _mm512_mask_i32gather_epi64(src, mask, index, arr.as_ptr() as *const u8, 8);
        assert_eq_m512i(r, _mm512_setr_epi64(2, 16, 2, 48, 2, 80, 2, 112));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_i64gather_epi64() {
        let mut arr = [0i64; 128];
        for i in 0..128i64 {
            arr[i as usize] = i;
        }
        // A multiplier of 8 is word-addressing
        #[rustfmt::skip]
        let index = _mm512_setr_epi64(0, 16, 32, 48, 64, 80, 96, 112);
        let r = _mm512_i64gather_epi64(index, arr.as_ptr() as *const u8, 8);
        assert_eq_m512i(r, _mm512_setr_epi64(0, 16, 32, 48, 64, 80, 96, 112));
    }

    #[simd_test(enable = "avx512f")]
    unsafe fn test_mm512_mask_i64gather_epi64() {
        let mut arr = [0i64; 128];
        for i in 0..128i64 {
            arr[i as usize] = i;
        }
        let src = _mm512_set1_epi64(2);
        let mask = 0b10101010;
        #[rustfmt::skip]
        let index = _mm512_setr_epi64(0, 16, 32, 48, 64, 80, 96, 112);
        // A multiplier of 8 is word-addressing
        let r = _mm512_mask_i64gather_epi64(src, mask, index, arr.as_ptr() as *const u8, 8);
        assert_eq_m512i(r, _mm512_setr_epi64(2, 16, 2, 48, 2, 80, 2, 112));
    }
}
