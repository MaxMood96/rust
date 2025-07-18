// FIXME(f16_f128): only tested on platforms that have symbols and aren't buggy
#![cfg(target_has_reliable_f128)]

use std::f128::consts;

use super::{assert_approx_eq, assert_biteq};

// Note these tolerances make sense around zero, but not for more extreme exponents.

/// Default tolerances. Works for values that should be near precise but not exact. Roughly
/// the precision carried by `100 * 100`.
const TOL: f128 = 1e-12;

/// For operations that are near exact, usually not involving math of different
/// signs.
const TOL_PRECISE: f128 = 1e-28;

/// Smallest number
const TINY_BITS: u128 = 0x1;

/// Next smallest number
const TINY_UP_BITS: u128 = 0x2;

/// Exponent = 0b11...10, Sifnificand 0b1111..10. Min val > 0
const MAX_DOWN_BITS: u128 = 0x7ffefffffffffffffffffffffffffffe;

/// Zeroed exponent, full significant
const LARGEST_SUBNORMAL_BITS: u128 = 0x0000ffffffffffffffffffffffffffff;

/// Exponent = 0b1, zeroed significand
const SMALLEST_NORMAL_BITS: u128 = 0x00010000000000000000000000000000;

/// First pattern over the mantissa
const NAN_MASK1: u128 = 0x0000aaaaaaaaaaaaaaaaaaaaaaaaaaaa;

/// Second pattern over the mantissa
const NAN_MASK2: u128 = 0x00005555555555555555555555555555;

// FIXME(f16_f128,miri): many of these have to be disabled since miri does not yet support
// the intrinsics.

#[test]
#[cfg(any(miri, target_has_reliable_f128_math))]
fn test_abs() {
    assert_biteq!(f128::INFINITY.abs(), f128::INFINITY);
    assert_biteq!(1f128.abs(), 1f128);
    assert_biteq!(0f128.abs(), 0f128);
    assert_biteq!((-0f128).abs(), 0f128);
    assert_biteq!((-1f128).abs(), 1f128);
    assert_biteq!(f128::NEG_INFINITY.abs(), f128::INFINITY);
    assert_biteq!((1f128 / f128::NEG_INFINITY).abs(), 0f128);
    assert!(f128::NAN.abs().is_nan());
}

#[test]
fn test_is_sign_positive() {
    assert!(f128::INFINITY.is_sign_positive());
    assert!(1f128.is_sign_positive());
    assert!(0f128.is_sign_positive());
    assert!(!(-0f128).is_sign_positive());
    assert!(!(-1f128).is_sign_positive());
    assert!(!f128::NEG_INFINITY.is_sign_positive());
    assert!(!(1f128 / f128::NEG_INFINITY).is_sign_positive());
    assert!(f128::NAN.is_sign_positive());
    assert!(!(-f128::NAN).is_sign_positive());
}

#[test]
fn test_is_sign_negative() {
    assert!(!f128::INFINITY.is_sign_negative());
    assert!(!1f128.is_sign_negative());
    assert!(!0f128.is_sign_negative());
    assert!((-0f128).is_sign_negative());
    assert!((-1f128).is_sign_negative());
    assert!(f128::NEG_INFINITY.is_sign_negative());
    assert!((1f128 / f128::NEG_INFINITY).is_sign_negative());
    assert!(!f128::NAN.is_sign_negative());
    assert!((-f128::NAN).is_sign_negative());
}

