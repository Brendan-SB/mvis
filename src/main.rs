mod config;
mod consts;
mod fft;

use config::Config;
use kira::{
    manager::{AudioManager, AudioManagerSettings},
    sound::{Sound, SoundSettings},
};
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

    let config = Config::new_from_arguments(&args);

    println!("Loading audio...");

    let sound = Sound::from_file(&config.audio_file_path, SoundSettings::default()).unwrap();
    let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
    let sound_handle = audio_manager.add_sound(sound.clone()).unwrap();

    let sound_handle_duration = sound_handle.duration();

    assert!(
        sound_handle_duration >= 0.02_f64,
        "Your sound file is too short."
    );

    let now = SystemTime::now();

    println!("Playing audio...");

    {
        let mut i_prev = 0_f64;
        let mut i = 0.02_f64;

        while i < sound_handle_duration {
            i_prev = i;
            i += 0.02_f64;
        }
    }

    let duration_remaining = sound_handle_duration - now.elapsed().unwrap().as_secs_f64();

    if duration_remaining > 0_f64 {
        sleep(Duration::from_secs_f64(duration_remaining));
    }
}
