use ndarray::Array1;

macro_rules! array1_f32_min {
    ($x:expr) => {
        {
            let mut min = $x[0];
            for &item in $x {
                if item < min {
                    min = item;
                }
            }

            min
        }
    }
}

fn fft(x: &Array1<f32>) {
    let x_size_f32 = x.len() as f32;

    assert_eq!(x_size_f32.log(2_f32) % 1_f32, 0_f32);

    let min = array1_f32_min!(x);
}
