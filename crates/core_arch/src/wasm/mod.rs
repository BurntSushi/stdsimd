//! This module provides intrinsics specific to the WebAssembly
//! architecture. Here you'll find intrinsics specific to WebAssembly that
//! aren't otherwise surfaced somewhere in a cross-platform abstraction of
//! `std`, and you'll also find functions for leveraging WebAssembly
//! proposals such as [atomics] and [simd].
//!
//! Intrinsics in this module are modeled after the WebAssembly instructions
//! that they represent. All functions are named after the instruction they
//! intend to correspond to, and the arguments/results correspond to the type
//! signature of the instruction itself. Stable  WebAssembly instructions are
//! [documented online][instrdoc].
//!
//! [instrdoc]: https://webassembly.github.io/spec/core/valid/instructions.html
//!
//! If a proposal is not yet stable in WebAssembly itself then the functions
//! within this function may be unstable and require the nightly channel of
//! Rust to use. As the proposal itself stabilizes the intrinsics in this
//! module should stabilize as well.
//!
//! [atomics]: https://github.com/webassembly/threads
//! [simd]: https://github.com/webassembly/simd
//!
//! See the [module documentation](../index.html) for general information
//! about the `arch` module and platform intrinsics.
//!
//! ## Atomics
//!
//! The [threads proposal][atomics] for WebAssembly adds a number of
//! instructions for dealing with multithreaded programs. Most instructions
//! added in the [atomics] proposal are exposed in Rust through the
//! `std::sync::atomic` module. Some instructions, however, don't have
//! direct equivalents in Rust so they're exposed here instead.
//!
//! Note that the instructions added in the [atomics] proposal can work in
//! either a context with a shared wasm memory and without. These intrinsics
//! are always available in the standard library, but you likely won't be
//! able to use them too productively unless you recompile the standard
//! library (and all your code) with `-Ctarget-feature=+atomics`.
//!
//! It's also worth pointing out that multi-threaded WebAssembly and its
//! story in Rust is still in a somewhat "early days" phase as of the time
//! of this writing. Pieces should mostly work but it generally requires a
//! good deal of manual setup. At this time it's not as simple as "just call
//! `std::thread::spawn`", but it will hopefully get there one day!
//!
//! ## SIMD
//!
//! The [simd proposal][simd] for WebAssembly adds a new `v128` type for a
//! 128-bit SIMD register. It also adds a large array of instructions to
//! operate on the `v128` type to perform data processing. The SIMD proposal
//! at the time of this writing is in [phase 4] which means that it's in the
//! standardization phase. It's expected that once some testing on nightly
//! has happened a stabilization proposal will be made for the Rust
//! intrinsics. If you notice anything awry please feel free to [open an
//! issue](https://github.com/rust-lang/stdarch/issues/new).
//!
//! [phase 4]: https://github.com/webassembly/proposals
//!
//! Using SIMD is intended to be similar to as you would on `x86_64`, for
//! example. You'd write a function such as:
//!
//! ```rust,ignore
//! #[target_feature(enable = "simd128")]
//! unsafe fn uses_simd() {
//!     #[cfg(target_arch = "wasm32")]
//!     use std::arch::wasm32::*;
//!     #[cfg(target_arch = "wasm64")]
//!     use std::arch::wasm64::*;
//!     // ...
//! }
//! ```
//!
//! Unlike `x86_64`, however, WebAssembly does not currently have dynamic
//! detection at runtime as to whether SIMD is supported (this is one of the
//! motivators for the [conditional sections][condsections] and [feature
//! detection] proposals, but that is still pretty early days). This means
//! that your binary will either have SIMD and can only run on engines
//! which support SIMD, or it will not have SIMD at all. For compatibility
//! the standard library itself does not use any SIMD internally.
//! Determining how best to ship your WebAssembly binary with SIMD is
//! largely left up to you as it can can be pretty nuanced depending on
//! your situation.
//!
//! [condsections]: https://github.com/webassembly/conditional-sections
//! [feature detection]: https://github.com/WebAssembly/feature-detection
//!
//! To enable SIMD support at compile time you need to do one of two things:
//!
//! * First you can annotate functions with `#[target_feature(enable =
//!   "simd128")]`. This causes just that one function to have SIMD support
//!   available to it, and intrinsics will get inlined as usual in this
//!   situation.
//!
//! * Second you can compile your program with `-Ctarget-feature=+simd128`.
//!   This compilation flag blanket enables SIMD support for your entire
//!   compilation. Note that this does not include the standard library
//!   unless you [recompile the standard library][buildstd].
//!
//! [buildstd]: https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#build-std
//!
//! If you enable SIMD via either of these routes then you'll have a
//! WebAssembly binary that uses SIMD instructions, and you'll need to ship
//! that accordingly. Also note that if you call SIMD intrinsics but don't
//! enable SIMD via either of these mechanisms, you'll still have SIMD
//! generated in your program. This means to generate a binary without SIMD
//! you'll need to avoid both options above plus calling into any intrinsics
//! in this module.

#[cfg(test)]
use stdarch_test::assert_instr;

mod atomic;
pub use self::atomic::*;

mod simd128;
pub use self::simd128::*;

mod memory;
pub use self::memory::*;

/// Generates the trap instruction `UNREACHABLE`
#[cfg_attr(test, assert_instr(unreachable))]
#[inline]
#[stable(feature = "unreachable_wasm32", since = "1.37.0")]
pub unsafe fn unreachable() -> ! {
    crate::intrinsics::abort()
}
