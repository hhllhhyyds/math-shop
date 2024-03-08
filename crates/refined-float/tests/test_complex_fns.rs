use refined_float::{Float32, Float64};

#[test]
fn test_sin_f32() {
    assert_eq!(Float32(1.0), Float32(std::f32::consts::FRAC_PI_2).sin());
}

#[test]
fn test_sin_f64() {
    assert_eq!(Float64(1.0), Float64(std::f64::consts::FRAC_PI_2).sin());
}
