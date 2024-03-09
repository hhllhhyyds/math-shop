use refined_float::{Float32, Float64};

#[test]
fn test_add_f32() {
    assert_eq!(Float32(1.0), Float32(0.5 + 0.5));
}

#[test]
fn test_add_f64() {
    assert_eq!(Float64(1.0), Float64(0.5 + 0.5));
}

#[test]
fn test_sum_f32() {
    assert!(
        [1.0, 2.0, 3.0]
            .map(|x| Float32(x))
            .into_iter()
            .sum::<Float32>()
            == Float32(6.0)
    )
}

#[test]
fn test_sum_f64() {
    assert!(
        [1.0, 2.0, 3.0]
            .map(|x| Float64(x))
            .into_iter()
            .sum::<Float64>()
            == Float64(6.0)
    )
}
