use crate::common::unit_complex;
use refined_float::{Complex, FloatTraitsForComplex};

/// Discrete Fourier Transform
pub fn dft<F: FloatTraitsForComplex>(data: &[Complex<F>]) -> Vec<Complex<F>> {
    let n = data.len();
    (0..n)
        .map(|k| {
            (0..n)
                .map(|j| data[j] * unit_complex(n as u64, -((j * k) as i64)))
                .sum()
        })
        .collect::<Vec<_>>()
}

/// Inverse dft
pub fn idft<F: FloatTraitsForComplex>(data: &[Complex<F>]) -> Vec<Complex<F>> {
    let n = data.len();
    (0..n)
        .map(|k| {
            (0..n)
                .map(|j| data[j] * unit_complex(n as u64, (j * k) as i64))
                .sum::<Complex<F>>()
                .scale(F::ONE / F::from_f64(n as f64))
        })
        .collect::<Vec<_>>()
}
