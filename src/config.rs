use serde::{Deserialize, Serialize};

use std::env;
use std::fs::File;
use std::io::Read;

use args::{Args, ArgsError};
use getopts::Occur;

use std::path::PathBuf;

use home::home_dir;

const PROGRAM_NAME: &'static str = "mvis";
const PROGRAM_DESC: &'static str = "A command line music visualizer.";

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub help: bool,
    pub volume: f32,
    pub audio_file_path: String,
    pub config_file_path: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            help: false,
            volume: 1_f32,
            audio_file_path: String::new(),
            config_file_path: String::new(),
        }
    }

    pub fn new_from_config(path: &String) -> Self {
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();

                file.read_to_string(&mut contents).unwrap();

                serde_json::from_str(contents.as_str()).unwrap()
            }
            Err(_) => panic!("Config file does not exist."),
        }
    }

    pub fn update_from_arguments(config: &mut Config) -> Result<(), ArgsError> {
        let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);

        args.flag("h", "help", "Print the usage menu.");
        args.option(
            "v",
            "volume",
            "Sets the volume.",
            "VOLUME",
            Occur::Req,
            Some(String::from("1.0")),
        );
        args.option(
            "f",
            "file",
            "The path to the audio file.",
            "FILE",
            Occur::Optional,
            Some(
                [
                    home_dir().unwrap(),
                    PathBuf::from(".config"),
                    PathBuf::from("mvis"),
                    PathBuf::from("config.json"),
                ]
                .iter()
                .collect::<PathBuf>()
                .into_os_string()
                .into_string()
                .unwrap(),
            ),
        );
        args.option(
            "c",
            "config",
            "The path to the config file. Default: ~/.config/mvis/config.json.",
            "CONFIG",
            Occur::Req,
            None,
        );

        args.parse(env::args()).unwrap();

        config.config_file_path = args.value_of("config").unwrap();

        *config = Self::new_from_config(&config.config_file_path);

        config.help = args.value_of("help").unwrap();
        config.volume = args.value_of("volume").unwrap();
        config.audio_file_path = args.value_of("file").unwrap();

        Ok(())
    }
}
