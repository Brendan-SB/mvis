use crate::{
    config::Config,
    handlers::{handle_mp3, handle_ogg, handle_wav},
};
use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn run<T>(config: &Config, device: &cpal::Device, device_config: &cpal::SupportedStreamConfig)
where
    T: 'static + Send + cpal::Sample + audio::Sample + audio::Translate<f32>,
    f32: audio::Translate<i16>,
{
    let mut file = BufReader::new(File::open(&config.audio_file_path).unwrap());
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    match infer::get(&buf).unwrap().mime_type() {
        "audio/mp3" => handle_mp3(),
        "audio/ogg" => handle_ogg(),
        "audio/wav" => handle_wav(),
        _ => panic!("That file extension is not supported."),
    }
}
