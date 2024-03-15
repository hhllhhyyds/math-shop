use crate::common::{n_is_power_of_2, unit_complex};
use refined_float::{Complex, FloatTraitsForComplex};

fn fft_local<F: FloatTraitsForComplex, const SIGN: i64>(
    data: &mut [Complex<F>],
    temp: &mut [Complex<F>],
) {
    let n = data.len();
    if n > 1 {
        (0..(n / 2)).for_each(|j| {
            temp[j] = data[j] + data[j + n / 2];
            temp[j + n / 2] =
                (data[j] - data[j + n / 2]) * unit_complex(n as u64, SIGN * (j as i64));
        });

        fft_local::<F, SIGN>(&mut temp[..(n / 2)], &mut data[..(n / 2)]);
        fft_local::<F, SIGN>(&mut temp[(n / 2)..], &mut data[(n / 2)..]);

        (0..(n / 2)).for_each(|j| {
            data[2 * j] = temp[j];
            data[2 * j + 1] = temp[j + (n / 2)];
        });
    }
}

pub fn fft<F: FloatTraitsForComplex>(data: &[Complex<F>]) -> Vec<Complex<F>> {
    assert!(!data.is_empty());

    let n = data.len();

    if n == 1 {
        data.to_vec()
    } else {
        assert!(n_is_power_of_2(n));

        let mut data = data.to_vec();
        let mut temp = vec![Complex::new(F::ZERO, F::ZERO); n];

        fft_local::<F, -1>(&mut data, &mut temp);

        data
    }
}

pub fn ifft<F: FloatTraitsForComplex>(data: &[Complex<F>]) -> Vec<Complex<F>> {
    assert!(!data.is_empty());

    let n = data.len();

    if n == 1 {
        data.to_vec()
    } else {
        assert!(n_is_power_of_2(n));

        let mut data = data.to_vec();
        let mut temp = vec![Complex::new(F::ZERO, F::ZERO); n];

        fft_local::<F, 1>(&mut data, &mut temp);

        data.iter_mut()
            .for_each(|x| *x = x.scale(F::ONE / F::from_f64(n as f64)));

        data
    }
}
