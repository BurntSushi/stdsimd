// ARM Neon intrinsic specification.
//
// This file contains the specification for a number of
// intrinsics that allows us to generate them along with
// their test cases.
//
// To the syntax of the file - it's not very intelligently parsed!
//
// # Comments
// start with AT LEAST two, or four or more slashes  so // is a
// comment /////// is too.
//
// # Sections
// Sections start with EXACTLY three slashes followed
// by AT LEAST one space. Sections are used for two things:
//
// 1) they serve as the doc comment for the given intrinics.
// 2) they reset all variables (name, fn, etc.)
//
// # Variables
//
// name    - The prefix of the function, suffixes are auto
//           generated by the type they get passed.
//
// fn      - The function to call in rust-land.
//
// aarch64 - The intrinsic to check on aarch64 architecture.
//           If this is given but no arm intrinsic is provided,
//           the function will exclusively be generated for
//           aarch64.
//           This is used to generate both aarch64 specific and
//           shared intrinics by first only specifying th aarch64
//           variant then the arm variant.
//
// arm     - The arm v7 intrinics used to checked for arm code
//           generation. All neon functions available in arm are
//           also available in aarch64. If no aarch64 intrinic was
//           set they are assumed to be the same.
//           Intrinics ending with a `.` will have a size suffixes
//           added (such as `i8` or `i64`) that is not sign specific
//           Intrinics ending with a `.s` will have a size suffixes
//           added (such as `s8` or `u64`) that is sign specific
//
// a       - First input for tests, it gets scaled to the size of
//           the type.
//
// b       - Second input for tests, it gets scaled to the size of
//           the type.
//
// # special values
//
// TRUE - 'true' all bits are set to 1
// FALSE - 'false' all bits are set to 0
// FF - same as 'true'
// MIN - minimal value (either 0 or the lowest negative number)
// MAX - maximal value proper to overflow
//
// # validate <values>
// Validates a and b aginst the expected result of the test.
// The special values 'TRUE' and 'FALSE' can be used to
// represent the correct NEON representation of true or
// false values. It too gets scaled to the type.
//
// Validate needs to be called before generate as it sets
// up the rules for validation that get generated for each
// type.
// # generate <types>
// The generate command generates the intrinsics, it uses the
// Variables set and can be called multiple times while overwriting
// some of the variables.

/// Vector bitwise and
name = vand
fn = simd_and
arm = vand
aarch64 = and
a = 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x00
b = 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F, 0x0F
validate 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x00
b = 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
validate 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
generate int*_t, uint*_t, int64x*_t, uint64x*_t

/// Vector bitwise or (immediate, inclusive)
name = vorr
fn = simd_or
arm = vorr
aarch64 = orr
a = 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
b = 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
validate 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
generate int*_t, uint*_t, int64x*_t, uint64x*_t


/// Vector bitwise exclusive or (vector)
name = veor
fn = simd_xor
arm = veor
aarch64 = eor
a = 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
b = 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
validate 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F
generate int*_t, uint*_t, int64x*_t, uint64x*_t

////////////////////
// Absolute difference between the arguments
////////////////////

/// Absolute difference between the arguments
name = vabd
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
validate 15, 13, 11, 9, 7, 5, 3, 1, 1, 3, 5, 7, 9, 11, 13, 15

arm = vabd.s
aarch64 = sabd
link-arm = vabds._EXT_
link-aarch64 = sabd._EXT_
generate int*_t

arm = vabd.s
aarch64 = uabd
link-arm = vabdu._EXT_
link-aarch64 = uabd._EXT_
generate uint*_t

/// Absolute difference between the arguments of Floating
name = vabd
a = 1.0, 2.0, 5.0, -4.0
b = 9.0, 3.0, 2.0, 8.0
validate 8.0, 1.0, 3.0, 12.0

aarch64 = fabd
link-aarch64 = fabd._EXT_
generate float64x*_t

arm = vabd.s
aarch64 = fabd
link-arm = vabds._EXT_
link-aarch64 = fabd._EXT_
generate float*_t

////////////////////
// equality
////////////////////

/// Compare bitwise Equal (vector)
name = vceq
fn = simd_eq
a = MIN, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, MAX
b = MIN, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, MAX
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
a = MIN, MIN, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0xCC, 0x0D, 0xEE, MAX
b = MIN, MAX, 0x02, 0x04, 0x04, 0x00, 0x06, 0x08, 0x08, 0x00, 0x0A, 0x0A, 0xCC, 0xD0, 0xEE, MIN
validate TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE, TRUE, FALSE

