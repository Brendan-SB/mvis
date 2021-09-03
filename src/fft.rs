#[macro_export]
macro_rules! fft {
    ($x:expr) => {};
}

use ndarray::{s, Array, Array1, NewAxis};
use num_complex::Complex;
use std::f32::consts::PI;

pub fn fft(x: Array1<f32>) {
    let x_size = x.len() as f32;

    assert_eq!(
        x_size.log(2_f32) % 1_f32,
        0_f32,
        "Value must be a power of 2."
    );

    let n = Array::range(0_f32, f32::min(x_size, 2_f32), 1_f32)
        .into_iter()
        .map(|x| Complex::new(x, 0_f32))
        .collect::<Array1<Complex<f32>>>();

    let k = x.slice(s![.., NewAxis]);

    let exp = Complex::new(0_f32, -2_f32) * PI * n;
}
