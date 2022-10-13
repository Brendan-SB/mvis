use crate::{config::Config, display::Display, fft::fft};
use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{PlaybackState, StaticSoundData},
};
use num_complex::Complex;
use std::{
    io,
    io::Write,
    thread,
    time::{Duration, Instant},
};

pub fn play(config: &Config, audio_file_path: &String) -> anyhow::Result<()> {
    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())?;

    print!("Loading sound...");

    io::stdout().flush()?;

    let sound = StaticSoundData::from_file(&audio_file_path, Default::default())?;

    println!("Complete.");

    let mut display = Display::new(&config)?;
    let handle = manager.play(sound.clone())?;

    while handle.state() != PlaybackState::Stopped {
        let frame_start = Instant::now();
        let index = handle.position() * sound.sample_rate as f64;
        let start = (index.floor() as i64 % sound.frames.len() as i64) as usize;
        let end = ((index + sound.sample_rate as f64 * config.detail as f64).ceil() as i64
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
