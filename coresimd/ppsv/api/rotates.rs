//! Implements integer rotates.
#![allow(unused)]

macro_rules! impl_vector_rotates {
    ($id:ident, $elem_ty:ident) => {
        impl $id {
            /// Shifts the bits of each lane to the left by the specified amount in
            /// the corresponding lane of `n`, wrapping the truncated bits to
            /// the end of the resulting integer.
            ///
            /// Please note this isn't the same operation as `<<`!. Also note it
            /// isn't equivalent to `slice::rotate_left`, it doesn't move the vector's
            /// lanes around. (that can be implemented with vector shuffles).
            #[inline]
            pub fn rotate_left(self, n: $id) -> $id {
                const LANE_WIDTH: $elem_ty = ::mem::size_of::<$elem_ty>() as $elem_ty * 8;
                // Protect against undefined behavior for over-long bit shifts
                let n = n % LANE_WIDTH;
                (self << n) | (self >> (LANE_WIDTH - n))
            }

            /// Shifts the bits of each lane to the right by the specified amount in
            /// the corresponding lane of `n`, wrapping the truncated bits to
            /// the beginning of the resulting integer.
            ///
            /// Please note this isn't the same operation as `>>`!. Also note it
            /// isn't similar to `slice::rotate_right`, it doesn't move the vector's
            /// lanes around. (that can be implemented with vector shuffles).
            #[inline]
            pub fn rotate_right(self, n: $id) -> $id {
                const LANE_WIDTH: $elem_ty = ::mem::size_of::<$elem_ty>() as $elem_ty * 8;
                // Protect against undefined behavior for over-long bit shifts
                let n = n % LANE_WIDTH;
                (self >> n) | (self << (LANE_WIDTH - n))
            }
        }
    };
}

#[cfg(test)]
macro_rules! test_vector_rotate_ops {
    ($id:ident, $elem_ty:ident) => {
        #[test]
        fn rotate_ops() {
            use coresimd::simd::$id;
            use std::mem;
            let z = $id::splat(0 as $elem_ty);
            let o = $id::splat(1 as $elem_ty);
            let t = $id::splat(2 as $elem_ty);
            let f = $id::splat(4 as $elem_ty);

            let max =
                $id::splat((mem::size_of::<$elem_ty>() * 8 - 1) as $elem_ty);

            // rotate_right
            assert_eq!(z.rotate_right(z), z);
            assert_eq!(z.rotate_right(o), z);
            assert_eq!(z.rotate_right(t), z);

            assert_eq!(o.rotate_right(z), o);
            assert_eq!(t.rotate_right(z), t);
            assert_eq!(f.rotate_right(z), f);
            assert_eq!(f.rotate_right(max), f << 1);

            assert_eq!(o.rotate_right(o), o << max);
            assert_eq!(t.rotate_right(o), o);
            assert_eq!(t.rotate_right(t), o << max);
            assert_eq!(f.rotate_right(o), t);
            assert_eq!(f.rotate_right(t), o);

            // rotate_left
            assert_eq!(z.rotate_left(z), z);
            assert_eq!(o.rotate_left(z), o);
            assert_eq!(t.rotate_left(z), t);
            assert_eq!(f.rotate_left(z), f);
            assert_eq!(f.rotate_left(max), t);

            assert_eq!(o.rotate_left(o), t);
            assert_eq!(o.rotate_left(t), f);
            assert_eq!(t.rotate_left(o), f);
        }
    };
}