aarch64 = cmeq
generate uint64x*_t, int64x1_t:uint64x1_t, int64x2_t:uint64x2_t, poly64x1_t:uint64x1_t, poly64x2_t:uint64x2_t

arm = vceq.
generate uint*_t, int8x8_t:uint8x8_t, int8x16_t:uint8x16_t, int16x4_t:uint16x4_t, int16x8_t:uint16x8_t, int32x2_t:uint32x2_t, int32x4_t:uint32x4_t

/// Floating-point compare equal
name = vceq
fn = simd_eq
a = 1.2, 3.4, 5.6, 7.8
b = 1.2, 3.4, 5.6, 7.8
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = fcmeq
generate float64x1_t:uint64x1_t, float64x2_t:uint64x2_t

arm = vceq.
// we are missing float16x4_t:uint16x4_t, float16x8_t:uint16x8_t
generate float32x2_t:uint32x2_t, float32x4_t:uint32x4_t

/// Signed compare bitwise equal to zero
name = vceqz
fn = simd_eq
a =  MIN, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, MAX
fixed = 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
validate FALSE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE

aarch64 = cmeq
generate int8x8_t:uint8x8_t, int8x16_t:uint8x16_t, int16x4_t:uint16x4_t, int16x8_t:uint16x8_t, int32x2_t:uint32x2_t, int32x4_t:uint32x4_t, int64x1_t:uint64x1_t, int64x2_t:uint64x2_t, poly64x1_t:uint64x1_t, poly64x2_t:uint64x2_t

/// Unsigned compare bitwise equal to zero
name = vceqz
fn = simd_eq
a = MIN, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, MAX
fixed = 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
validate TRUE, TRUE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE, FALSE

aarch64 = cmeq
generate uint*_t, uint64x*_t

/// Floating-point compare bitwise equal to zero
name = vceqz
fn = simd_eq
a =  0.0, 1.2, 3.4, 5.6
fixed = 0.0, 0.0, 0.0, 0.0
validate TRUE, FALSE, FALSE, FALSE

aarch64 = fcmeq
generate float32x2_t:uint32x2_t, float32x4_t:uint32x4_t, float64x1_t:uint64x1_t, float64x2_t:uint64x2_t

/// Signed compare bitwise Test bits nonzero
name = vtst
multi_fn = simd_and, c:in_t
multi_fn = fixed, d:in_t
multi_fn = simd_ne, c, transmute(d)
a = MIN, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, MAX
b = MIN, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, MAX
fixed = 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
validate TRUE, FALSE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = cmtst
generate int64x1_t:uint64x1_t, int64x2_t:uint64x2_t, poly64x1_t:uint64x1_t, poly64x2_t:uint64x2_t

arm = vtst
generate int8x8_t:uint8x8_t, int8x16_t:uint8x16_t, int16x4_t:uint16x4_t, int16x8_t:uint16x8_t, int32x2_t:uint32x2_t, int32x4_t:uint32x4_t

/// Unsigned compare bitwise Test bits nonzero
name = vtst
multi_fn = simd_and, c:in_t
multi_fn = fixed, d:in_t
multi_fn = simd_ne, c, transmute(d)
a = MIN, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, MAX
b = MIN, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, MAX
fixed = 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
validate FALSE, FALSE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = cmtst
generate uint64x*_t

arm = vtst
generate uint*_t

////////////////////
// Floating-point absolute value
////////////////////

/// Floating-point absolute value
name = vabs
fn = simd_fabs
a =  -0.1, -2.2, -3.3, -6.6
validate 0.1, 2.2, 3.3, 6.6
aarch64 = fabs
generate float64x1_t:float64x1_t, float64x2_t:float64x2_t

arm = vabs
generate float32x2_t:float32x2_t, float32x4_t:float32x4_t

////////////////////
// greater then
////////////////////

/// Compare signed greater than
name = vcgt
fn = simd_gt
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
aarch64 = cmgt
generate int64x1_t:uint64x1_t, int64x2_t:uint64x2_t

arm = vcgt.s
generate int8x8_t:uint8x8_t, int8x16_t:uint8x16_t, int16x4_t:uint16x4_t, int16x8_t:uint16x8_t, int32x2_t:uint32x2_t, int32x4_t:uint32x4_t

