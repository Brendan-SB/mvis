mod config;
mod consts;
mod fft;

use config::Config;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::path::Path;

fn run<T>(config: &Config, device: &cpal::Device, device_config: &cpal::SupportedStreamConfig)
where
    T: 'static + Send + cpal::Sample + audio::Sample + audio::Translate<f32>,
    f32: audio::Translate<i16>,
{
    match config.audio_file_path.split(".").last().unwrap() {
        "mp3" => {}
        "ogg" => {}
        "wav" => {}
        _ => panic!("That file extension is not supported."),
    }
}

fn main() {
    Config::try_create_default_config();

    let args = Config::create_args();

    if args.value_of("help").unwrap() {
        println!("{}", args.full_usage());

        return;
    }

    let config = Config::new_from_arguments(&args);

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let device_config = device.default_output_config().unwrap();

    match device_config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&config, &device, &device_config),
        cpal::SampleFormat::I16 => run::<i16>(&config, &device, &device_config),
        cpal::SampleFormat::U16 => run::<u16>(&config, &device, &device_config),
    }
}
