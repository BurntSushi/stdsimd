use std::mem;

#[cfg(test)]
use stdsimd_test::assert_instr;

use simd_llvm::simd_shuffle4;
use v256::*;

/// Add packed double-precision (64-bit) floating-point elements
/// in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vaddpd))]
pub unsafe fn _mm256_add_pd(a: f64x4, b: f64x4) -> f64x4 {
    a + b
}

/// Add packed single-precision (32-bit) floating-point elements in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vaddps))]
pub unsafe fn _mm256_add_ps(a: f32x8, b: f32x8) -> f32x8 {
    a + b
}

/// Compute the bitwise AND of a packed double-precision (64-bit) floating-point elements
/// in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
// Should be 'vandpd' instuction.
// See https://github.com/rust-lang-nursery/stdsimd/issues/71
#[cfg_attr(test, assert_instr(vandps))]
pub unsafe fn _mm256_and_pd(a: f64x4, b: f64x4) -> f64x4 {
    let a: u64x4 = mem::transmute(a);
    let b: u64x4 = mem::transmute(b);
    mem::transmute(a & b)
}

/// Compute the bitwise AND of packed single-precision (32-bit) floating-point elements in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vandps))]
pub unsafe fn _mm256_and_ps(a: f32x8, b: f32x8) -> f32x8 {
    let a: u32x8 = mem::transmute(a);
    let b: u32x8 = mem::transmute(b);
    mem::transmute(a & b)
}

/// Compute the bitwise OR packed double-precision (64-bit) floating-point elements
/// in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
// Should be 'vorpd' instuction.
// See https://github.com/rust-lang-nursery/stdsimd/issues/71
#[cfg_attr(test, assert_instr(vorps))]
pub unsafe fn _mm256_or_pd(a: f64x4, b: f64x4) -> f64x4 {
    let a: u64x4 = mem::transmute(a);
    let b: u64x4 = mem::transmute(b);
    mem::transmute(a | b)
}

/// Compute the bitwise OR packed single-precision (32-bit) floating-point elements in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vorps))]
pub unsafe fn _mm256_or_ps(a: f32x8, b: f32x8) -> f32x8 {
    let a: u32x8 = mem::transmute(a);
    let b: u32x8 = mem::transmute(b);
    mem::transmute(a | b)
}

/// Compute the bitwise NOT of packed double-precision (64-bit) floating-point elements in `a`
/// and then AND with `b`.
#[inline(always)]
#[target_feature = "+avx"]
// Should be 'vandnpd' instruction.
#[cfg_attr(test, assert_instr(vandnps))]
pub unsafe fn _mm256_andnot_pd(a: f64x4, b: f64x4) -> f64x4 {
    let a: u64x4 = mem::transmute(a);
    let b: u64x4 = mem::transmute(b);
    mem::transmute((!a) & b)
}

/// Compute the bitwise NOT of packed single-precision (32-bit) floating-point elements in `a`
/// and then AND with `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vandnps))]
pub unsafe fn _mm256_andnot_ps(a: f32x8, b: f32x8) -> f32x8 {
    let a: u32x8 = mem::transmute(a);
    let b: u32x8 = mem::transmute(b);
    mem::transmute((!a) & b)
}

/// Compare packed double-precision (64-bit) floating-point elements 
/// in `a` and `b`, and return packed maximum values
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vmaxpd))]
pub unsafe fn _mm256_max_pd(a: f64x4, b: f64x4) -> f64x4 {
    maxpd256(a, b)
}

/// Compare packed single-precision (32-bit) floating-point elements in `a` and `b`, 
/// and return packed maximum values
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vmaxps))]
pub unsafe fn _mm256_max_ps(a: f32x8, b: f32x8) -> f32x8 {
    maxps256(a, b)
}

/// Compare packed double-precision (64-bit) floating-point elements 
/// in `a` and `b`, and return packed minimum values
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vminpd))]
pub unsafe fn _mm256_min_pd(a: f64x4, b: f64x4) -> f64x4 {
    minpd256(a, b)
}

