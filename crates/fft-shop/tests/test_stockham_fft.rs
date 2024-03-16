use approx::AbsDiffEq;
use fft_shop::{cooley_tukey, stockham};
use refined_float::{Complex, Float64};

mod common;

#[test]
fn test_fft() {
    let data = common::random_complex_in_unit_square(2_i32.pow(17) as usize)
        .collect::<Vec<Complex<Float64>>>();
    let fft0 = cooley_tukey::fft(&data);
    let fft1 = stockham::fft(&data);

    fft0.iter()
        .zip(fft1.iter())
        .for_each(|(x, y)| assert!(x == y));

    let data = common::random_complex_in_unit_square(2_i32.pow(18) as usize)
        .collect::<Vec<Complex<Float64>>>();
    let fft0 = cooley_tukey::fft(&data);
    let fft1 = stockham::fft(&data);

    fft0.iter()
        .zip(fft1.iter())
        .for_each(|(x, y)| assert!(x == y))
}

#[test]
fn test_ifft() {
    let data = common::random_complex_in_unit_square(2_i32.pow(17) as usize)
        .collect::<Vec<Complex<Float64>>>();
    let fft = stockham::fft(&data);
    let ifft = stockham::ifft(&fft);

    data.iter()
        .zip(ifft.iter())
        .for_each(|(x, y)| assert!(x.abs_diff_eq(y, Float64(1e-14))));

    let data = common::random_complex_in_unit_square(2_i32.pow(18) as usize)
        .collect::<Vec<Complex<Float64>>>();
    let fft = stockham::fft(&data);
    let ifft = stockham::ifft(&fft);

    data.iter()
        .zip(ifft.iter())
        .for_each(|(x, y)| assert!(x.abs_diff_eq(y, Float64(1e-14))));

    let fft_c = cooley_tukey::fft(&data);
    let ifft_c = cooley_tukey::ifft(&fft);

    fft.iter()
        .zip(fft_c.iter())
        .for_each(|(x, y)| assert!(x == y));

    ifft.iter()
        .zip(ifft_c.iter())
        .for_each(|(x, y)| assert!(x == y))
}
