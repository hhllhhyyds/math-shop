#![cfg(feature = "approx")]

use approx::AbsDiffEq;
use refined_float::{Float32, Float64};

#[test]
fn test_abs_diff_eq_f32() {
    assert!(Float32(1.0).abs_diff_eq(&Float32(1.0), Float32(1e-6)));
    assert!(!Float32(1.0).abs_diff_eq(&Float32(1.0 + 1e-5), Float32(1e-6)));
}

#[test]
fn test_abs_diff_eq_f64() {
    assert!(Float64(1.0).abs_diff_eq(&Float64(1.0), Float64(1e-6)));
    assert!(!Float64(1.0).abs_diff_eq(&Float64(1.0 + 1e-5), Float64(1e-6)));
}