/// Compare packed single-precision (32-bit) floating-point elements in `a` and `b`, 
/// and return packed minimum values
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vminps))]
pub unsafe fn _mm256_min_ps(a: f32x8, b: f32x8) -> f32x8 {
    minps256(a, b)
}

/// Add packed double-precision (64-bit) floating-point elements
/// in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vmulpd))]
pub unsafe fn _mm256_mul_pd(a: f64x4, b: f64x4) -> f64x4 {
    a * b
}

/// Add packed single-precision (32-bit) floating-point elements in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vmulps))]
pub unsafe fn _mm256_mul_ps(a: f32x8, b: f32x8) -> f32x8 {
    a * b
}

/// Alternatively add and subtract packed double-precision (64-bit)
/// floating-point elements in `a` to/from packed elements in `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vaddsubpd))]
pub unsafe fn _mm256_addsub_pd(a: f64x4, b: f64x4) -> f64x4 {
    addsubpd256(a, b)
}

/// Alternatively add and subtract packed single-precision (32-bit)
/// floating-point elements in `a` to/from packed elements in `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vaddsubps))]
pub unsafe fn _mm256_addsub_ps(a: f32x8, b: f32x8) -> f32x8 {
    addsubps256(a, b)
}

/// Subtract packed double-precision (64-bit) floating-point elements in `b`
/// from packed elements in `a`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vsubpd))]
pub unsafe fn _mm256_sub_pd(a: f64x4, b: f64x4) -> f64x4 {
    a - b
}

/// Subtract packed single-precision (32-bit) floating-point elements in `b`
/// from packed elements in `a`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vsubps))]
pub unsafe fn _mm256_sub_ps(a: f32x8, b: f32x8) -> f32x8 {
    a - b
}

/// Compute the division of each of the 8 packed 32-bit floating-point elements
/// in `a` by the corresponding packed elements in `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vdivps))]
pub unsafe fn _mm256_div_ps(a: f32x8, b: f32x8) -> f32x8 {
    a / b
}

/// Compute the division of each of the 4 packed 64-bit floating-point elements
/// in `a` by the corresponding packed elements in `b`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vdivpd))]
pub unsafe fn _mm256_div_pd(a: f64x4, b: f64x4) -> f64x4 {
    a / b
}


/// Round packed double-precision (64-bit) floating point elements in `a`
/// according to the flag `b`. The value of `b` may be as follows:
///
/// ```ignore
/// 0x00: Round to the nearest whole number.
/// 0x01: Round down, toward negative infinity.
/// 0x02: Round up, toward positive infinity.
/// 0x03: Truncate the values.
/// ```
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vroundpd, b = 0x3))]
pub unsafe fn _mm256_round_pd(a: f64x4, b: i32) -> f64x4 {
    macro_rules! call {
        ($imm8:expr) => { roundpd256(a, $imm8) }
    }
    constify_imm8!(b, call)
}

/// Round packed double-precision (64-bit) floating point elements in `a` toward
/// positive infinity.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vroundpd))]
pub unsafe fn _mm256_ceil_pd(a: f64x4) -> f64x4 {
    roundpd256(a, 0x02)
}

/// Round packed double-precision (64-bit) floating point elements in `a` toward
/// negative infinity.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vroundpd))]
pub unsafe fn _mm256_floor_pd(a: f64x4) -> f64x4 {
    roundpd256(a, 0x01)
}

/// Round packed single-precision (32-bit) floating point elements in `a`
/// according to the flag `b`. The value of `b` may be as follows:
/// 0x00: Round to the nearest whole number.
/// 0x01: Round down, toward negative infinity.
/// 0x02: Round up, toward positive infinity.
/// 0x03: Truncate the values.
/// For a few additional values options, check the LLVM docs:
/// https://github.com/llvm-mirror/clang/blob/dcd8d797b20291f1a6b3e0ddda085aa2bbb382a8/lib/Headers/avxintrin.h#L382
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vroundps, b = 0x00))]
pub unsafe fn _mm256_round_ps(a: f32x8, b: i32) -> f32x8 {
    macro_rules! call {
        ($imm8:expr) => {
            roundps256(a, $imm8)
        }
    }
    constify_imm8!(b, call)
}