#[test]
fn test_next_up() {
    let tiny = f128::from_bits(TINY_BITS);
    let tiny_up = f128::from_bits(TINY_UP_BITS);
    let max_down = f128::from_bits(MAX_DOWN_BITS);
    let largest_subnormal = f128::from_bits(LARGEST_SUBNORMAL_BITS);
    let smallest_normal = f128::from_bits(SMALLEST_NORMAL_BITS);
    assert_biteq!(f128::NEG_INFINITY.next_up(), f128::MIN);
    assert_biteq!(f128::MIN.next_up(), -max_down);
    assert_biteq!((-1.0 - f128::EPSILON).next_up(), -1.0f128);
    assert_biteq!((-smallest_normal).next_up(), -largest_subnormal);
    assert_biteq!((-tiny_up).next_up(), -tiny);
    assert_biteq!((-tiny).next_up(), -0.0f128);
    assert_biteq!((-0.0f128).next_up(), tiny);
    assert_biteq!(0.0f128.next_up(), tiny);
    assert_biteq!(tiny.next_up(), tiny_up);
    assert_biteq!(largest_subnormal.next_up(), smallest_normal);
    assert_biteq!(1.0f128.next_up(), 1.0 + f128::EPSILON);
    assert_biteq!(f128::MAX.next_up(), f128::INFINITY);
    assert_biteq!(f128::INFINITY.next_up(), f128::INFINITY);

    // Check that NaNs roundtrip.
    let nan0 = f128::NAN;
    let nan1 = f128::from_bits(f128::NAN.to_bits() ^ 0x002a_aaaa);
    let nan2 = f128::from_bits(f128::NAN.to_bits() ^ 0x0055_5555);
    assert_biteq!(nan0.next_up(), nan0);
    assert_biteq!(nan1.next_up(), nan1);
    assert_biteq!(nan2.next_up(), nan2);
}

#[test]
fn test_next_down() {
    let tiny = f128::from_bits(TINY_BITS);
    let tiny_up = f128::from_bits(TINY_UP_BITS);
    let max_down = f128::from_bits(MAX_DOWN_BITS);
    let largest_subnormal = f128::from_bits(LARGEST_SUBNORMAL_BITS);
    let smallest_normal = f128::from_bits(SMALLEST_NORMAL_BITS);
    assert_biteq!(f128::NEG_INFINITY.next_down(), f128::NEG_INFINITY);
    assert_biteq!(f128::MIN.next_down(), f128::NEG_INFINITY);
    assert_biteq!((-max_down).next_down(), f128::MIN);
    assert_biteq!((-1.0f128).next_down(), -1.0 - f128::EPSILON);
    assert_biteq!((-largest_subnormal).next_down(), -smallest_normal);
    assert_biteq!((-tiny).next_down(), -tiny_up);
    assert_biteq!((-0.0f128).next_down(), -tiny);
    assert_biteq!((0.0f128).next_down(), -tiny);
    assert_biteq!(tiny.next_down(), 0.0f128);
    assert_biteq!(tiny_up.next_down(), tiny);
    assert_biteq!(smallest_normal.next_down(), largest_subnormal);
    assert_biteq!((1.0 + f128::EPSILON).next_down(), 1.0f128);
    assert_biteq!(f128::MAX.next_down(), max_down);
    assert_biteq!(f128::INFINITY.next_down(), f128::MAX);

    // Check that NaNs roundtrip.
    let nan0 = f128::NAN;
    let nan1 = f128::from_bits(f128::NAN.to_bits() ^ 0x002a_aaaa);
    let nan2 = f128::from_bits(f128::NAN.to_bits() ^ 0x0055_5555);
    assert_biteq!(nan0.next_down(), nan0);
    assert_biteq!(nan1.next_down(), nan1);
    assert_biteq!(nan2.next_down(), nan2);
}

#[test]
#[cfg(not(miri))]
#[cfg(target_has_reliable_f128_math)]
fn test_mul_add() {
    let nan: f128 = f128::NAN;
    let inf: f128 = f128::INFINITY;
    let neg_inf: f128 = f128::NEG_INFINITY;
    assert_biteq!(12.3f128.mul_add(4.5, 6.7), 62.0500000000000000000000000000000037);
    assert_biteq!((-12.3f128).mul_add(-4.5, -6.7), 48.6500000000000000000000000000000049);
    assert_biteq!(0.0f128.mul_add(8.9, 1.2), 1.2);
    assert_biteq!(3.4f128.mul_add(-0.0, 5.6), 5.6);
    assert!(nan.mul_add(7.8, 9.0).is_nan());
    assert_biteq!(inf.mul_add(7.8, 9.0), inf);
    assert_biteq!(neg_inf.mul_add(7.8, 9.0), neg_inf);
    assert_biteq!(8.9f128.mul_add(inf, 3.2), inf);
    assert_biteq!((-3.2f128).mul_add(2.4, neg_inf), neg_inf);
}

