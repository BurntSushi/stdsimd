//! `core_arch`

#![cfg_attr(not(bootstrap), allow(automatic_links))]
#[macro_use]
mod macros;

#[cfg(any(target_arch = "arm", target_arch = "aarch64", doc))]
mod arm_shared;

mod simd;

#[doc = include_str!("core_arch_docs.md")]
#[stable(feature = "simd_arch", since = "1.27.0")]
pub mod arch {
    /// Platform-specific intrinsics for the `x86` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "x86", doc))]
    #[doc(cfg(target_arch = "x86"))]
    #[stable(feature = "simd_x86", since = "1.27.0")]
    pub mod x86 {
        #[stable(feature = "simd_x86", since = "1.27.0")]
        pub use crate::core_arch::x86::*;
    }

    /// Platform-specific intrinsics for the `x86_64` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "x86_64", doc))]
    #[doc(cfg(target_arch = "x86_64"))]
    #[stable(feature = "simd_x86", since = "1.27.0")]
    pub mod x86_64 {
        #[stable(feature = "simd_x86", since = "1.27.0")]
        pub use crate::core_arch::x86::*;
        #[stable(feature = "simd_x86", since = "1.27.0")]
        pub use crate::core_arch::x86_64::*;
    }

    /// Platform-specific intrinsics for the `arm` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "arm", doc))]
    #[doc(cfg(target_arch = "arm"))]
    #[unstable(feature = "stdsimd", issue = "27731")]
    pub mod arm {
        pub use crate::core_arch::arm::*;
    }

    /// Platform-specific intrinsics for the `aarch64` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "aarch64", doc))]
    #[doc(cfg(target_arch = "aarch64"))]
    #[unstable(feature = "stdsimd", issue = "27731")]
    pub mod aarch64 {
        pub use crate::core_arch::aarch64::*;
    }

    /// Platform-specific intrinsics for the `wasm32` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "wasm32", doc))]
    #[doc(cfg(target_arch = "wasm32"))]
    #[stable(feature = "simd_wasm32", since = "1.33.0")]
    pub mod wasm32 {
        #[stable(feature = "simd_wasm32", since = "1.33.0")]
        pub use crate::core_arch::wasm::*;
    }

    /// Platform-specific intrinsics for the `wasm64` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "wasm64", doc))]
    #[doc(cfg(target_arch = "wasm64"))]
    #[stable(feature = "", since = "0.0.0")]
    pub mod wasm64 {
        #[stable(feature = "", since = "0.0.0")]
        pub use crate::core_arch::wasm::*;
    }

    /// Platform-specific intrinsics for the `mips` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "mips", doc))]
    #[doc(cfg(target_arch = "mips"))]
    #[unstable(feature = "stdsimd", issue = "27731")]
    pub mod mips {
        pub use crate::core_arch::mips::*;
    }

    /// Platform-specific intrinsics for the `mips64` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "mips64", doc))]
    #[doc(cfg(target_arch = "mips64"))]
    #[unstable(feature = "stdsimd", issue = "27731")]
    pub mod mips64 {
        pub use crate::core_arch::mips::*;
    }

    /// Platform-specific intrinsics for the `PowerPC` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "powerpc", doc))]
    #[doc(cfg(target_arch = "powerpc"))]
    #[unstable(feature = "stdsimd", issue = "27731")]
    pub mod powerpc {
        pub use crate::core_arch::powerpc::*;
    }

    /// Platform-specific intrinsics for the `PowerPC64` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "powerpc64", doc))]
    #[doc(cfg(target_arch = "powerpc64"))]
    #[unstable(feature = "stdsimd", issue = "27731")]
    pub mod powerpc64 {
        pub use crate::core_arch::powerpc64::*;
    }

    /// Platform-specific intrinsics for the `NVPTX` platform.
    ///
    /// See the [module documentation](../index.html) for more details.
    #[cfg(any(target_arch = "nvptx", target_arch = "nvptx64", doc))]
    #[doc(cfg(any(target_arch = "nvptx", target_arch = "nvptx64")))]
    #[unstable(feature = "stdsimd", issue = "27731")]
    pub mod nvptx {
        pub use crate::core_arch::nvptx::*;
    }
}

mod simd_llvm;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", doc))]
#[doc(cfg(any(target_arch = "x86", target_arch = "x86_64")))]
mod x86;
#[cfg(any(target_arch = "x86_64", doc))]
#[doc(cfg(target_arch = "x86_64"))]
mod x86_64;

#[cfg(any(target_arch = "aarch64", doc))]
#[doc(cfg(target_arch = "aarch64"))]
mod aarch64;
#[cfg(any(target_arch = "arm", doc))]
#[doc(cfg(any(target_arch = "arm")))]
mod arm;

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64", doc))]
#[doc(cfg(any(target_arch = "wasm32", target_arch = "wasm64")))]
mod wasm;

#[cfg(any(target_arch = "mips", target_arch = "mips64", doc))]
#[doc(cfg(any(target_arch = "mips", target_arch = "mips64")))]
mod mips;

#[cfg(any(target_arch = "powerpc", target_arch = "powerpc64", doc))]
#[doc(cfg(any(target_arch = "powerpc", target_arch = "powerpc64")))]
mod powerpc;

#[cfg(any(target_arch = "powerpc64", doc))]
#[doc(cfg(target_arch = "powerpc64"))]
mod powerpc64;

#[cfg(any(target_arch = "nvptx", target_arch = "nvptx64", doc))]
#[doc(cfg(any(target_arch = "nvptx", target_arch = "nvptx64")))]
mod nvptx;
