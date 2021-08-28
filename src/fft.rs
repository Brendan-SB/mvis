use ndarray::{s, Array, Array1, NewAxis};

fn fft(x: &Array1<f32>) {
    let x_size = x.len() as f32;

    assert_eq!(
        x_size.log(2_f32) % 1_f32,
        0_f32,
        "Value must be a power of 2."
    );

    let x_arranged = Array::range(0_f32, 1_f32, f32::min(x_size, 2_f32));

    let k = x.slice(s![.., NewAxis]);
}