#[test]
#[cfg(any(miri, target_has_reliable_f128_math))]
fn test_recip() {
    let nan: f128 = f128::NAN;
    let inf: f128 = f128::INFINITY;
    let neg_inf: f128 = f128::NEG_INFINITY;
    assert_biteq!(1.0f128.recip(), 1.0);
    assert_biteq!(2.0f128.recip(), 0.5);
    assert_biteq!((-0.4f128).recip(), -2.5);
    assert_biteq!(0.0f128.recip(), inf);
    assert_approx_eq!(
        f128::MAX.recip(),
        8.40525785778023376565669454330438228902076605e-4933,
        1e-4900
    );
    assert!(nan.recip().is_nan());
    assert_biteq!(inf.recip(), 0.0);
    assert_biteq!(neg_inf.recip(), -0.0);
}

#[test]
#[cfg(not(miri))]
#[cfg(target_has_reliable_f128_math)]
fn test_powi() {
    let nan: f128 = f128::NAN;
    let inf: f128 = f128::INFINITY;
    let neg_inf: f128 = f128::NEG_INFINITY;
    assert_biteq!(1.0f128.powi(1), 1.0);
    assert_approx_eq!((-3.1f128).powi(2), 9.6100000000000005506706202140776519387, TOL);
    assert_approx_eq!(5.9f128.powi(-2), 0.028727377190462507313100483690639638451, TOL);
    assert_biteq!(8.3f128.powi(0), 1.0);
    assert!(nan.powi(2).is_nan());
    assert_biteq!(inf.powi(3), inf);
    assert_biteq!(neg_inf.powi(2), inf);
}

#[test]
#[cfg(not(miri))]
#[cfg(target_has_reliable_f128_math)]
fn test_sqrt_domain() {
    assert!(f128::NAN.sqrt().is_nan());
    assert!(f128::NEG_INFINITY.sqrt().is_nan());
    assert!((-1.0f128).sqrt().is_nan());
    assert_biteq!((-0.0f128).sqrt(), -0.0);
    assert_biteq!(0.0f128.sqrt(), 0.0);
    assert_biteq!(1.0f128.sqrt(), 1.0);
    assert_biteq!(f128::INFINITY.sqrt(), f128::INFINITY);
}

#[test]
fn test_to_degrees() {
    let pi: f128 = consts::PI;
    let nan: f128 = f128::NAN;
    let inf: f128 = f128::INFINITY;
    let neg_inf: f128 = f128::NEG_INFINITY;
    assert_biteq!(0.0f128.to_degrees(), 0.0);
    assert_approx_eq!((-5.8f128).to_degrees(), -332.31552117587745090765431723855668471, TOL);
    assert_approx_eq!(pi.to_degrees(), 180.0, TOL);
    assert!(nan.to_degrees().is_nan());
    assert_biteq!(inf.to_degrees(), inf);
    assert_biteq!(neg_inf.to_degrees(), neg_inf);
    assert_biteq!(1_f128.to_degrees(), 57.2957795130823208767981548141051703);
}

#[test]
fn test_to_radians() {
    let pi: f128 = consts::PI;
    let nan: f128 = f128::NAN;
    let inf: f128 = f128::INFINITY;
    let neg_inf: f128 = f128::NEG_INFINITY;
    assert_biteq!(0.0f128.to_radians(), 0.0);
    assert_approx_eq!(154.6f128.to_radians(), 2.6982790235832334267135442069489767804, TOL);
    assert_approx_eq!((-332.31f128).to_radians(), -5.7999036373023566567593094812182763013, TOL);
    // check approx rather than exact because round trip for pi doesn't fall on an exactly
    // representable value (unlike `f32` and `f64`).
    assert_approx_eq!(180.0f128.to_radians(), pi, TOL_PRECISE);
    assert!(nan.to_radians().is_nan());
    assert_biteq!(inf.to_radians(), inf);
    assert_biteq!(neg_inf.to_radians(), neg_inf);
}

