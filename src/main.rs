mod clean_tmp_dir;
mod config;
mod consts;
mod display;
mod download_mp3;
mod fft;
mod play;

use clean_tmp_dir::clean_tmp_dir;
use config::Config;
use download_mp3::download_mp3;
use home::home_dir;
use play::play;
use std::fs;

fn main() {
    let tmp_dir_path = home_dir().unwrap().join(".local/share/mvis/tmp");

    fs::create_dir_all(&tmp_dir_path).unwrap();
    clean_tmp_dir(&tmp_dir_path).unwrap();

    Config::try_create_default_config_file();

    let args = Config::create_args();

    if args.value_of("help").unwrap() {
        println!("{}", args.full_usage());

        return;
    }

    let config = Config::from_arguments(&args);

    match args.value_of("file") {
        Ok(v) => {
            play(&config, &v);
        }

        Err(_) => {
            play(
                &config,
                &download_mp3(
                    &tmp_dir_path,
                    &args.value_of("url").expect("Supply either a file path or a url to download as a command line argument."),
                ).unwrap(),
            );

            clean_tmp_dir(&tmp_dir_path).unwrap();
        }
    }
}
