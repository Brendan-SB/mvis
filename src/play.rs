use crate::{config::Config, display::Display, fft::fft};
use kira::{
    manager::{backend::cpal::CpalBackend, AudioManager, AudioManagerSettings},
    sound::static_sound::{PlaybackState, StaticSoundData},
};
use num_complex::Complex;
use std::{io, io::Write, thread, time::Duration};

pub fn play(config: &Config, audio_file_path: &String) {
    let mut manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default())
        .expect("Could not create audio manager.");

    print!("Loading sound...");

    io::stdout().flush().unwrap();

    let sound = StaticSoundData::from_file(&audio_file_path, Default::default())
        .expect("Failed to load file.");

    println!("Complete.");

    let mut display = Display::new(&config);
    let handle = manager.play(sound.clone()).unwrap();

    while handle.state() != PlaybackState::Stopped {
        let index = (handle.position() * sound.sample_rate as f64).round() as usize;
        let mut buffer = Vec::new();

        for i in (index % sound.frames.len())
            ..((index as f32 + sound.sample_rate as f32 * config.detail).round() as usize
                % sound.frames.len())
        {
            let frame = sound.frames[i];

            buffer.push(Complex {
                re: (frame.left + frame.right) / 2_f32,
                im: 0_f32,
            });
        }

        fft(&mut buffer);

        display.update(&buffer);

        thread::sleep(Duration::from_secs_f32(1_f32 / sound.sample_rate as f32));
    }
}