#[test]
fn test_float_bits_conv() {
    assert_eq!((1f128).to_bits(), 0x3fff0000000000000000000000000000);
    assert_eq!((12.5f128).to_bits(), 0x40029000000000000000000000000000);
    assert_eq!((1337f128).to_bits(), 0x40094e40000000000000000000000000);
    assert_eq!((-14.25f128).to_bits(), 0xc002c800000000000000000000000000);
    assert_biteq!(f128::from_bits(0x3fff0000000000000000000000000000), 1.0);
    assert_biteq!(f128::from_bits(0x40029000000000000000000000000000), 12.5);
    assert_biteq!(f128::from_bits(0x40094e40000000000000000000000000), 1337.0);
    assert_biteq!(f128::from_bits(0xc002c800000000000000000000000000), -14.25);

    // Check that NaNs roundtrip their bits regardless of signaling-ness
    // 0xA is 0b1010; 0x5 is 0b0101 -- so these two together clobbers all the mantissa bits
    let masked_nan1 = f128::NAN.to_bits() ^ NAN_MASK1;
    let masked_nan2 = f128::NAN.to_bits() ^ NAN_MASK2;
    assert!(f128::from_bits(masked_nan1).is_nan());
    assert!(f128::from_bits(masked_nan2).is_nan());

    assert_eq!(f128::from_bits(masked_nan1).to_bits(), masked_nan1);
    assert_eq!(f128::from_bits(masked_nan2).to_bits(), masked_nan2);
}

#[test]
#[should_panic]
fn test_clamp_min_greater_than_max() {
    let _ = 1.0f128.clamp(3.0, 1.0);
}

#[test]
#[should_panic]
fn test_clamp_min_is_nan() {
    let _ = 1.0f128.clamp(f128::NAN, 1.0);
}

#[test]
#[should_panic]
fn test_clamp_max_is_nan() {
    let _ = 1.0f128.clamp(3.0, f128::NAN);
}

