use core::ops::Range;
use rand::{thread_rng, Rng};
use refined_float::{Complex, FloatTraitsForComplex};

pub fn random_complex_iter<F: FloatTraitsForComplex>(
    range_real: Range<f64>,
    range_imag: Range<f64>,
    num: usize,
) -> impl Iterator<Item = Complex<F>> {
    let mut rng = thread_rng();
    (0..num).map(move |_| {
        Complex::new(
            F::from_f64(rng.gen_range(range_real.clone())),
            F::from_f64(rng.gen_range(range_imag.clone())),
        )
    })
}

pub fn random_complex_in_unit_square<F: FloatTraitsForComplex>(
    num: usize,
) -> impl Iterator<Item = Complex<F>> {
    random_complex_iter(-1.0..1.0, -1.0..1.0, num)
}
