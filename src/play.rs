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
        .expect("Could not create audio manager. Make sure you have an audio device enabled.");

    println!("Loading sound...");

    let sound =
        Sound::from_file(&audio_file_path, SoundSettings::default()).expect("Failed to load file.");
    let mut sound_handle = audio_manager.add_sound(sound.clone()).unwrap();

    let mut display = Display::new(&config);

    let sound_handle_duration_millis_i64 = (sound_handle.duration() * 1000_f64) as i64;
    let sample_interval_f64_seconds = config.sample_interval as f64 / 1000_f64;
    let offset = (config.sample_interval * config.level_of_detail) as i64;

    let mut frame_timer = SystemTime::now();

    sound_handle
        .play({
            let mut instance_settings = InstanceSettings::default();

            instance_settings.volume = Value::from(config.volume);

            instance_settings
        })
        .unwrap();

    for i in (0..=sound_handle_duration_millis_i64).step_by(config.sample_interval) {
        {
            let mut buffer = Vec::new();

            for j in i..=i + offset {
                let frame = sound.get_frame_at_position(j as f64 / 1000_f64);

                buffer.push(Complex::new((frame.right + frame.left) / 2_f32, 0_f32));
            }

            fft(&mut buffer);

            display.update(&buffer);
        }

        let remaining = sample_interval_f64_seconds - frame_timer.elapsed().unwrap().as_secs_f64();

        if remaining > 0_f64 {
            sleep(Duration::from_secs_f64(remaining));
        }

        frame_timer = SystemTime::now();
    }
}
