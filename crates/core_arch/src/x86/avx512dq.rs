use crate::{
    core_arch::{simd::*, simd_llvm::*, x86::*},
    mem::{self, transmute},
    ptr,
};

#[cfg(test)]
use stdarch_test::assert_instr;

/// Compute the bitwise AND of packed double-precision (64-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_and_pd&expand=100,288)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandps))]
// FIXME: should be `vandpd` instruction.
pub unsafe fn _mm512_and_pd(a: __m512d, b: __m512d) -> __m512d {
    transmute(simd_and(transmute::<_, u64x8>(a), transmute::<_, u64x8>(b)))
}

/// Compute the bitwise XOR of packed double-precision (64-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_xor_pd&expand=100,6160)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vxorps))]
// FIXME: should be `vxorpd` instruction.
pub unsafe fn _mm512_xor_pd(a: __m512d, b: __m512d) -> __m512d {
    transmute(simd_xor(transmute::<_, u64x8>(a), transmute::<_, u64x8>(b)))
}

/// Compute the bitwise NOT of packed double-precision (64-bit) floating-point elements in a and then AND with b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_andnot_pd&expand=100,326)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandnps))]
// FIXME: should be `vandpd` instruction.
pub unsafe fn _mm512_andnot_pd(a: __m512d, b: __m512d) -> __m512d {
    _mm512_and_pd(
        _mm512_xor_pd(a, transmute(_mm512_set1_epi64(u64::MAX as i64))),
        b,
    )
}

/// Compute the bitwise NOT of packed double-precision (64-bit) floating-point elements in a and then AND with b, and
/// store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_andnot_pd&expand=100,327)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandnpd))]
pub unsafe fn _mm512_mask_andnot_pd(src: __m512d, k: __mmask8, a: __m512d, b: __m512d) -> __m512d {
    let andnot = _mm512_andnot_pd(a, b).as_f64x8();
    transmute(simd_select_bitmask(k, andnot, src.as_f64x8()))
}

/// Compute the bitwise NOT of packed double-precision (64-bit) floating-point elements in a and then AND with b, and
/// store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_andnot_pd&expand=100,328)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandnpd))]
pub unsafe fn _mm512_maskz_andnot_pd(k: __mmask8, a: __m512d, b: __m512d) -> __m512d {
    let andnot = _mm512_andnot_pd(a, b).as_f64x8();
    let zero = _mm512_setzero_pd().as_f64x8();
    transmute(simd_select_bitmask(k, andnot, zero))
}

/// Compute the bitwise AND of packed single-precision (32-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_and_ps&expand=100,297)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandps))]
pub unsafe fn _mm512_and_ps(a: __m512, b: __m512) -> __m512 {
    transmute(simd_and(a.as_f32x16(), b.as_f32x16()))
}

/// Compute the bitwise XOR of packed single-precision (32-bit) floating-point elements in a and b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_xor_ps&expand=100,6169)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vxorps))]
pub unsafe fn _mm512_xor_ps(a: __m512, b: __m512) -> __m512 {
    transmute(simd_xor(a.as_f32x16(), b.as_f32x16()))
}

/// Compute the bitwise NOT of packed single-precision (32-bit) floating-point elements in a and then AND with b, and store the results in dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_andnot_ps&expand=100,335)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandnps))]
pub unsafe fn _mm512_andnot_ps(a: __m512, b: __m512) -> __m512 {
    _mm512_and_ps(_mm512_xor_ps(a, _mm512_set1_ps(f32::MAX)), b)
}

/// Compute the bitwise NOT of packed single-precision (32-bit) floating-point elements in a and then AND with b, and
/// store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_andnot_ps&expand=100,336)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandnps))]
pub unsafe fn _mm512_mask_andnot_ps(src: __m512, k: __mmask16, a: __m512, b: __m512) -> __m512 {
    let andnot = _mm512_andnot_ps(a, b).as_f32x16();
    transmute(simd_select_bitmask(k, andnot, src.as_f32x16()))
}

/// Compute the bitwise NOT of packed single-precision (32-bit) floating-point elements in a and then AND with b, and
/// store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_andnot_ps&expand=100,337)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandnps))]
pub unsafe fn _mm512_maskz_andnot_ps(k: __mmask16, a: __m512, b: __m512) -> __m512 {
    let andnot = _mm512_andnot_ps(a, b).as_f32x16();
    let zero = _mm512_setzero_ps().as_f32x16();
    transmute(simd_select_bitmask(k, andnot, zero))
}

/// Compute the bitwise AND of packed double-precision (64-bit) floating-point elements in a and b, and
/// store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_and_pd&expand=100,289)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandpd))]
pub unsafe fn _mm512_mask_and_pd(src: __m512d, k: __mmask8, a: __m512d, b: __m512d) -> __m512d {
    let and = _mm512_and_pd(a, b).as_f64x8();
    transmute(simd_select_bitmask(k, and, src.as_f64x8()))
}

