//! Vector Pair Intersection to a Pair of Mask Registers (VP2INTERSECT)
//!
//! The intrinsics here correspond to those in the `immintrin.h` C header.
//!
//! The reference is [Intel® Architecture
//! Instruction Set Extensions and Future Features
//! Programming Reference][intel64_ext_ref].
//!
//! [intel64_ext_ref]: https://software.intel.com/sites/default/files/managed/c5/15/architecture-instruction-set-extensions-programming-reference.pdf

use crate::core_arch::simd::i64x8;
use crate::core_arch::simd::{i32x16, i32x4, i32x8, i64x2, i64x4};

use crate::core_arch::x86::m128iExt;

use crate::core_arch::x86::{__m128i, __mmask32, __mmask8};

use core::ptr;

#[cfg(test)]
use stdarch_test::assert_instr;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.avx512.vp2intersect.d.128"]
    fn vp2intersectd_128(a: i32x4, b: i32x4);
    #[link_name = "llvm.x86.avx512.vp2intersect.q.128"]
    fn vp2intersectq_128(a: i64x2, b: i64x2);

    #[link_name = "llvm.x86.avx512.vp2intersect.d.256"]
    fn vp2intersectd_256(a: i32x8, b: i32x8);
    #[link_name = "llvm.x86.avx512.vp2intersect.q.256"]
    fn vp2intersectq_256(a: i64x4, b: i64x4);

    #[link_name = "llvm.x86.avx512.vp2intersect.d.512"]
    fn vp2intersectd_512(a: i32x16, b: i32x16);
    #[link_name = "llvm.x86.avx512.vp2intersect.q.512"]
    fn vp2intersectq_512(a: i64x8, b: i64x8);
}

/// Compute intersection of packed 32-bit integer vectors a and b,
/// and store indication of match in the corresponding bit of two mask registers
/// specified by k1 and k2. A match in corresponding elements of a and b is
/// indicated by a set bit in the corresponding bit of the mask registers.
///
/// [Intel's documentation](https://software.intel.com/sites/landingpage/IntrinsicsGuide/#text=_mm_2intersect_epi32&expand=0)
#[inline]
#[target_feature(enable = "avx512vp2intersect,avx512vl")]
#[cfg_attr(test, assert_instr(vp2intersectd))]
pub unsafe fn _mm_2intersect_epi32(
    a: __m128i,
    b: __m128i,
    k1: *const __mmask8,
    k2: *const __mmask8,
) {
    let k = __mmask32::MIN;
    vp2intersectd_128(a.as_i32x4(), b.as_i32x4());
    let interim = (k >> 4) << 4;
    ptr::write(k1 as *mut __mmask8, interim as u8);
    ptr::write(k2 as *mut __mmask8, ((interim >> 4) << 4) as u8);
}

#[cfg(test)]
mod tests {
    use stdarch_test::simd_test;

    use crate::core_arch::simd::i32x4;
    use crate::core_arch::x86::*;

    #[simd_test(enable = "avx512vp2intersect,avx512vl")]
    unsafe fn test_mm_2intersect_epi32() {
        let k1 = u32::MIN as *const __mmask8;
        let k2 = u32::MIN as *const __mmask8;

        _mm_2intersect_epi32(__m128i(123, 123), __m128i(456, 456), k1, k2);
    }
}