#[test]
fn test_total_cmp() {
    use core::cmp::Ordering;

    fn quiet_bit_mask() -> u128 {
        1 << (f128::MANTISSA_DIGITS - 2)
    }

    // FIXME(f16_f128): test subnormals when powf is available
    // fn min_subnorm() -> f128 {
    //     f128::MIN_POSITIVE / f128::powf(2.0, f128::MANTISSA_DIGITS as f128 - 1.0)
    // }

    // fn max_subnorm() -> f128 {
    //     f128::MIN_POSITIVE - min_subnorm()
    // }

    fn q_nan() -> f128 {
        f128::from_bits(f128::NAN.to_bits() | quiet_bit_mask())
    }

    fn s_nan() -> f128 {
        f128::from_bits((f128::NAN.to_bits() & !quiet_bit_mask()) + 42)
    }

    assert_eq!(Ordering::Equal, (-q_nan()).total_cmp(&-q_nan()));
    assert_eq!(Ordering::Equal, (-s_nan()).total_cmp(&-s_nan()));
    assert_eq!(Ordering::Equal, (-f128::INFINITY).total_cmp(&-f128::INFINITY));
    assert_eq!(Ordering::Equal, (-f128::MAX).total_cmp(&-f128::MAX));
    assert_eq!(Ordering::Equal, (-2.5_f128).total_cmp(&-2.5));
    assert_eq!(Ordering::Equal, (-1.0_f128).total_cmp(&-1.0));
    assert_eq!(Ordering::Equal, (-1.5_f128).total_cmp(&-1.5));
    assert_eq!(Ordering::Equal, (-0.5_f128).total_cmp(&-0.5));
    assert_eq!(Ordering::Equal, (-f128::MIN_POSITIVE).total_cmp(&-f128::MIN_POSITIVE));
    // assert_eq!(Ordering::Equal, (-max_subnorm()).total_cmp(&-max_subnorm()));
    // assert_eq!(Ordering::Equal, (-min_subnorm()).total_cmp(&-min_subnorm()));
    assert_eq!(Ordering::Equal, (-0.0_f128).total_cmp(&-0.0));
    assert_eq!(Ordering::Equal, 0.0_f128.total_cmp(&0.0));
    // assert_eq!(Ordering::Equal, min_subnorm().total_cmp(&min_subnorm()));
    // assert_eq!(Ordering::Equal, max_subnorm().total_cmp(&max_subnorm()));
    assert_eq!(Ordering::Equal, f128::MIN_POSITIVE.total_cmp(&f128::MIN_POSITIVE));
    assert_eq!(Ordering::Equal, 0.5_f128.total_cmp(&0.5));
    assert_eq!(Ordering::Equal, 1.0_f128.total_cmp(&1.0));
    assert_eq!(Ordering::Equal, 1.5_f128.total_cmp(&1.5));
    assert_eq!(Ordering::Equal, 2.5_f128.total_cmp(&2.5));
    assert_eq!(Ordering::Equal, f128::MAX.total_cmp(&f128::MAX));
    assert_eq!(Ordering::Equal, f128::INFINITY.total_cmp(&f128::INFINITY));
    assert_eq!(Ordering::Equal, s_nan().total_cmp(&s_nan()));
    assert_eq!(Ordering::Equal, q_nan().total_cmp(&q_nan()));

    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-s_nan()));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-f128::INFINITY));
    assert_eq!(Ordering::Less, (-f128::INFINITY).total_cmp(&-f128::MAX));
    assert_eq!(Ordering::Less, (-f128::MAX).total_cmp(&-2.5));
    assert_eq!(Ordering::Less, (-2.5_f128).total_cmp(&-1.5));
    assert_eq!(Ordering::Less, (-1.5_f128).total_cmp(&-1.0));
    assert_eq!(Ordering::Less, (-1.0_f128).total_cmp(&-0.5));
    assert_eq!(Ordering::Less, (-0.5_f128).total_cmp(&-f128::MIN_POSITIVE));
    // assert_eq!(Ordering::Less, (-f128::MIN_POSITIVE).total_cmp(&-max_subnorm()));
    // assert_eq!(Ordering::Less, (-max_subnorm()).total_cmp(&-min_subnorm()));
    // assert_eq!(Ordering::Less, (-min_subnorm()).total_cmp(&-0.0));
    assert_eq!(Ordering::Less, (-0.0_f128).total_cmp(&0.0));
    // assert_eq!(Ordering::Less, 0.0_f128.total_cmp(&min_subnorm()));
    // assert_eq!(Ordering::Less, min_subnorm().total_cmp(&max_subnorm()));
    // assert_eq!(Ordering::Less, max_subnorm().total_cmp(&f128::MIN_POSITIVE));
    assert_eq!(Ordering::Less, f128::MIN_POSITIVE.total_cmp(&0.5));
    assert_eq!(Ordering::Less, 0.5_f128.total_cmp(&1.0));
    assert_eq!(Ordering::Less, 1.0_f128.total_cmp(&1.5));
    assert_eq!(Ordering::Less, 1.5_f128.total_cmp(&2.5));
    assert_eq!(Ordering::Less, 2.5_f128.total_cmp(&f128::MAX));
    assert_eq!(Ordering::Less, f128::MAX.total_cmp(&f128::INFINITY));
    assert_eq!(Ordering::Less, f128::INFINITY.total_cmp(&s_nan()));
    assert_eq!(Ordering::Less, s_nan().total_cmp(&q_nan()));

    assert_eq!(Ordering::Greater, (-s_nan()).total_cmp(&-q_nan()));
    assert_eq!(Ordering::Greater, (-f128::INFINITY).total_cmp(&-s_nan()));
    assert_eq!(Ordering::Greater, (-f128::MAX).total_cmp(&-f128::INFINITY));
    assert_eq!(Ordering::Greater, (-2.5_f128).total_cmp(&-f128::MAX));
    assert_eq!(Ordering::Greater, (-1.5_f128).total_cmp(&-2.5));
    assert_eq!(Ordering::Greater, (-1.0_f128).total_cmp(&-1.5));
    assert_eq!(Ordering::Greater, (-0.5_f128).total_cmp(&-1.0));
    assert_eq!(Ordering::Greater, (-f128::MIN_POSITIVE).total_cmp(&-0.5));
    // assert_eq!(Ordering::Greater, (-max_subnorm()).total_cmp(&-f128::MIN_POSITIVE));
    // assert_eq!(Ordering::Greater, (-min_subnorm()).total_cmp(&-max_subnorm()));
    // assert_eq!(Ordering::Greater, (-0.0_f128).total_cmp(&-min_subnorm()));
    assert_eq!(Ordering::Greater, 0.0_f128.total_cmp(&-0.0));
    // assert_eq!(Ordering::Greater, min_subnorm().total_cmp(&0.0));
    // assert_eq!(Ordering::Greater, max_subnorm().total_cmp(&min_subnorm()));
    // assert_eq!(Ordering::Greater, f128::MIN_POSITIVE.total_cmp(&max_subnorm()));
    assert_eq!(Ordering::Greater, 0.5_f128.total_cmp(&f128::MIN_POSITIVE));
    assert_eq!(Ordering::Greater, 1.0_f128.total_cmp(&0.5));
    assert_eq!(Ordering::Greater, 1.5_f128.total_cmp(&1.0));
    assert_eq!(Ordering::Greater, 2.5_f128.total_cmp(&1.5));
    assert_eq!(Ordering::Greater, f128::MAX.total_cmp(&2.5));
    assert_eq!(Ordering::Greater, f128::INFINITY.total_cmp(&f128::MAX));
    assert_eq!(Ordering::Greater, s_nan().total_cmp(&f128::INFINITY));
    assert_eq!(Ordering::Greater, q_nan().total_cmp(&s_nan()));

    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-s_nan()));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-f128::INFINITY));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-f128::MAX));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-2.5));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-1.5));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-1.0));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-0.5));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-f128::MIN_POSITIVE));
    // assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-max_subnorm()));
    // assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-min_subnorm()));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&-0.0));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&0.0));
    // assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&min_subnorm()));
    // assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&max_subnorm()));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&f128::MIN_POSITIVE));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&0.5));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&1.0));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&1.5));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&2.5));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&f128::MAX));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&f128::INFINITY));
    assert_eq!(Ordering::Less, (-q_nan()).total_cmp(&s_nan()));

    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-f128::INFINITY));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-f128::MAX));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-2.5));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-1.5));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-1.0));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-0.5));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-f128::MIN_POSITIVE));
    // assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-max_subnorm()));
    // assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-min_subnorm()));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&-0.0));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&0.0));
    // assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&min_subnorm()));
    // assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&max_subnorm()));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&f128::MIN_POSITIVE));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&0.5));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&1.0));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&1.5));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&2.5));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&f128::MAX));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&f128::INFINITY));
    assert_eq!(Ordering::Less, (-s_nan()).total_cmp(&s_nan()));
}

