mod config;
mod consts;
mod display;
mod fft;

use config::Config;
use display::Display;
use fft::fft;
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

fn main() {
    Config::try_create_default_config();

    let args = Config::create_args();

    if args.value_of("help").unwrap() {
        println!("{}", args.full_usage());

        return;
    }

    let config = Config::from_arguments(&args);

    let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();

    println!("Loading sound...");

    let sound = Sound::from_file(&config.audio_file_path, SoundSettings::default()).unwrap();
    let mut sound_handle = audio_manager.add_sound(sound.clone()).unwrap();

    let mut display = Display::new(&config);

    let sound_handle_duration_millis_i64 = (sound_handle.duration() * 1000_f64) as i64;
    let sample_interval_f64 = config.sample_interval as f64;
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

            display.update(&fft(&buffer));
        }

        let remaining =
            sample_interval_f64 / 1000_f64 - frame_timer.elapsed().unwrap().as_secs_f64();

        if remaining > 0_f64 {
            sleep(Duration::from_secs_f64(remaining));
        }

        frame_timer = SystemTime::now();
    }
}