/// Round packed single-precision (32-bit) floating point elements in `a` toward
/// positive infinity.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vroundps))]
pub unsafe fn _mm256_ceil_ps(a: f32x8) -> f32x8 {
    roundps256(a, 0x02)
}

/// Round packed single-precision (32-bit) floating point elements in `a` toward
/// negative infinity.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vroundps))]
pub unsafe fn _mm256_floor_ps(a: f32x8) -> f32x8 {
    roundps256(a, 0x01)
}

/// Return the square root of packed single-precision (32-bit) floating point
/// elements in `a`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vsqrtps))]
pub unsafe fn _mm256_sqrt_ps(a: f32x8) -> f32x8 {
    sqrtps256(a)
}

/// Return the square root of packed double-precision (64-bit) floating point
/// elements in `a`.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vsqrtpd))]
pub unsafe fn _mm256_sqrt_pd(a: f64x4) -> f64x4 {
    sqrtpd256(a)
}

/// Blend packed double-precision (64-bit) floating-point elements from
/// `a` and `b` using control mask `imm8`.
#[inline(always)]
#[target_feature = "+avx"]
// FIXME too many instructions in the disassembly
//#[cfg_attr(test, assert_instr(vblendpd, imm8 = 0))]
pub unsafe fn _mm256_blend_pd(a: f64x4, b: f64x4, imm8: i32) -> f64x4 {
    macro_rules! blend4 {
        ($a:expr, $b:expr, $c:expr, $d:expr) => {
            simd_shuffle4(a, b, [$a, $b, $c, $d]);
        }
    }
    macro_rules! blend3 {
        ($a:expr, $b: expr, $c: expr) => {
            match imm8 & 0x8 {
                0 => blend4!($a, $b, $c, 3),
                _ => blend4!($a, $b, $c, 7),
            }
        }
    }
    macro_rules! blend2 {
        ($a:expr, $b:expr) => {
            match imm8 & 0x4 {
                0 => blend3!($a, $b, 2),
                _ => blend3!($a, $b, 6),
            }
        }
    }
    macro_rules! blend1 {
        ($a:expr) => {
            match imm8 & 0x2 {
                0 => blend2!($a, 1),
                _ => blend2!($a, 5),
            }
        }
    }
    match imm8 & 0x1 {
        0 => blend1!(0),
        _ => blend1!(4),
    }
}

/// Blend packed double-precision (64-bit) floating-point elements from
/// `a` and `b` using `c` as a mask.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vblendvpd))]
pub unsafe fn _mm256_blendv_pd(a: f64x4, b: f64x4, c: f64x4) -> f64x4 {
    vblendvpd(a, b, c)
}

/// Blend packed single-precision (32-bit) floating-point elements from
/// `a` and `b` using `c` as a mask.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vblendvps))]
pub unsafe fn _mm256_blendv_ps(a: f32x8, b: f32x8, c: f32x8) -> f32x8 {
    vblendvps(a, b, c)
}

/// Horizontally add adjacent pairs of double-precision (64-bit) floating-point
/// elements in `a` and `b`, and pack the results.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vhaddpd))]
pub unsafe fn _mm256_hadd_pd(a: f64x4, b: f64x4) -> f64x4 {
    vhaddpd(a, b)
}

/// Horizontally add adjacent pairs of single-precision (32-bit) floating-point
/// elements in `a` and `b`, and pack the results.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vhaddps))]
pub unsafe fn _mm256_hadd_ps(a: f32x8, b: f32x8) -> f32x8 {
    vhaddps(a, b)
}

/// Horizontally subtract adjacent pairs of double-precision (64-bit) floating-point
/// elements in `a` and `b`, and pack the results.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vhsubpd))]
pub unsafe fn _mm256_hsub_pd(a: f64x4, b: f64x4) -> f64x4 {
    vhsubpd(a, b)
}

/// Horizontally subtract adjacent pairs of single-precision (32-bit) floating-point
/// elements in `a` and `b`, and pack the results.
#[inline(always)]
#[target_feature = "+avx"]
#[cfg_attr(test, assert_instr(vhsubps))]
pub unsafe fn _mm256_hsub_ps(a: f32x8, b: f32x8) -> f32x8 {
    vhsubps(a, b)
}