#[test]
fn test_algebraic() {
    let a: f128 = 123.0;
    let b: f128 = 456.0;

    // Check that individual operations match their primitive counterparts.
    //
    // This is a check of current implementations and does NOT imply any form of
    // guarantee about future behavior. The compiler reserves the right to make
    // these operations inexact matches in the future.
    let eps = if cfg!(miri) { 1e-6 } else { 0.0 };

    assert_approx_eq!(a.algebraic_add(b), a + b, eps);
    assert_approx_eq!(a.algebraic_sub(b), a - b, eps);
    assert_approx_eq!(a.algebraic_mul(b), a * b, eps);
    assert_approx_eq!(a.algebraic_div(b), a / b, eps);
    assert_approx_eq!(a.algebraic_rem(b), a % b, eps);
}

#[test]
fn test_from() {
    assert_biteq!(f128::from(false), 0.0);
    assert_biteq!(f128::from(true), 1.0);
    assert_biteq!(f128::from(u8::MIN), 0.0);
    assert_biteq!(f128::from(42_u8), 42.0);
    assert_biteq!(f128::from(u8::MAX), 255.0);
    assert_biteq!(f128::from(i8::MIN), -128.0);
    assert_biteq!(f128::from(42_i8), 42.0);
    assert_biteq!(f128::from(i8::MAX), 127.0);
    assert_biteq!(f128::from(u16::MIN), 0.0);
    assert_biteq!(f128::from(42_u16), 42.0);
    assert_biteq!(f128::from(u16::MAX), 65535.0);
    assert_biteq!(f128::from(i16::MIN), -32768.0);
    assert_biteq!(f128::from(42_i16), 42.0);
    assert_biteq!(f128::from(i16::MAX), 32767.0);
    assert_biteq!(f128::from(u32::MIN), 0.0);
    assert_biteq!(f128::from(42_u32), 42.0);
    assert_biteq!(f128::from(u32::MAX), 4294967295.0);
    assert_biteq!(f128::from(i32::MIN), -2147483648.0);
    assert_biteq!(f128::from(42_i32), 42.0);
    assert_biteq!(f128::from(i32::MAX), 2147483647.0);
    // FIXME(f16_f128): Uncomment these tests once the From<{u64,i64}> impls are added.
    // assert_eq!(f128::from(u64::MIN), 0.0);
    // assert_eq!(f128::from(42_u64), 42.0);
    // assert_eq!(f128::from(u64::MAX), 18446744073709551615.0);
    // assert_eq!(f128::from(i64::MIN), -9223372036854775808.0);
    // assert_eq!(f128::from(42_i64), 42.0);
    // assert_eq!(f128::from(i64::MAX), 9223372036854775807.0);
}
