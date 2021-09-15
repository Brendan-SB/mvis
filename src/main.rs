mod config;
mod consts;
mod display;
mod fft;

use config::Config;
use display::Display;
use fft::fft_thread;
use kira::{
    instance::InstanceSettings,
    manager::{AudioManager, AudioManagerSettings},
    sound::{Sound, SoundSettings},
    Value,
};
use ringbuf::RingBuffer;
use std::{
    thread::{sleep, spawn},
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

    let sound_handle_duration_millis = sound_handle.duration() * 1000_f64;

    let (mut producer, mut consumer) =
        RingBuffer::new((sound_handle_duration_millis / 20_f64) as usize).split();

    let fft_worker = spawn(move || {
        fft_thread(&sound, sound_handle_duration_millis as i64, &mut producer);
    });

    sound_handle
        .play({
            let mut instance_settings = InstanceSettings::default();

            instance_settings.volume = Value::from(config.volume);

            instance_settings
        })
        .unwrap();

    {
        let mut frame_timer = SystemTime::now();

        let duration_1_millis = Duration::from_millis(1);
        let duration_20_millis = Duration::from_millis(20);

        let mut i = 0;

        while i < consumer.capacity() {
            if consumer.is_empty() {
                sleep(duration_1_millis);

                continue;
            }

            {
                let remaining =
                    duration_20_millis.as_secs_f64() - frame_timer.elapsed().unwrap().as_secs_f64();

                if remaining > 0_f64 {
                    sleep(Duration::from_secs_f64(remaining));
                }
            }

            i += 1;

            frame_timer = SystemTime::now();
        }
    }

    fft_worker.join().unwrap();
}
