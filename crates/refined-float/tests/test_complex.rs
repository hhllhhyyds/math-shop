#[cfg(feature = "approx")]
use approx::AbsDiffEq;
use refined_float::{Complex, Float32, Float64};

#[test]
fn test_conjugate() {
    assert_eq!(
        Complex::new(Float64(0.1), Float64(0.2)).conjugate(),
        Complex::new(Float64(0.1), Float64(-0.2))
    );
    let a = Complex::new(Float64(0.1), Float64(0.2));
    assert_eq!(a.conjugate().conjugate(), a);
}

#[test]
#[cfg(feature = "approx")]
fn test_add() {
    let mut a = Complex::new(Float32(0.1), Float32(0.2));
    let b = Complex::new(Float32(0.3), Float32(0.4));
    let c = Complex::new(Float32(0.4), Float32(0.6));
    assert!(
        (a + b).abs_diff_eq(&c, Float32(f32::EPSILON)),
        "a + b = {}, c = {}",
        a + b,
        c
    );
    a += b;
    assert!(a.abs_diff_eq(&c, Float32(f32::EPSILON)));
}

#[test]
#[cfg(feature = "approx")]
fn test_sub() {
    let mut a = Complex::new(Float32(0.1), Float32(0.2));
    let b = Complex::new(Float32(0.3), Float32(0.4));
    let c = Complex::new(Float32(-0.2), Float32(-0.2));

    assert!((a - b).abs_diff_eq(&c, Float32(f32::EPSILON)));
    a -= b;
    assert!(a.abs_diff_eq(&c, Float32(f32::EPSILON)));

    let d = b + c;
    let f = d - c;
    assert!(b.abs_diff_eq(&f, Float32(f32::EPSILON)));
}

#[test]
#[cfg(feature = "approx")]
fn test_mul() {
    let mut a = Complex::new(Float32(0.1), Float32(0.2));
    let b = Complex::new(Float32(0.3), Float32(0.4));
    let c = Complex::new(Float32(-0.05), Float32(0.1));
    assert!((a * b).abs_diff_eq(&c, Float32(f32::EPSILON)));
    a *= b;
    assert!(a.abs_diff_eq(&c, Float32(f32::EPSILON)));
}

#[test]
#[cfg(feature = "approx")]
fn test_div() {
    let mut a = Complex::new(Float32(0.1), Float32(0.2));
    let b = Complex::new(Float32(0.3), Float32(0.4));
    let c = Complex::new(Float32(0.44), Float32(0.08));
    assert!((a / b).abs_diff_eq(&c, Float32(f32::EPSILON)));
    a /= b;
    assert!(
        a.abs_diff_eq(&c, Float32(f32::EPSILON)),
        "a = {}, c = {}",
        a,
        c
    );

    let d = b / c;
    let f = d * c;
    assert!(b.abs_diff_eq(&f, Float32(f32::EPSILON)));
}

#[test]
fn test_neg() {
    assert_eq!(
        -Complex::new(Float32(0.3), Float32(-0.4)),
        Complex::new(Float32(-0.3), Float32(0.4))
    );

    let a = Complex::new(Float32(0.3), Float32(-0.4));
    assert_eq!(-(-a), a);
}
