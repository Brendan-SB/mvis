extern crate args;
extern crate ncurses;

mod config;
mod fft;

use config::Config;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut config = Config::new();
    Config::update_from_arguments(&mut config);

    let file = BufReader::new(File::open(config.audio_file_path).unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);

    sink.sleep_until_end();
}
