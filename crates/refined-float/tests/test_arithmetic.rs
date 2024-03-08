use refined_float::{Float32, Float64};

#[test]
fn test_add_f32() {
    assert_eq!(Float32(1.0), Float32(0.5 + 0.5));
}

#[test]
fn test_add_f64() {
    assert_eq!(Float64(1.0), Float64(0.5 + 0.5));
}
