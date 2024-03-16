use approx::AbsDiffEq;
use fft_shop::{cooley_tukey, recursive_radix2_fft};
use refined_float::{Complex, Float64};

mod common;

#[test]
fn test_fft() {
    let data = common::random_complex_in_unit_square(2_i32.pow(18) as usize)
        .collect::<Vec<Complex<Float64>>>();
    let fft0 = recursive_radix2_fft::fft(&data);
    let fft1 = cooley_tukey::fft(&data);

    fft0.iter()
        .zip(fft1.iter())
        .for_each(|(x, y)| assert!(x == y))
}

#[test]
fn test_ifft() {
    let data = common::random_complex_in_unit_square(2_i32.pow(18) as usize)
        .collect::<Vec<Complex<Float64>>>();
    let fft = cooley_tukey::fft(&data);
    let ifft = cooley_tukey::ifft(&fft);

    data.iter()
        .zip(ifft.iter())
        .for_each(|(x, y)| assert!(x.abs_diff_eq(y, Float64(1e-14))))
}