/// Compute the bitwise XOR of packed double-precision (64-bit) floating-point
/// elements in `a` and `b`.
#[inline(always)]
#[target_feature = "+avx"]
// FIXME Should be 'vxorpd' instruction.
#[cfg_attr(test, assert_instr(vxorps))]
pub unsafe fn _mm256_xor_pd(a: f64x4, b: f64x4) -> f64x4 {
    let a: u64x4 = mem::transmute(a);
    let b: u64x4 = mem::transmute(b);
    mem::transmute(a ^ b)
}

/// LLVM intrinsics used in the above functions
#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.x86.avx.addsub.pd.256"]
    fn addsubpd256(a: f64x4, b: f64x4) -> f64x4;
    #[link_name = "llvm.x86.avx.addsub.ps.256"]
    fn addsubps256(a: f32x8, b: f32x8) -> f32x8;
    #[link_name = "llvm.x86.avx.max.pd.256"]
    fn maxpd256(a: f64x4, b: f64x4) -> f64x4;
    #[link_name = "llvm.x86.avx.max.ps.256"]
    fn maxps256(a: f32x8, b: f32x8) -> f32x8;
    #[link_name = "llvm.x86.avx.min.pd.256"]
    fn minpd256(a: f64x4, b: f64x4) -> f64x4;
    #[link_name = "llvm.x86.avx.min.ps.256"]
    fn minps256(a: f32x8, b: f32x8) -> f32x8;
    #[link_name = "llvm.x86.avx.round.pd.256"]
    fn roundpd256(a: f64x4, b: i32) -> f64x4;
    #[link_name = "llvm.x86.avx.round.ps.256"]
    fn roundps256(a: f32x8, b: i32) -> f32x8;
    #[link_name = "llvm.x86.avx.sqrt.pd.256"]
    fn sqrtpd256(a: f64x4) -> f64x4;
    #[link_name = "llvm.x86.avx.sqrt.ps.256"]
    fn sqrtps256(a: f32x8) -> f32x8;
    #[link_name = "llvm.x86.avx.blendv.pd.256"]
    fn vblendvpd(a: f64x4, b: f64x4, c: f64x4) -> f64x4;
    #[link_name = "llvm.x86.avx.blendv.ps.256"]
    fn vblendvps(a: f32x8, b: f32x8, c: f32x8) -> f32x8;
    #[link_name = "llvm.x86.avx.hadd.pd.256"]
    fn vhaddpd(a: f64x4, b: f64x4) -> f64x4;
    #[link_name = "llvm.x86.avx.hadd.ps.256"]
    fn vhaddps(a: f32x8, b: f32x8) -> f32x8;
    #[link_name = "llvm.x86.avx.hsub.pd.256"]
    fn vhsubpd(a: f64x4, b: f64x4) -> f64x4;
    #[link_name = "llvm.x86.avx.hsub.ps.256"]
    fn vhsubps(a: f32x8, b: f32x8) -> f32x8;
}

#[cfg(test)]
mod tests {
    use stdsimd_test::simd_test;

    use v256::*;
    use x86::avx;

