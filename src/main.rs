extern crate args;
extern crate ncurses;

mod config;
mod consts;
mod fft;

use config::Config;
use kira::{
    manager::{AudioManager, AudioManagerSettings},
    sound::SoundSettings,
};

fn main() {
    Config::try_create_default_config();

    let args = Config::create_args();

    if args.value_of("help").unwrap() {
        println!("{}", args.full_usage());

        return;
    }

    let config = Config::new_from_arguments(&args);

    let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();

    let mut sound_handle =
        audio_manager.load_sound(&config.audio_file_path, SoundSettings::default()).unwrap();

    sound_handle.play(config.create_instance_settings()).unwrap();

    std::thread::sleep(std::time::Duration::from_secs_f64(sound_handle.duration()));
}
