mod config;
mod consts;
mod fft;

use config::Config;
use kira::{
    arrangement::{Arrangement, ArrangementSettings, SoundClip},
    manager::{AudioManager, AudioManagerSettings},
    sound::SoundSettings,
};
use std::{cell::RefCell, thread::sleep, time::Duration};

fn main() {
    Config::try_create_default_config();

    let args = Config::create_args();

    if args.value_of("help").unwrap() {
        println!("{}", args.full_usage());

        return;
    }

    let config = Config::new_from_arguments(&args);

    let mut audio_manager = AudioManager::new(AudioManagerSettings::default()).unwrap();
    let sound_handle = audio_manager
        .load_sound(&config.audio_file_path, SoundSettings::default())
        .unwrap();


    let arrangement_handle = audio_manager.add_arrangement(
        {
            let mut arrangement = Arrangement::new(ArrangementSettings::default());

            arrangement.add_clip(SoundClip::new(&sound_handle, 0_f64).trim(20_f64));

            arrangement
        }
    );

    sleep(Duration::from_secs_f64(sound_handle.duration()));
}
