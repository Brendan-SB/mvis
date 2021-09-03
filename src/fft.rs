use ndarray::{s, Array1, NewAxis};
use num_complex::Complex;
use std::f32::consts::PI;

pub fn fft(x: Array1<f32>) {
    assert_eq!(x.len() as f32, 0_f32, "Value must be a power of 2.");

    let min = usize::min(x.len(), 2_usize);
    let mut n = Vec::<Complex<f32>>::new();

    for i in 0_usize..min {
        n.push(Complex::new(i as f32, 0_f32));
    }

    let k = x.slice(s![.., NewAxis]);

    let exp = Complex::new(0_f32, -2_f32) * PI * Array1::from_vec(n) * k / Complex::new(min as f32, 0_f32);
}
