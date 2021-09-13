use crate::{config::Config, consts::CHUNK_SIZE};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use minimp3::{Decoder, Error, Frame};
use std::fs::File;

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: cpal::Sample, 
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from::<f32>(&next_sample());

        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}

fn handle_mp3(config: &Config) {
    let mut decoder = Decoder::new(File::open(&config.audio_file_path).unwrap());
}

pub fn run<T>(config: &Config, device: &cpal::Device, device_config: &cpal::SupportedStreamConfig)
where
    T: cpal::Sample,
{
    let mut supported_configs_range = device.supported_output_configs().unwrap();
    let supported_config = supported_configs_range
        .next()
        .unwrap()
        .with_max_sample_rate();
    let stream_config: cpal::StreamConfig = supported_config.into();

    let stream = device
        .build_output_stream(
            &stream_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {},
            move |err| {
                println!("Stream error occured: {}.", err);
            },
        )
        .unwrap();

    match infer::get_from_path(&config.audio_file_path)
        .unwrap()
        .unwrap()
        .mime_type()
    {
        "audio/mpeg" => handle_mp3(config),
        file_type => panic!("The file type {} is not supported.", file_type),
    }

    stream.play().unwrap();
}
