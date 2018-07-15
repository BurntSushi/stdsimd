#![feature(stdsimd)]
#![deny(warnings)]

#[macro_use]
extern crate stdsimd;

use stdsimd::simd::*;

fn main() {
    // check that shuffle! does not produce warnings when used in an unsafe block
    unsafe {
        let a = f32x4::new(0., 1., 2., 3.);
        let b = f32x4::new(4., 5., 6., 7.);
        let e = f32x4::new(0., 2., 4., 6.);
        assert_eq!(e, shuffle!(a, b, [0, 2, 4, 6]));
    }
}