    #[simd_test = "avx"]
    unsafe fn _mm256_add_pd() {
        let a = f64x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f64x4::new(5.0, 6.0, 7.0, 8.0);
        let r = avx::_mm256_add_pd(a, b);
        let e = f64x4::new(6.0, 8.0, 10.0, 12.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_add_ps() {
        let a = f32x8::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = f32x8::new(9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let r = avx::_mm256_add_ps(a, b);
        let e = f32x8::new(10.0, 12.0, 14.0, 16.0, 18.0, 20.0, 22.0, 24.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_and_pd() {
        let a = f64x4::splat(1.0);
        let b = f64x4::splat(0.6);
        let r = avx::_mm256_and_pd(a, b);
        let e = f64x4::splat(0.5);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_and_ps() {
        let a = f32x8::splat(1.0);
        let b = f32x8::splat(0.6);
        let r = avx::_mm256_and_ps(a, b);
        let e = f32x8::splat(0.5);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_or_pd() {
        let a = f64x4::splat(1.0);
        let b = f64x4::splat(0.6);
        let r = avx::_mm256_or_pd(a, b);
        let e = f64x4::splat(1.2);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_or_ps() {
        let a = f32x8::splat(1.0);
        let b = f32x8::splat(0.6);
        let r = avx::_mm256_or_ps(a, b);
        let e = f32x8::splat(1.2);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_andnot_pd() {
        let a = f64x4::splat(0.0);
        let b = f64x4::splat(0.6);
        let r = avx::_mm256_andnot_pd(a, b);
        assert_eq!(r, b);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_andnot_ps() {
        let a = f32x8::splat(0.0);
        let b = f32x8::splat(0.6);
        let r = avx::_mm256_andnot_ps(a, b);
        assert_eq!(r, b);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_max_pd() {
        let a = f64x4::new(1.0, 4.0, 5.0, 8.0);
        let b = f64x4::new(2.0, 3.0, 6.0, 7.0);
        let r = avx::_mm256_max_pd(a, b);
        let e = f64x4::new(2.0, 4.0, 6.0, 8.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_max_ps() {
        let a = f32x8::new(1.0, 4.0, 5.0, 8.0, 9.0, 12.0, 13.0, 16.0);
        let b = f32x8::new(2.0, 3.0, 6.0, 7.0, 10.0, 11.0, 14.0, 15.0);
        let r = avx::_mm256_max_ps(a, b);
        let e = f32x8::new(2.0, 4.0, 6.0, 8.0, 10.0, 12.0, 14.0, 16.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_min_pd() {
        let a = f64x4::new(1.0, 4.0, 5.0, 8.0);
        let b = f64x4::new(2.0, 3.0, 6.0, 7.0);
        let r = avx::_mm256_min_pd(a, b);
        let e = f64x4::new(1.0, 3.0, 5.0, 7.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_min_ps() {
        let a = f32x8::new(1.0, 4.0, 5.0, 8.0, 9.0, 12.0, 13.0, 16.0);
        let b = f32x8::new(2.0, 3.0, 6.0, 7.0, 10.0, 11.0, 14.0, 15.0);
        let r = avx::_mm256_min_ps(a, b);
        let e = f32x8::new(1.0, 3.0, 5.0, 7.0, 9.0, 11.0, 13.0, 15.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_mul_pd() {
        let a = f64x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f64x4::new(5.0, 6.0, 7.0, 8.0);
        let r = avx::_mm256_mul_pd(a, b);
        let e = f64x4::new(5.0, 12.0, 21.0, 32.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_mul_ps() {
        let a = f32x8::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0);
        let b = f32x8::new(9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let r = avx::_mm256_mul_ps(a, b);
        let e = f32x8::new(9.0, 20.0, 33.0, 48.0, 65.0, 84.0, 105.0, 128.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_addsub_pd() {
        let a = f64x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f64x4::new(5.0, 6.0, 7.0, 8.0);
        let r = avx::_mm256_addsub_pd(a, b);
        let e = f64x4::new(-4.0, 8.0, -4.0, 12.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_addsub_ps() {
        let a = f32x8::new(1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0);
        let b = f32x8::new(5.0, 6.0, 7.0, 8.0, 5.0, 6.0, 7.0, 8.0);
        let r = avx::_mm256_addsub_ps(a, b);
        let e = f32x8::new(-4.0, 8.0, -4.0, 12.0, -4.0, 8.0, -4.0, 12.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_sub_pd() {
        let a = f64x4::new(1.0, 2.0, 3.0, 4.0);
        let b = f64x4::new(5.0, 6.0, 7.0, 8.0);
        let r = avx::_mm256_sub_pd(a, b);
        let e = f64x4::new(-4.0,-4.0,-4.0,-4.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_sub_ps() {
        let a = f32x8::new(1.0, 2.0, 3.0, 4.0, -1.0, -2.0, -3.0, -4.0);
        let b = f32x8::new(5.0, 6.0, 7.0, 8.0, 3.0, 2.0, 1.0, 0.0);
        let r = avx::_mm256_sub_ps(a, b);
        let e = f32x8::new(-4.0, -4.0, -4.0, -4.0, -4.0, -4.0, -4.0, -4.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_round_pd() {
        let a = f64x4::new(1.55, 2.2, 3.99, -1.2);
        let result_closest = avx::_mm256_round_pd(a, 0b00000000);
        let result_down = avx::_mm256_round_pd(a, 0b00000001);
        let result_up = avx::_mm256_round_pd(a, 0b00000010);
        let expected_closest = f64x4::new(2.0, 2.0, 4.0, -1.0);
        let expected_down = f64x4::new(1.0, 2.0, 3.0, -2.0);
        let expected_up = f64x4::new(2.0, 3.0, 4.0, -1.0);
        assert_eq!(result_closest, expected_closest);
        assert_eq!(result_down, expected_down);
        assert_eq!(result_up, expected_up);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_floor_pd() {
        let a = f64x4::new(1.55, 2.2, 3.99, -1.2);
        let result_down = avx::_mm256_floor_pd(a);
        let expected_down = f64x4::new(1.0, 2.0, 3.0, -2.0);
        assert_eq!(result_down, expected_down);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_ceil_pd() {
        let a = f64x4::new(1.55, 2.2, 3.99, -1.2);
        let result_up = avx::_mm256_ceil_pd(a);
        let expected_up = f64x4::new(2.0, 3.0, 4.0, -1.0);
        assert_eq!(result_up, expected_up);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_round_ps() {
        let a = f32x8::new(1.55, 2.2, 3.99, -1.2, 1.55, 2.2, 3.99, -1.2);
        let result_closest = avx::_mm256_round_ps(a, 0b00000000);
        let result_down = avx::_mm256_round_ps(a, 0b00000001);
        let result_up = avx::_mm256_round_ps(a, 0b00000010);
        let expected_closest = f32x8::new(2.0, 2.0, 4.0, -1.0, 2.0, 2.0, 4.0, -1.0);
        let expected_down = f32x8::new(1.0, 2.0, 3.0, -2.0, 1.0, 2.0, 3.0, -2.0);
        let expected_up = f32x8::new(2.0, 3.0, 4.0, -1.0, 2.0, 3.0, 4.0, -1.0);
        assert_eq!(result_closest, expected_closest);
        assert_eq!(result_down, expected_down);
        assert_eq!(result_up, expected_up);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_floor_ps() {
        let a = f32x8::new(1.55, 2.2, 3.99, -1.2, 1.55, 2.2, 3.99, -1.2);
        let result_down = avx::_mm256_floor_ps(a);
        let expected_down = f32x8::new(1.0, 2.0, 3.0, -2.0, 1.0, 2.0, 3.0, -2.0);
        assert_eq!(result_down, expected_down);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_ceil_ps() {
        let a = f32x8::new(1.55, 2.2, 3.99, -1.2, 1.55, 2.2, 3.99, -1.2);
        let result_up = avx::_mm256_ceil_ps(a);
        let expected_up = f32x8::new(2.0, 3.0, 4.0, -1.0, 2.0, 3.0, 4.0, -1.0);
        assert_eq!(result_up, expected_up);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_sqrt_pd() {
        let a = f64x4::new(4.0, 9.0, 16.0, 25.0);
        let r = avx::_mm256_sqrt_pd(a, );
        let e = f64x4::new(2.0, 3.0, 4.0, 5.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_sqrt_ps() {
        let a = f32x8::new(4.0, 9.0, 16.0, 25.0, 4.0, 9.0, 16.0, 25.0);
        let r = avx::_mm256_sqrt_ps(a);
        let e = f32x8::new(2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_div_ps() {
        let a = f32x8::new(4.0, 9.0, 16.0, 25.0, 4.0, 9.0, 16.0, 25.0);
        let b = f32x8::new(4.0, 3.0, 2.0, 5.0, 8.0, 9.0, 64.0, 50.0);
        let r = avx::_mm256_div_ps(a, b);
        let e = f32x8::new(1.0, 3.0, 8.0, 5.0, 0.5, 1.0, 0.25, 0.5);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_div_pd() {
        let a = f64x4::new(4.0, 9.0, 16.0, 25.0);
        let b = f64x4::new(4.0, 3.0, 2.0, 5.0);
        let r = avx::_mm256_div_pd(a, b);
        let e = f64x4::new(1.0, 3.0, 8.0, 5.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_blend_pd() {
        let a = f64x4::new(4.0, 9.0, 16.0, 25.0);
        let b = f64x4::new(4.0, 3.0, 2.0, 5.0);
        let r = avx::_mm256_blend_pd(a, b, 0x0);
        assert_eq!(r, f64x4::new(4.0, 9.0, 16.0, 25.0));
        let r = avx::_mm256_blend_pd(a, b, 0x3);
        assert_eq!(r, f64x4::new(4.0, 3.0, 16.0, 25.0));
        let r = avx::_mm256_blend_pd(a, b, 0xF);
        assert_eq!(r, f64x4::new(4.0, 3.0, 2.0, 5.0));
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_blendv_pd() {
        let a = f64x4::new(4.0, 9.0, 16.0, 25.0);
        let b = f64x4::new(4.0, 3.0, 2.0, 5.0);
        let c = f64x4::new(0.0, 0.0, !0 as f64, !0 as f64);
        let r = avx::_mm256_blendv_pd(a, b, c);
        let e = f64x4::new(4.0, 9.0, 2.0, 5.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_blendv_ps() {
        let a = f32x8::new(4.0, 9.0, 16.0, 25.0, 4.0, 9.0, 16.0, 25.0);
        let b = f32x8::new(4.0, 3.0, 2.0, 5.0, 8.0, 9.0, 64.0, 50.0);
        let c = f32x8::new(0.0, 0.0, 0.0, 0.0, !0 as f32, !0 as f32, !0 as f32, !0 as f32);
        let r = avx::_mm256_blendv_ps(a, b, c);
        let e = f32x8::new(4.0, 9.0, 16.0, 25.0, 8.0, 9.0, 64.0, 50.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_hadd_pd() {
        let a = f64x4::new(4.0, 9.0, 16.0, 25.0);
        let b = f64x4::new(4.0, 3.0, 2.0, 5.0);
        let r = avx::_mm256_hadd_pd(a, b);
        let e = f64x4::new(13.0, 7.0, 41.0, 7.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_hadd_ps() {
        let a = f32x8::new(4.0, 9.0, 16.0, 25.0, 4.0, 9.0, 16.0, 25.0);
        let b = f32x8::new(4.0, 3.0, 2.0, 5.0, 8.0, 9.0, 64.0, 50.0);
        let r = avx::_mm256_hadd_ps(a, b);
        let e = f32x8::new(13.0, 41.0, 7.0, 7.0, 13.0, 41.0, 17.0, 114.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_hsub_pd() {
        let a = f64x4::new(4.0, 9.0, 16.0, 25.0);
        let b = f64x4::new(4.0, 3.0, 2.0, 5.0);
        let r = avx::_mm256_hsub_pd(a, b);
        let e = f64x4::new(-5.0, 1.0, -9.0, -3.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_hsub_ps() {
        let a = f32x8::new(4.0, 9.0, 16.0, 25.0, 4.0, 9.0, 16.0, 25.0);
        let b = f32x8::new(4.0, 3.0, 2.0, 5.0, 8.0, 9.0, 64.0, 50.0);
        let r = avx::_mm256_hsub_ps(a, b);
        let e = f32x8::new(-5.0, -9.0, 1.0, -3.0, -5.0, -9.0, -1.0, 14.0);
        assert_eq!(r, e);
    }

    #[simd_test = "avx"]
    unsafe fn _mm256_xor_pd() {
        let a = f64x4::new(4.0, 9.0, 16.0, 25.0);
        let b = f64x4::new(0.0, 0.0, 0.0, 0.0);
        let r = avx::_mm256_xor_pd(a, b);
        assert_eq!(r, a);
    }
}