/// Compute the bitwise AND of packed double-precision (64-bit) floating-point elements in a and b, and
/// store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_and_pd&expand=100,290)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandpd))]
pub unsafe fn _mm512_maskz_and_pd(k: __mmask8, a: __m512d, b: __m512d) -> __m512d {
    let and = _mm512_and_pd(a, b).as_f64x8();
    let zero = _mm512_setzero_pd().as_f64x8();
    transmute(simd_select_bitmask(k, and, zero))
}

/// Compute the bitwise AND of packed single-precision (32-bit) floating-point elements in a and b, and
/// store the results in dst using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_and_ps&expand=100,298)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandps))]
pub unsafe fn _mm512_mask_and_ps(src: __m512, k: __mmask16, a: __m512, b: __m512) -> __m512 {
    let and = _mm512_and_ps(a, b).as_f32x16();
    transmute(simd_select_bitmask(k, and, src.as_f32x16()))
}

/// Compute the bitwise AND of packed single-precision (32-bit) floating-point elements in a and b, and
/// store the results in dst using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_and_ps&expand=100,299)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vandps))]
pub unsafe fn _mm512_maskz_and_ps(k: __mmask16, a: __m512, b: __m512) -> __m512 {
    let and = _mm512_and_ps(a, b).as_f32x16();
    let zero = _mm512_setzero_ps().as_f32x16();
    transmute(simd_select_bitmask(k, and, zero))
}

/// Broadcast the lower 2 packed single-precision (32-bit) floating-point elements from a to all elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_broadcast_f32x2&expand=100,477)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf32x2))]
pub unsafe fn _mm512_broadcast_f32x2(a: __m128) -> __m512 {
    simd_shuffle8(a, a, [0, 1, 2, 3, 0, 1, 2, 3])
}

/// Broadcast the lower 2 packed single-precision (32-bit) floating-point elements from a to all elements of dst
/// using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_broadcast_f32x2&expand=100,478)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf32x2))]
pub unsafe fn _mm512_mask_broadcast_f32x2(src: __m512, k: __mmask16, a: __m128) -> __m512 {
    let broadcast = _mm512_broadcast_f32x2(a).as_f32x16();
    transmute(simd_select_bitmask(k, broadcast, src.as_f32x16()))
}

/// Broadcast the lower 2 packed single-precision (32-bit) floating-point elements from a to all elements of dst
/// using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_broadcast_f32x2&expand=100,479)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf32x2))]
pub unsafe fn _mm512_maskz_broadcast_f32x2(k: __mmask16, a: __m128) -> __m512 {
    let broadcast = _mm512_broadcast_f32x2(a).as_f32x16();
    let zero = _mm512_setzero_ps().as_f32x16();
    transmute(simd_select_bitmask(k, broadcast, zero))
}

/// Broadcast the 8 packed single-precision (32-bit) floating-point elements from a to all elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_broadcast_f32x8&expand=100,486)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf32x8))] // sequence generation
pub unsafe fn _mm512_broadcast_f32x8(a: __m256) -> __m512 {
    simd_shuffle32(
        a,
        a,
        [
            0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0,
            1, 2, 3,
        ],
    )
}

/// Broadcast the 8 packed single-precision (32-bit) floating-point elements from a to all elements of dst
/// using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_broadcast_f32x8&expand=100,487)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf32x8))] // sequence generation
pub unsafe fn _mm512_mask_broadcast_f32x8(src: __m512, k: __mmask16, a: __m256) -> __m512 {
    let broadcast = _mm512_broadcast_f32x8(a).as_f32x16();
    transmute(simd_select_bitmask(k, broadcast, src.as_f32x16()))
}

/// Broadcast the 8 packed single-precision (32-bit) floating-point elements from a to all elements of dst
/// using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_broadcast_f32x8&expand=100,488)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf32x8))] // sequence generation
pub unsafe fn _mm512_maskz_broadcast_f32x8(k: __mmask16, a: __m256) -> __m512 {
    let broadcast = _mm512_broadcast_f32x8(a).as_f32x16();
    let zero = _mm512_setzero_ps().as_f32x16();
    transmute(simd_select_bitmask(k, broadcast, zero))
}

/// Broadcast the 2 packed double-precision (64-bit) floating-point elements from a to all elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_broadcast_f64x2&expand=100,492)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf64x2))] // sequence generation
pub unsafe fn _mm512_broadcast_f64x2(a: __m128d) -> __m512d {
    simd_shuffle4(a, a, [0, 1, 2, 3])
}