/// Compare unsigned highe
name = vcgt
fn = simd_gt
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = cmhi
generate uint64x*_t

arm = vcgt.s
generate uint*_t

/// Floating-point compare greater than
name = vcgt
fn = simd_gt
a = 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9
b = 0.1, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = fcmgt
generate float64x1_t:uint64x1_t, float64x2_t:uint64x2_t

arm = vcgt.s
// we are missing float16x4_t:uint16x4_t, float16x8_t:uint16x8_t
generate float32x2_t:uint32x2_t, float32x4_t:uint32x4_t

////////////////////
// lesser then
////////////////////

/// Compare signed less than
name = vclt
fn = simd_lt
a = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
aarch64 = cmgt
generate int64x1_t:uint64x1_t, int64x2_t:uint64x2_t

arm = vcgt.s
generate int8x8_t:uint8x8_t, int8x16_t:uint8x16_t, int16x4_t:uint16x4_t, int16x8_t:uint16x8_t, int32x2_t:uint32x2_t, int32x4_t:uint32x4_t

/// Compare unsigned less than
name = vclt
fn = simd_lt
a = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = cmhi
generate uint64x*_t

arm = vcgt.s
generate uint*_t

/// Floating-point compare less than
name = vclt
fn = simd_lt
a = 0.1, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8
b = 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = fcmgt
generate float64x1_t:uint64x1_t, float64x2_t:uint64x2_t

arm = vcgt.s
// we are missing float16x4_t:uint16x4_t, float16x8_t:uint16x8_t
generate float32x2_t:uint32x2_t, float32x4_t:uint32x4_t

////////////////////
// lesser then equals
////////////////////

/// Compare signed less than or equal
name = vcle
fn = simd_le
a = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = cmge
generate int64x1_t:uint64x1_t, int64x2_t:uint64x2_t

arm = vcge.s
generate int8x8_t:uint8x8_t, int8x16_t:uint8x16_t, int16x4_t:uint16x4_t, int16x8_t:uint16x8_t, int32x2_t:uint32x2_t, int32x4_t:uint32x4_t

/// Compare unsigned less than or equal
name = vcle
fn = simd_le
a = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = cmhs
generate uint64x*_t

arm = vcge.s
generate uint*_t

/// Floating-point compare less than or equal
name = vcle
fn = simd_le
a = 0.1, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8
b = 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE
aarch64 = fcmge
generate float64x1_t:uint64x1_t, float64x2_t:uint64x2_t

// we are missing float16x4_t:uint16x4_t, float16x8_t:uint16x8_t
arm = vcge.s
generate float32x2_t:uint32x2_t, float32x4_t:uint32x4_t

////////////////////
// greater then equals
////////////////////

/// Compare signed greater than or equal
name = vcge
fn = simd_ge
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = cmge
generate int64x1_t:uint64x1_t, int64x2_t:uint64x2_t

arm = vcge.s
generate int8x8_t:uint8x8_t, int8x16_t:uint8x16_t, int16x4_t:uint16x4_t, int16x8_t:uint16x8_t, int32x2_t:uint32x2_t, int32x4_t:uint32x4_t

/// Compare unsigned greater than or equal
name = vcge
fn = simd_ge
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = cmhs
generate uint64x*_t

arm = vcge.s
generate uint*_t

/// Floating-point compare greater than or equal
name = vcge
fn = simd_ge
a = 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8, 8.9
b = 0.1, 1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8
validate TRUE, TRUE, TRUE, TRUE, TRUE, TRUE

aarch64 = fcmge
generate float64x1_t:uint64x1_t, float64x2_t:uint64x2_t

arm = vcge.s
// we are missing float16x4_t:uint16x4_t, float16x8_t:uint16x8_t
generate float32x2_t:uint32x2_t, float32x4_t:uint32x4_t

/// Saturating subtract
name = vqsub
a = 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate 41, 40, 39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 28, 27, 26

arm = vqsub.s
aarch64 = uqsub
link-arm = vqsubu._EXT_
link-aarch64 = uqsub._EXT_
generate uint*_t

arm = vqsub.s
aarch64 = sqsub
link-arm = vqsubs._EXT_
link-aarch64 = sqsub._EXT_
generate int*_t

/// Halving add
name = vhadd
a = 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 29


arm = vhadd.s
aarch64 = uhadd
link-aarch64 = uhadd._EXT_
link-arm = vhaddu._EXT_
generate uint*_t


