extern crate args;
extern crate ncurses;

mod config;
mod fft;

use args::Args;
use config::Config;
use ndarray::Array;
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

const PROGRAM_NAME: &'static str = "mvis";
const PROGRAM_DESC: &'static str = "A command line music visualizer.";

fn main() {
    Config::try_create_default_config();

    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    let mut config = Config::new();
    let mut help = false;
    config.update_from_arguments(&mut args, &mut help).unwrap();

    if help {
        print!("{}", args.full_usage());

        return;
    }

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    println!("{}", config.audio_file_path);

    let file = BufReader::new(File::open(config.audio_file_path).unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let array = Array::random(1024, Uniform::new(0_f32, 10_f32));

    fft::fft(&array);

    sink.append(source);

    sink.sleep_until_end();
}
