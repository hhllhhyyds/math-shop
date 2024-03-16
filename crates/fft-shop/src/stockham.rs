use crate::helper::{n_is_power_of_2, unit_complex};
use refined_float::{Complex, FloatTraitsForComplex};

fn fft_local<F: FloatTraitsForComplex, const SIGN: i64>(data: &mut [Complex<F>]) {
    let n = data.len();
    if n > 1 {
        assert!(n_is_power_of_2(n));
        let p = n.ilog2() as usize;

        let mut x = data.to_vec();

        let x0 = data;
        let x1: &mut [Complex<F>] = x.as_mut();
        let xs = [x0, x1];

        let mut len = n / 2;
        let mut m = 1;

        for t in 0..p {
            for j in 0..len {
                for k in 0..m {
                    let c0 = xs[t % 2][k + j * m];
                    let c1 = xs[t % 2][k + j * m + len * m];
                    xs[(t + 1) % 2][k + 2 * j * m] = c0 + c1;
                    xs[(t + 1) % 2][k + 2 * j * m + m] =
                        (c0 - c1) * unit_complex(len as u64 * 2, SIGN * (j as i64));
                }
            }
            len /= 2;
            m *= 2;
        }

        if p % 2 == 1 {
            for i in 0..n {
                xs[0][i] = xs[1][i];
            }
        }

        if SIGN == 1 {
            xs[0]
                .iter_mut()
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
