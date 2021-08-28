use serde::{Deserialize, Serialize};

use std::env;
use std::fs::File;
use std::io::{Read, Write};

use args::validations::{Order, OrderValidation};
use args::{Args, ArgsError};
use getopts::Occur;

use home::home_dir;

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub volume: f32,
    pub audio_file_path: String,
}

impl Config {
    pub fn try_create_default_config() {
        let path = home_dir().unwrap().join(".config/mvis/config.json");

        if !path.exists() {
            File::create(path)
                .unwrap()
                .write_all(&serde_json::to_string(&Self::new()).unwrap().as_bytes())
                .unwrap();
        }
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

    pub fn update_from_arguments(
        &mut self,
        args: &mut Args,
        help: &mut bool,
    ) -> Result<(), ArgsError> {
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
        *self = Self::new_from_config(args.value_of("config").unwrap());

        {
            let gte_0 = Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 0_f32));
            let lte_1 = Box::new(OrderValidation::new(Order::LessThanOrEqual, 1_f32));
            self.volume = args.validated_value_of("volume", &[gte_0, lte_1]).unwrap();
        }

        self.audio_file_path = args.value_of("file").unwrap();

        println!("Value: {}", self.audio_file_path);

        Ok(())
    }
}