arm = vhadd.s
aarch64 = shadd
link-aarch64 = shadd._EXT_
link-arm = vhadds._EXT_
generate int*_t

/// Rounding halving add
name = vrhadd
a = 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate 22, 22, 23, 23, 24, 24, 25, 25, 26, 26, 27, 27, 28, 28, 29, 29

arm = vrhadd.s
aarch64 = urhadd
link-arm = vrhaddu._EXT_
link-aarch64 = urhadd._EXT_
generate uint*_t

arm = vrhadd.s
aarch64 = srhadd
link-arm = vrhadds._EXT_
link-aarch64 = srhadd._EXT_
generate int*_t

/// Saturating add
name = vqadd
a = 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42, 42
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58

arm = vqadd.s
aarch64 = uqadd
link-arm = vqaddu._EXT_
link-aarch64 = uqadd._EXT_
generate uint*_t

arm = vqadd.s
aarch64 = sqadd
link-arm = vqadds._EXT_
link-aarch64 = sqadd._EXT_
generate int*_t

/// Multiply
name = vmul
a = 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2
b = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
validate 1, 4, 3, 8, 5, 12, 7, 16, 9, 20, 11, 24, 13, 28, 15, 32
arm = vmul.
aarch64 = mul
fn = simd_mul
generate int*_t, uint*_t

/// Multiply
name = vmul
fn = simd_mul
a = 1.0, 2.0, 1.0, 2.0
b = 2.0, 3.0, 4.0, 5.0
validate 2.0, 6.0, 4.0, 10.0

aarch64 = fmul
generate float64x*_t

arm = vmul.
generate float*_t


/// Subtract
name = vsub
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2
validate 0, 0, 2, 2, 4, 4, 6, 6, 8, 8, 10, 10, 12, 12, 14, 14
arm = vsub.
aarch64 = sub
fn = simd_sub
generate int*_t, uint*_t, int64x*_t, uint64x*_t

/// Subtract
name = vsub
fn = simd_sub
a = 1.0, 4.0, 3.0, 8.0
b = 1.0, 2.0, 3.0, 4.0
validate 0.0, 2.0, 0.0, 4.0

aarch64 = fsub
generate float64x*_t

arm = vsub.
generate float*_t


/// Signed halving subtract
name = vhsub
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2, 1, 2
validate 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7

arm = vhsub.s
aarch64 = uhsub
link-arm = vhsubu._EXT_
link-aarch64 = uhsub._EXT_
generate uint*_t

arm = vhsub.s
aarch64 = shsub
link-arm = vhsubs._EXT_
link-aarch64 = shsub._EXT_
generate int*_t

/// Maximum (vector)
name = vmax
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
validate 16, 15, 14, 13, 12, 11, 10, 9, 9, 10, 11, 12, 13, 14, 15, 16

arm = vmax
aarch64 = smax
link-arm = vmaxs._EXT_
link-aarch64 = smax._EXT_
generate int*_t

arm = vmax
aarch64 = umax
link-arm = vmaxu._EXT_
link-aarch64 = umax._EXT_
generate uint*_t

/// Maximum (vector)
name = vmax
a = 1.0, -2.0, 3.0, -4.0
b = 0.0, 3.0, 2.0, 8.0
validate 1.0, 3.0, 3.0, 8.0

aarch64 = fmax
link-aarch64 = fmax._EXT_
generate float64x*_t

arm = vmax
aarch64 = fmax
link-arm = vmaxs._EXT_
link-aarch64 = fmax._EXT_
generate float*_t

/// Minimum (vector)
name = vmin
a = 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16
b = 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1
validate 1, 2, 3, 4, 5, 6, 7, 8, 8, 7, 6, 5, 4, 3, 2, 1

arm = vmin
aarch64 = smin
link-arm = vmins._EXT_
link-aarch64 = smin._EXT_
generate int*_t

arm = vmin
aarch64 = umin
link-arm = vminu._EXT_
link-aarch64 = umin._EXT_
generate uint*_t

/// Minimum (vector)
name = vmin
a = 1.0, -2.0, 3.0, -4.0
b = 0.0, 3.0, 2.0, 8.0
validate 0.0, -2.0, 2.0, -4.0

aarch64 = fmin
link-aarch64 = fmin._EXT_
generate float64x*_t

arm = vmin
aarch64 = fmin
link-arm = vmins._EXT_
link-aarch64 = fmin._EXT_
generate float*_t
