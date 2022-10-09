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
        let index = (handle.position() * sound.sample_rate as f64) as f32;
        let start = (index.floor() as i32 % sound.frames.len() as i32) as usize;
        let end = ((index + sound.sample_rate as f32 * config.detail).ceil() as i32
            % sound.frames.len() as i32) as usize;

        if start >= end {
            break;
        }

        let mut buffer = Vec::new();

        for i in start..end {
            let frame = sound.frames[i];

            buffer.push(Complex {
                re: (frame.left + frame.right) / 2_f32,
                im: 0_f32,
            });
        }

        fft(&mut buffer);

        display.update(&buffer)?;

        let frame_end = Instant::now();
        let time = frame_end.duration_since(frame_start);
        let fps = 1.0 / time.as_secs_f32();

        if fps > config.fps as f32 {
            thread::sleep(Duration::from_secs_f32(1.0 / (fps - config.fps as f32)));
        }
    }

    Ok(())
}
