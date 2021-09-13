mod config;
mod consts;
mod fft;
mod stream;

use crate::stream::run;
use config::Config;
use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    Config::try_create_default_config();

    let args = Config::create_args();

    if args.value_of("help").unwrap() {
        println!("{}", args.full_usage());

        return;
    }

    let config = Config::from_arguments(&args);

    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let device_config = device.default_output_config().unwrap();

    match device_config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&config, &device, &device_config),
        cpal::SampleFormat::I16 => run::<i16>(&config, &device, &device_config),
        cpal::SampleFormat::U16 => run::<u16>(&config, &device, &device_config),
    }
}
