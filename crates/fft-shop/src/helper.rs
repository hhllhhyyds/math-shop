use refined_float::{Complex, FloatTraitsForComplex};

/// Complex on unit circle, e^{2π i / n}
#[inline]
pub fn unit_complex<F: FloatTraitsForComplex>(n: u64, i: i64) -> Complex<F> {
    let i = i % n as i64;
    let n = F::from_f64(n as f64);
    let i2 = F::from_f64((2 * i) as f64);
    let pi = F::from_f64(core::f64::consts::PI);
    let angle = i2 * pi / n;
    Complex::new(angle.cos(), angle.sin())
}

#[inline]
pub fn n_is_power_of_2(n: usize) -> bool {
    n > 1 && (n & (n - 1) == 0)
}
