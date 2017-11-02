//! This module implements minimal run-time feature detection for x86.
//!
//! The features are detected using the `detect_features` function below.
//! This function uses the CPUID instruction to read the feature flags from the
//! CPU and encodes them in an `usize` where each bit position represents
//! whether a feature is available (bit is set) or unavaiable (bit is cleared).
//!
//! The enum `__Feature` is used to map bit positions to feature names, and the
//! the `__unstable_detect_feature!` macro is used to map string literals (e.g.
//! "avx") to these bit positions (e.g. `__Feature::avx`).
//!
//!
//! The run-time feature detection is performed by the
//! `__unstable_detect_feature(__Feature) -> bool` function. On its first call,
//! this functions queries the CPU for the available features and stores them
//! in a global `AtomicUsize` variable. The query is performed by just checking
//! whether the feature bit in this global variable is set or cleared.
use std::sync::atomic::{AtomicUsize, Ordering};

/// This macro maps the string-literal feature names to values of the
/// `__Feature` enum at compile-time. The feature names used are the same as
/// those of rustc `target_feature` and `cfg_target_feature` features.
///
/// PLESE: do not use this, it is an implementation detail subjected to change.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __unstable_detect_feature {
    ("sse") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::sse{})  };
    ("sse2") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::sse2{})
    };
    ("sse3") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::sse3{})
    };
    ("ssse3") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::ssse3{})
    };
    ("sse4.1") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::sse4_1{})
    };
    ("sse4.2") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::sse4_2{})
    };
    ("avx") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::avx{})
    };
    ("avx2") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::avx2{})
    };
    ("fma") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::fma{})
    };
    ("bmi") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::bmi{})
    };
    ("bmi2") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::bmi2{})
    };
    ("abm") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::abm{})
    };
    ("lzcnt") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::abm{})
    };
    ("tbm") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::tbm{})
    };
    ("popcnt") => {
        $crate::vendor::__unstable_detect_feature(
            $crate::vendor::__Feature::popcnt{})
    };
    ($t:tt) => {
        compile_error!(concat!("unknown target feature: ", $t))
    };
}

/// X86 CPU Feature enum. Each variant denotes a position in a bitset for a
/// particular feature.
///
/// PLEASE: do not use this, it is an implementation detail subject to change.
#[doc(hidden)]
#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum __Feature {
    /// SSE (Streaming SIMD Extensions)
    sse,
    /// SSE2 (Streaming SIMD Extensions 2)
    sse2,
    /// SSE3 (Streaming SIMD Extensions 3)
    sse3,
    /// SSSE3 (Supplemental Streaming SIMD Extensions 3)
    ssse3,
    /// SSE4.1 (Streaming SIMD Extensions 4.1)
    sse4_1,
    /// SSE4.2 (Streaming SIMD Extensions 4.2)
    sse4_2,
    /// AVX (Advanced Vector Extensions)
    avx,
    /// AVX2 (Advanced Vector Extensions 2)
    avx2,
    /// FMA (Fused Multiply Add)
    fma,
    /// BMI1 (Bit Manipulation Instructions 1)
    bmi,
    /// BMI1 (Bit Manipulation Instructions 2)
    bmi2,
    /// ABM (Advanced Bit Manipulation) on AMD / LZCNT (Leading Zero
    /// Count) on Intel
    abm,
    /// TBM (Trailing Bit Manipulation)
    tbm,
    /// POPCNT (Population Count)
    popcnt,

    #[doc(hidden)] __NonExhaustive,
}

/// Sets the `bit`-th bit of `x`.
fn set_bit(x: usize, bit: u32) -> usize {
    debug_assert!(32 > bit);
    x | 1 << bit
}

/// Tests the `bit`-th bit of `x`.
fn test_bit(x: usize, bit: u32) -> bool {
    debug_assert!(32 > bit);
    x & (1 << bit) != 0
}

