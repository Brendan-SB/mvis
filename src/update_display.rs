use num_complex::Complex;

pub fn update_display(data: &[Complex<f32>]) {
    for (i, v) in data.iter().enumerate() {
        let dist = (v.re * v.re + v.im * v.im).sqrt();
    }
}
