mod config;
mod consts;
mod fft;

use config::Config;
use std::{
    thread::sleep,
    io::{Read, BufReader},
    fs::File,
    time::{SystemTime, Duration},
};
use kira::{
    sound::SoundSettings,
    manager::{AudioManager, AudioManagerSettings},
    instance::InstanceSettings,
    Value,
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

    let mut sound_handle = audio_manager.load_sound(&config.audio_file_path, SoundSettings::default()).unwrap();

    let mut instance_settings = InstanceSettings::default();
    instance_settings.volume = Value::from(config.volume);

    sound_handle.play(instance_settings).unwrap();

    sleep(Duration::from_secs_f64(sound_handle.duration()));
}