/// Run-time feature detection on x86 works by using the CPUID instruction.
///
/// The [CPUID Wikipedia page][wiki_cpuid] contains
/// all the information about which flags to set to query which values, and in
/// which registers these are reported.
///
/// The definitive references are:
/// - [Intel 64 and IA-32 Architectures Software Developer's Manual Volume 2:
///   Instruction Set Reference, A-Z][intel64_ref].
/// - [AMD64 Architecture Programmer's Manual, Volume 3: General-Purpose and
///   System Instructions][amd64_ref].
///
/// [wiki_cpuid]: https://en.wikipedia.org/wiki/CPUID
/// [intel64_ref]: http://www.intel.de/content/dam/www/public/us/en/documents/manuals/64-ia-32-architectures-software-developer-instruction-set-reference-manual-325383.pdf
/// [amd64_ref]: http://support.amd.com/TechDocs/24594.pdf
fn detect_features() -> usize {
    use super::cpuid::{__cpuid, has_cpuid, CpuidResult};
    let mut value: usize = 0;

    // If the x86 CPU does not support the CPUID instruction then it is too
    // old to support any of the currently-detectable features.
    if !has_cpuid() {
        return value;
    }

    // Calling `cpuid` from here on is safe because the CPU has the `cpuid`
    // instruction.

    // 1. EAX=1, ECX=0: Queries "Processor Info and Feature Bits";
    // Contains information about most x86 features.
    let CpuidResult {
        ecx: proc_info_ecx,
        edx: proc_info_edx,
        ..
    } = unsafe { __cpuid(0x0000_0001_u32) };

    // 2. EAX=7, ECX=0: Queries "Extended Features";
    // Contains information about bmi,bmi2, and avx2 support.
    let CpuidResult {
        ebx: extended_features_ebx,
        ..
    } = unsafe { __cpuid(0x0000_0007_u32) };

    let proc_info_ecx = proc_info_ecx as usize;
    let proc_info_edx = proc_info_edx as usize;

    let extended_features_ebx = extended_features_ebx as usize;

    if test_bit(extended_features_ebx, 3) {
        value = set_bit(value, __Feature::bmi as u32);
    }
    if test_bit(extended_features_ebx, 8) {
        value = set_bit(value, __Feature::bmi2 as u32);
    }

    if test_bit(proc_info_ecx, 0) {
        value = set_bit(value, __Feature::sse3 as u32);
    }
    if test_bit(proc_info_ecx, 5) {
        value = set_bit(value, __Feature::abm as u32);
    }
    if test_bit(proc_info_ecx, 9) {
        value = set_bit(value, __Feature::ssse3 as u32);
    }
    if test_bit(proc_info_ecx, 12) {
        value = set_bit(value, __Feature::fma as u32);
    }
    if test_bit(proc_info_ecx, 19) {
        value = set_bit(value, __Feature::sse4_1 as u32);
    }
    if test_bit(proc_info_ecx, 20) {
        value = set_bit(value, __Feature::sse4_2 as u32);
    }
    if test_bit(proc_info_ecx, 21) {
        value = set_bit(value, __Feature::tbm as u32);
    }
    if test_bit(proc_info_ecx, 23) {
        value = set_bit(value, __Feature::popcnt as u32);
    }

    if test_bit(proc_info_edx, 25) {
        value = set_bit(value, __Feature::sse as u32);
    }
    if test_bit(proc_info_edx, 26) {
        value = set_bit(value, __Feature::sse2 as u32);
    }

    // ECX[26] detects XSAVE and ECX[27] detects OSXSAVE, that is, whether the
    // OS is AVX enabled and supports saving the state of the AVX/AVX2 vector
    // registers on context-switches, see:
    //
    // - https://software.intel.com/en-us/blogs/2011/04/14/is-avx-enabled
    // - https://hg.mozilla.
    // org/mozilla-central/file/64bab5cbb9b6/mozglue/build/SSE.cpp#l190
    //
    if test_bit(proc_info_ecx, 26) && test_bit(proc_info_ecx, 27) {
        use super::xsave::_xgetbv;

        // This is safe because on x86 `xgetbv` is always available.
        if unsafe { _xgetbv(0) } & 6 == 6 {
            if test_bit(proc_info_ecx, 28) {
                value = set_bit(value, __Feature::avx as u32);
            }
            if test_bit(extended_features_ebx, 5) {
                value = set_bit(value, __Feature::avx2 as u32);
            }
        }
    }

    value
}

/// This global variable is a bitset used to cache the features supported by
/// the CPU.
static FEATURES: AtomicUsize = AtomicUsize::new(::std::usize::MAX);

/// Performs run-time feature detection.
///
/// On its first invocation, it detects the CPU features and caches them
/// in the `FEATURES` global variable as an `AtomicUsize`.
///
/// It uses the `__Feature` variant to index into this variable as a bitset. If
/// the bit is set, the feature is enabled, and otherwise it is disabled.
///
/// PLEASE: do not use this, it is an implementation detail subject to change.
#[doc(hidden)]
pub fn __unstable_detect_feature(x: __Feature) -> bool {
    if FEATURES.load(Ordering::Relaxed) == ::std::usize::MAX {
        FEATURES.store(detect_features(), Ordering::Relaxed);
    }
    test_bit(FEATURES.load(Ordering::Relaxed), x as u32)
}

#[cfg(test)]
mod tests {
    #[test]
    fn runtime_detection_x86_nocapture() {
        println!("sse: {:?}", cfg_feature_enabled!("sse"));
        println!("sse2: {:?}", cfg_feature_enabled!("sse2"));
        println!("sse3: {:?}", cfg_feature_enabled!("sse3"));
        println!("ssse3: {:?}", cfg_feature_enabled!("ssse3"));
        println!("sse4.1: {:?}", cfg_feature_enabled!("sse4.1"));
        println!("sse4.2: {:?}", cfg_feature_enabled!("sse4.2"));
        println!("avx: {:?}", cfg_feature_enabled!("avx"));
        println!("avx2: {:?}", cfg_feature_enabled!("avx2"));
        println!("abm: {:?}", cfg_feature_enabled!("abm"));
        println!("bmi: {:?}", cfg_feature_enabled!("bmi"));
        println!("bmi2: {:?}", cfg_feature_enabled!("bmi2"));
        println!("tbm: {:?}", cfg_feature_enabled!("tbm"));
        println!("popcnt: {:?}", cfg_feature_enabled!("popcnt"));
        println!("lzcnt: {:?}", cfg_feature_enabled!("lzcnt"));
        println!("fma: {:?}", cfg_feature_enabled!("fma"));
    }
}
