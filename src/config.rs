use serde::{Deserialize, Serialize};

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

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
    pub audio_file_name: String,
    pub config_file_name: String,
}

impl Config {
    fn new() -> Self {
        Self {
            help: false,
            volume: 1_f32,
            audio_file_name: String::new(),
            config_file_name: String::new(),
        }
    }

    pub fn new_from_base_config() -> Self {
        match home::home_dir() {
            Some(mut p) => {
                p.push(".config/mvis");

                fs::create_dir_all(&p).unwrap();

                p.push("config.json");

                match File::open(&p) {
                    Ok(mut file) => {
                        let mut content = String::new();

                        file.read_to_string(&mut content).unwrap();

                        serde_json::from_str(&content).unwrap()
                    }
                    Err(_) => {
                        let mut file = File::create(&p).unwrap();

                        let config = Self::new();

                        file.write_all(serde_json::to_string(&config).unwrap().as_bytes())
                            .unwrap();

                        config
                    }
                }
            }
            None => Self::new(),
        }
    }

    pub fn new_from_config(path: &str) -> Self {
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
            "c",
            "config",
            "The path to the config file. Default: ~/.config/mvis/config.json.",
            "CONFIG",
            Occur::Req,
            None,
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

        args.parse(env::args()).unwrap();

        config.help = args.value_of("help").unwrap();
        config.volume = args.value_of("volume").unwrap();

        Ok(())
    }
}
