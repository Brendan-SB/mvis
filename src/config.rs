use crate::consts::{PROGRAM_DESC, PROGRAM_NAME};
use args::{
    validations::{Order, OrderValidation},
    Args,
};
use getopts::Occur;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::{
    env,
    fs::{create_dir_all, File},
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub audio_file_path: String,
    pub volume: f64,
    pub sample_interval: usize,
    pub level_of_detail: usize,
    pub bar_width: u16,
}

impl Config {
    pub fn try_create_default_config() {
        let directory_path = home_dir().unwrap().join(".config/mvis");
        let file_path = directory_path.join("config.json");

        if !file_path.exists() {
            create_dir_all(directory_path).unwrap();

            File::create(file_path)
                .unwrap()
                .write_all(
                    &serde_json::to_string(&Self {
                        audio_file_path: String::new(),
                        volume: 1_f64,
                        sample_interval: 15_usize,
                        level_of_detail: 5_usize,
                        bar_width: 1_u16,
                    })
                    .unwrap()
                    .as_bytes(),
                )
                .unwrap();
        }
    }

    pub fn create_args() -> Args {
        let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);

        args.flag("h", "help", "Print the usage menu.");
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
        args.option(
            "v",
            "volume",
            "Sets the volume.",
            "VOLUME",
            Occur::Optional,
            Some(String::from("1")),
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
            "s",
            "sample-interval",
            "The interval the sample thread should take from the buffer at each step in milliseconds. Default: 15.",
            "SAMPLE_INTERVAL",
            Occur::Req,
            Some(
                String::from("15")
            ),
        );
        args.option(
            "l",
            "level-of-detail",
            "The level between the steps in the sample for loop. Default: 10.",
            "LEVEL_OF_DETAIL",
            Occur::Req,
            Some(String::from("5")),
        );
        args.option(
            "b",
            "bar-width",
            "The width of the bars.",
            "BAR_WIDTH",
            Occur::Optional,
            Some(String::from("1")),
        );

        args.parse(env::args()).unwrap();

        args
    }

    fn from_config(path: String) -> Self {
        let mut contents = String::new();

        File::open(&path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        serde_json::from_str(contents.as_str()).unwrap()
    }

    pub fn from_arguments(args: &Args) -> Self {
        let mut config = Self::from_config(args.value_of("config").unwrap());

        config.volume = args
            .validated_value_of(
                "volume",
                &[
                    Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 0_f64)),
                    Box::new(OrderValidation::new(Order::LessThanOrEqual, 1_f64)),
                ],
            )
            .unwrap();
        config.audio_file_path = args.value_of("file").unwrap();
        config.sample_interval = args
            .validated_value_of(
                "sample-interval",
                &[Box::new(OrderValidation::new(
                    Order::GreaterThanOrEqual,
                    1_usize,
                ))],
            )
            .unwrap();
        config.level_of_detail = args
            .validated_value_of(
                "level-of-detail",
                &[
                    Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 1_usize)),
                    Box::new(OrderValidation::new(Order::LessThanOrEqual, 1000_usize)),
                ],
            )
            .unwrap();
        config.bar_width = args
            .validated_value_of(
                "bar-width",
                &[
                    Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 1_u16)),
                    Box::new(OrderValidation::new(Order::LessThanOrEqual, 10_16)),
                ],
            )
            .unwrap();

        config
    }
}
