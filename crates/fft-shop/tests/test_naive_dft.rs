use approx::AbsDiffEq;
use fft_shop::naive_dft;
use rand::{thread_rng, Rng};
use refined_float::{Complex, Float32, Float64};

#[test]
fn test_naive_dft() {
    let data = (1..=10)
        .map(|x| Complex::new(Float32(x as f32), Float32::ZERO))
        .collect::<Vec<_>>();

    let dft_result = naive_dft::dft(&data);
    let result_from_python: [Complex<Float32>; 10] = [
        (Float32(55.), Float32(0.)).into(),
        (Float32(-5.), Float32(1.53884177e1)).into(),
        (Float32(-5.), Float32(6.8819096)).into(),
        (Float32(-5.), Float32(3.63271232)).into(),
        (Float32(-5.), Float32(1.62459848)).into(),
        (Float32(-5.), Float32(-1.55431223e-15)).into(),
        (Float32(-5.), Float32(-1.62459848)).into(),
        (Float32(-5.), Float32(-3.63271232)).into(),
        (Float32(-5.), Float32(-6.8819096)).into(),
        (Float32(-5.), Float32(-1.53884177e1)).into(),
    ];

    dft_result
        .iter()
        .zip(result_from_python.iter())
        .for_each(|(a, b)| {
            assert!(a.abs_diff_eq(&b, Float32(1e-3)));
        });
}

#[test]
fn test_inverse_dft() {
    let mut rng = thread_rng();
    let data = (0..1000)
        .map(|_| {
            Complex::new(
                Float64(rng.gen_range(-1.0..1.0)),
                Float64(rng.gen_range(-1.0..1.0)),
            )
        })
        .collect::<Vec<_>>();
    let dft = naive_dft::dft(&data);
    let idft = naive_dft::idft(&dft);

    data.iter()
        .zip(idft.iter())
        .for_each(|(x, y)| assert!(x.abs_diff_eq(&y, Float64(1e-12))))
}
