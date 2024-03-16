use crate::helper::{bit_rev, n_is_power_of_2, unit_complex};
use refined_float::{Complex, FloatTraitsForComplex};

fn fft_local<F: FloatTraitsForComplex, const SIGN: i64>(data: &mut [Complex<F>]) {
    let n = data.len();
    if n > 1 {
        assert!(n_is_power_of_2(n));
        let m = n.ilog2() as usize;

        let mut x = data.to_vec();
        let mut len = n;

        for _ in 0..m {
            len /= 2;
            for j in 0..len {
                for i in (j..n).step_by(len * 2) {
                    let t0 = x[i + len];
                    let t1 = x[i] - t0;
                    x[i] += t0;
                    x[i + len] = t1 * unit_complex(len as u64 * 2, SIGN * (j as i64));
                }
            }
        }

        for i in 0..n {
            data[i] = x[bit_rev(i, m)];
        }

        if SIGN == 1 {
            data.iter_mut()
                .for_each(|x| *x = x.scale(F::ONE / F::from_f64(n as f64)));
        }
    }
}

pub fn fft<F: FloatTraitsForComplex>(data: &[Complex<F>]) -> Vec<Complex<F>> {
    let mut data = data.to_vec();
    fft_local::<F, -1>(&mut data);
    data
}

pub fn ifft<F: FloatTraitsForComplex>(data: &[Complex<F>]) -> Vec<Complex<F>> {
    let mut data = data.to_vec();
    fft_local::<F, 1>(&mut data);
    data
}