/// Broadcast the 2 packed double-precision (64-bit) floating-point elements from a to all elements of dst
/// using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_broadcast_f64x2&expand=100,493)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf64x2))] // sequence generation
pub unsafe fn _mm512_mask_broadcast_f64x2(src: __m512d, k: __mmask8, a: __m128d) -> __m512d {
    let broadcast = _mm512_broadcast_f64x2(a).as_f64x8();
    transmute(simd_select_bitmask(k, broadcast, src.as_f64x8()))
}

/// Broadcast the 2 packed double-precision (64-bit) floating-point elements from a to all elements of dst
/// using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_broadcast_f64x2&expand=100,494)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcastf64x2))] // sequence generation
pub unsafe fn _mm512_maskz_broadcast_f64x2(k: __mmask8, a: __m128d) -> __m512d {
    let broadcast = _mm512_broadcast_f64x2(a).as_f64x8();
    let zero = _mm512_setzero_pd().as_f64x8();
    transmute(simd_select_bitmask(k, broadcast, zero))
}

/// Broadcast the lower 2 packed 32-bit integers from a to all elements of dst.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_broadcast_i32x2&expand=100,504)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcasti32x2))]
pub unsafe fn _mm512_broadcast_i32x2(a: __m128i) -> __m512i {
    let a = _mm512_castsi128_si512(a).as_i32x16();
    let ret: i32x16 = simd_shuffle8(a, a, [0, 1, 2, 3, 0, 1, 2, 3]);
    transmute(ret)
}

/// Broadcast the lower 2 packed 32-bit integers from a to all elements of dst
/// using writemask k (elements are copied from src when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_mask_broadcast_i32x2&expand=100,505)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcasti32x2))]
pub unsafe fn _mm512_mask_broadcast_i32x2(src: __m512i, k: __mmask16, a: __m128i) -> __m512i {
    let broadcast = _mm512_broadcast_i32x2(a).as_i32x16();
    transmute(simd_select_bitmask(k, broadcast, src.as_i32x16()))
}

/// Broadcast the lower 2 packed 32-bit integers from a to all elements of dst
/// using zeromask k (elements are zeroed out when the corresponding mask bit is not set).
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm512_maskz_broadcast_i32x2&expand=100,506)
#[inline]
#[target_feature(enable = "avx512dq")]
#[cfg_attr(test, assert_instr(vbroadcasti32x2))]
pub unsafe fn _mm512_maskz_broadcast_i32x2(k: __mmask16, a: __m128i) -> __m512i {
    let broadcast = _mm512_broadcast_i32x2(a).as_i32x16();
    let zero = _mm512_setzero_epi32().as_i32x16();
    transmute(simd_select_bitmask(k, broadcast, zero))
}

#[cfg(test)]
mod tests {
    use super::*;

    use stdarch_test::simd_test;

    use crate::core_arch::x86::*;
    use crate::core_arch::x86_64::*;
    use crate::hint::black_box;

    #[simd_test(enable = "avx512dq")]
    unsafe fn test_mm512_and_pd() {
        let a = _mm512_set_pd(
            2.34_f64, 2.34_f64, 2.34_f64, 2.34_f64, 0.0_f64, 0.0_f64, 0.0_f64, 8.94_f64,
        );
        let b = _mm512_set_pd(
            0.0_f64, 2.34_f64, 0.0_f64, 0.0_f64, 8.94_f64, 8.94_f64, 8.94_f64, 8.94_f64,
        );
        let r = _mm512_and_pd(a, b);
        let e = _mm512_set_pd(
            0.0_f64, 2.34_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 8.94_f64,
        );
        assert_eq_m512d(r, e);
    }

    #[simd_test(enable = "avx512dq")]
    unsafe fn test_mm512_xor_pd() {
        let a = _mm512_set_pd(
            2.34_f64, 2.34_f64, 2.34_f64, 2.34_f64, 0.0_f64, 0.0_f64, 0.0_f64, 8.94_f64,
        );
        let b = _mm512_set_pd(
            0.0_f64, 2.34_f64, 0.0_f64, 0.0_f64, 8.94_f64, 8.94_f64, 8.94_f64, 8.94_f64,
        );
        let r = _mm512_xor_pd(a, b);
        let e = _mm512_set_pd(
            2.34_f64, 0.0_f64, 2.34_f64, 2.34_f64, 8.94_f64, 8.94_f64, 8.94_f64, 0.0_f64,
        );
        assert_eq_m512d(r, e);
    }

    #[simd_test(enable = "avx512dq")]
    unsafe fn test_mm512_andnot_pd() {
        let a = _mm512_set1_pd(f64::from_bits(
            0b0000000000001111111111111111111111111111111111111111111111111111,
        ));
        let b = _mm512_set1_pd(0.123456_f64);
        let r = _mm512_andnot_pd(a, b);
        let e = _mm512_set1_pd(0.0625_f64);
        assert_eq_m512d(r, e);
    }
}
