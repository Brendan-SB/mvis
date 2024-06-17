use crate::{config::Config, display::Display};
use kira::{
    manager::{backend::DefaultBackend, AudioManager},
    sound::static_sound::{PlaybackState, StaticSoundData, StaticSoundSettings},
};
use num_complex::Complex;
use std::{
    f64::consts::PI,
    io,
    io::Write,
    thread,
    time::{Duration, Instant},
};

pub fn play(config: &Config, audio_file_path: &String) -> anyhow::Result<()> {
    let mut manager = AudioManager::<DefaultBackend>::new(Default::default())?;

    print!("Loading sound...");

    io::stdout().flush()?;

    let sound = StaticSoundData::from_file(
        audio_file_path,
        StaticSoundSettings::new().volume(config.volume),
    )?;

    println!("Complete.");

    let mut display = Display::new(config)?;
    let handle = manager.play(sound.clone())?;

    while handle.state() != PlaybackState::Stopped {
        let frame_start = Instant::now();
        let index = handle.position() * sound.sample_rate as f64;
        let start = (index.floor() as i64 % sound.frames.len() as i64) as usize;
        let end = ((index + sound.sample_rate as f64 * config.detail).ceil() as i64
            % sound.frames.len() as i64) as usize;

        if start >= end {
            break;
        }

        let mut buffer = Vec::new();

        for i in start..end {
            let frame = sound.frames[i];

            buffer.push(Complex {
                re: (frame.left + frame.right) as f64 / 2_f64,
                im: 0_f64,
            });
        }

        fft(&mut buffer);

        display.update(&buffer)?;

        if let Some(target_fps) = config.fps {
            let frame_end = Instant::now();
            let time = frame_end.duration_since(frame_start).as_secs_f64();

            if 1_f64 / time > target_fps as f64 {
                thread::sleep(Duration::from_secs_f64(1_f64 / target_fps as f64 - time));
            }
        }
    }

    Ok(())
}

pub fn fft(buf_a: &mut Vec<Complex<f64>>) {
    fn fft_inner(buf_a: &mut [Complex<f64>], buf_b: &mut [Complex<f64>], n: usize, step: usize) {
        static I: Complex<f64> = Complex {
            re: 0_f64,
            im: 1_f64,
        };

        if step >= n {
            return;
        }

        fft_inner(buf_b, buf_a, n, step * 2);
        fft_inner(&mut buf_b[step..], &mut buf_a[step..], n, step * 2);

        let (left, right) = buf_a.split_at_mut(n / 2);

        for i in (0..n).step_by(step * 2) {
            let t = (-I * PI * (i as f64) / (n as f64)).exp() * buf_b[i + step];

            left[i / 2] = buf_b[i] + t;
            right[i / 2] = buf_b[i] - t;
        }
    }

    let n_orig = buf_a.len();
    let n = n_orig.next_power_of_two();

    buf_a.append(&mut vec![
        Complex {
            re: 0_f64,
            im: 0_f64
        };
        n - n_orig
    ]);

    let mut buf_b = buf_a.clone();

    fft_inner(buf_a, &mut buf_b, n, 1);
}
