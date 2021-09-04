use num_complex::Complex;
use std::f64::consts::PI;

pub fn fft(mut input: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    const I: Complex<f64> = Complex {
        re: 0_f64,
        im: 1_f64,
    };

    fn fft_inner(buf_a: &mut [Complex<f64>], buf_b: &mut [Complex<f64>], n: usize, step: usize) {
        if step >= n {
            return;
        }

        fft_inner(buf_b, buf_a, n, step * 2);
        fft_inner(&mut buf_b[step..], &mut buf_a[step..], n, step * 2);

        let (left, right) = buf_a.split_at_mut(n / 2);

        for i in (0..n).step_by(step * 2) {
            let t = (-I * PI * (i as f64) / (n as f64)).exp() * buf_b[i + step];

            let i_over_2 = i / 2;

            left[i_over_2] = buf_b[i] + t;
            right[i_over_2] = buf_b[i] - t;
        }
    }

    input.append(&mut vec![
        Complex {
            re: 0_f64,
            im: 0_f64,
        };
        input.len().next_power_of_two() - input.len()
    ]);

    input
}
