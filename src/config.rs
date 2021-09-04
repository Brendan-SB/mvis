use serde::{Deserialize, Serialize};

use std::env;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};

use args::validations::{Order, OrderValidation};
use args::Args;
use getopts::Occur;

use home::home_dir;

const PROGRAM_NAME: &str = "mvis";
const PROGRAM_DESC: &str = "A command line music visualizer.";

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub volume: f32,
    pub audio_file_path: String,
}

impl Config {
    pub fn try_create_default_config() {
        let directory_path = home_dir().unwrap().join(".config/mvis");
        let file_path = directory_path.join("config.json");

        if !file_path.exists() {
            create_dir_all(directory_path).unwrap();
            File::create(file_path)
                .unwrap()
                .write_all(&serde_json::to_string(&Self::new()).unwrap().as_bytes())
                .unwrap();
        }
    }

    pub fn new_args() -> Args {
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
            None,
        );
        args.option(
            "c",
            "config",
            "The path to the config file. Default: ~/.config/mvis/config.json.",
            "CONFIG",
            Occur::Req,
            Some(
                home_dir()
                    .unwrap()
                    .join(".config/mvis/config.json")
                    .into_os_string()
                    .into_string()
                    .unwrap(),
            ),
        );

        args.parse(env::args()).unwrap();

        args
    }

    pub fn new() -> Self {
        Self {
            volume: 1_f32,
            audio_file_path: String::new(),
        }
    }

    pub fn new_from_config(path: String) -> Self {
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();

                file.read_to_string(&mut contents).unwrap();

                serde_json::from_str(contents.as_str()).unwrap()
            }
            Err(_) => panic!("Config file does not exist."),
        }
    }

    pub fn new_from_arguments(args: &mut Args) -> Self {
        let mut config = Self::new_from_config(args.value_of("config").unwrap());

        config.volume = args
            .validated_value_of(
                "volume",
                &[
                    Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 0_f32)),
                    Box::new(OrderValidation::new(Order::LessThanOrEqual, 1_f32)),
                ],
            )
            .unwrap();

        config.audio_file_path = args.value_of("file").unwrap();

        config
    }
}
