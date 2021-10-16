use crate::{config::Config, display::Display, fft::fft};
use kira::{
    instance::InstanceSettings,
    manager::{AudioManager, AudioManagerSettings},
    sound::{Sound, SoundSettings},
    Value,
};
use num_complex::Complex;
use std::{
    io,
    io::Write,
    thread::sleep,
    time::{Duration, SystemTime},
};

pub fn play(config: &Config, audio_file_path: &String) {
    let mut audio_manager = AudioManager::new(AudioManagerSettings::default())
        .expect("Could not create audio manager.");

    print!("Loading sound...");

    io::stdout().flush().unwrap();

    let sound =
        Sound::from_file(&audio_file_path, SoundSettings::default()).expect("Failed to load file.");
    let mut sound_handle = audio_manager.add_sound(sound.clone()).unwrap();

    println!("Complete.");

    let mut display = Display::new(&config);

    let sample_interval_f32_seconds = config.sample_interval as f32 / 1000_f32;
    let offset = (sound.sample_rate() as f32 * (config.sample_interval as f32 / 1000_f32)) as usize;

    let mut frame_timer_offset = 0_f32;

    sound_handle
        .play({
            let mut instance_settings = InstanceSettings::default();

            instance_settings.volume = Value::from(config.volume);

            instance_settings
        })
        .unwrap();

    let mut frame_timer = SystemTime::now();

    for i in (0..sound.frames().len() - offset).step_by(offset) {
        {
            let mut buffer = Vec::new();

            for j in (i..=i + offset).step_by(config.level_of_detail) {
                buffer.push(Complex {
                    re: (sound.frames()[j].left + sound.frames()[j].right) / 2_f32,
                    im: 0_f32,
                });
            }

            fft(&mut buffer);

            display.update(&buffer);
        }

        {
            let remaining = sample_interval_f32_seconds
                - frame_timer.elapsed().unwrap().as_secs_f32()
                + frame_timer_offset;

            if remaining > 0_f32 {
                if frame_timer_offset != 0_f32 {
                    frame_timer_offset = 0_f32;
                }

                sleep(Duration::from_secs_f32(remaining));
            } else {
                frame_timer_offset = remaining;
            }
        }

        frame_timer = SystemTime::now();
    }
}
