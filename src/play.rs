use crate::{config::Config, display::Display, fft::fft};
use kira::{
    instance::InstanceSettings,
    manager::{AudioManager, AudioManagerSettings},
    sound::{Sound, SoundSettings},
    Value,
};
use num_complex::Complex;
use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

pub fn play(config: &Config, audio_file_path: &String) {
    let mut audio_manager = AudioManager::new(AudioManagerSettings::default())
        .expect("Could not create audio manager.");

    println!("Loading sound...");

    let sound =
        Sound::from_file(&audio_file_path, SoundSettings::default()).expect("Failed to load file.");
    let mut sound_handle = audio_manager.add_sound(sound.clone()).unwrap();

    let mut display = Display::new(&config);

    let sample_interval_f64_seconds = config.sample_interval as f64 / 1000_f64;
    let offset = (sound.sample_rate() as f32 * (config.sample_interval as f32 / 1000_f32)) as usize;

    let mut frame_timer = SystemTime::now();

    sound_handle
        .play({
            let mut instance_settings = InstanceSettings::default();

            instance_settings.volume = Value::from(config.volume);

            instance_settings
        })
        .unwrap();

    for i in (0..=sound.frames().len() - offset).step_by(offset) {
        let mut buffer = Vec::new();

        for j in (i..=i + offset).step_by(config.level_of_detail) {
            buffer.push(Complex {
                re: (sound.frames()[j].left + sound.frames()[j].right) / 2_f32,
                im: 0_f32,
            });
        }

        fft(&mut buffer);

        display.update(&buffer);

        let remaining = sample_interval_f64_seconds - frame_timer.elapsed().unwrap().as_secs_f64();

        if remaining > 0_f64 {
            sleep(Duration::from_secs_f64(remaining));
        }

        frame_timer = SystemTime::now();
    }
}
