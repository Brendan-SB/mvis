extern crate args;
extern crate ncurses;

mod config;
mod fft;

use config::Config;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::{Cursor, Read};

fn main() {
    println!("Loading args...");

    let mut args = Config::new_args();

    if args.value_of("help").unwrap() {
        println!("{}", args.full_usage());

        return;
    }

    println!("Loading config...");

    let config = Config::new_from_arguments(&mut args);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    println!("Loading audio file...");

    let mut file = File::open(config.audio_file_path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();

    let source = Decoder::new(Cursor::new(buf.clone())).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    println!("Configuring sink...");

    sink.set_volume(config.volume);

    println!("Playing audio...");

    sink.append(source);
    sink.sleep_until_end();
}
