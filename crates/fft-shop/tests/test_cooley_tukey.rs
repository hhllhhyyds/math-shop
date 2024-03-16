use approx::AbsDiffEq;
use fft_shop::{cooley_tukey, recursive_radix2_fft};
use rand::{thread_rng, Rng};
use refined_float::{Complex, Float64};

#[test]
fn test_fft() {
    let mut rng = thread_rng();
    let data = (0..1024 * 256)
        .map(|_| {
            Complex::new(
                Float64(rng.gen_range(-1.0..1.0)),
                Float64(rng.gen_range(-1.0..1.0)),
            )
        })
        .collect::<Vec<_>>();

    let fft0 = recursive_radix2_fft::fft(&data);
    let fft1 = cooley_tukey::fft(&data);

    for (x, y) in fft0.iter().zip(fft1.iter()) {
        assert!(x == y);
    }
}

#[test]
fn test_ifft() {
    let mut rng = thread_rng();
    let data = (0..1024 * 512)
        .map(|_| {
            Complex::new(
                Float64(rng.gen_range(-1.0..1.0)),
                Float64(rng.gen_range(-1.0..1.0)),
            )
        })
        .collect::<Vec<_>>();

    let fft = cooley_tukey::fft(&data);
    let ifft = cooley_tukey::ifft(&fft);

    for (x, y) in data.iter().zip(ifft.iter()) {
        assert!(x.abs_diff_eq(y, Float64(1e-14)));
    }
}