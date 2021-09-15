use kira::sound::Sound;
use num_complex::Complex;
use ringbuf::Producer;
use std::f32::consts::PI;

fn fft(input: &[Complex<f32>]) -> Vec<Complex<f32>> {
    fn fft_inner(buf_a: &mut [Complex<f32>], buf_b: &mut [Complex<f32>], n: usize, step: usize) {
        static I: Complex<f32> = Complex { re: 0.0, im: 1.0 };

        if step >= n {
            return;
        }

        fft_inner(buf_b, buf_a, n, step * 2);
        fft_inner(&mut buf_b[step..], &mut buf_a[step..], n, step * 2);

        let (left, right) = buf_a.split_at_mut(n / 2);

        for i in (0..n).step_by(step * 2) {
            let t = (-I * PI * (i as f32) / (n as f32)).exp() * buf_b[i + step];
            left[i / 2] = buf_b[i] + t;
            right[i / 2] = buf_b[i] - t;
        }
    }

    let mut buf_a = input.to_vec();

    let n_orig = buf_a.len();
    let n = n_orig.next_power_of_two();

    buf_a.append(&mut vec![
        Complex {
            re: 0_f32,
            im: 0_f32
        };
        n - n_orig
    ]);

    let mut buf_b = buf_a.clone();

    fft_inner(&mut buf_a, &mut buf_b, n, 1);

    buf_a
}

pub fn fft_thread(sound: &Sound, duration: i64, producer: &mut Producer<Vec<Complex<f32>>>) {
    for (start, end) in (0..=duration - 20)
        .step_by(20)
        .zip((20..=duration).step_by(20))
    {
        let mut buffer = Vec::new();

        for i in start..=end {
            let frame = sound.get_frame_at_position(i as f64);

            buffer.push(Complex::new((frame.right + frame.left) / 2_f32, 0_f32));
        }

        producer.push(fft(&buffer)).unwrap();
    }
}
