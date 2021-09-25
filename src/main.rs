mod config;
mod consts;
mod display;
mod download_mp3;
mod fft;
mod play;
mod temp_dir;

use config::Config;
use download_mp3::download_mp3;
use play::play;

fn main() {
    temp_dir::clean_temp_dir();

    Config::try_create_default_config_file();

    let args = Config::create_args();

    if args.value_of("help").unwrap() {
        println!("{}", args.full_usage());

        return;
    }

    match args.value_of("file") {
        Ok(v) => {
            play(&args, &v);
        }

        Err(_) => {
            play(&args, &download_mp3(&args.value_of("url").unwrap()));
        }
    }
}
