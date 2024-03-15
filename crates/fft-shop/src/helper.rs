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

/// Reverse the lgn lower bits in an usize integer
#[inline]
pub fn bit_rev(x: usize, lgn: usize) -> usize {
    let mut value = 0;
    let mut x = x;

    for i in (1..=lgn).rev() {
        value |= (1 & x) << (i - 1);
        x >>= 1;
    }

    value
}

#[cfg(test)]
mod tests {
    use super::bit_rev;

    #[test]
    fn test_bit_rev() {
        assert_eq!(bit_rev(1, 3), 4);
        assert_eq!(bit_rev(3, 3), 6);
        assert_eq!(bit_rev(4, 3), 1);
        assert_eq!(bit_rev(6, 3), 3);
    }
}
