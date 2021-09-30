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
use tui::{
    style,
    style::{Color, Modifier},
};

#[derive(Serialize, Deserialize)]
pub struct Style {
    pub fg: Option<String>,
    pub bg: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub volume: f64,
    pub sample_interval: usize,
    pub level_of_detail: usize,
    pub bar_width: u16,
    pub style: Style,
}

impl Style {
    fn new() -> Self {
        Self { fg: None, bg: None }
    }

    fn decode_style_value(hex: &Option<String>) -> Option<Color> {
        match hex {
            Some(v) => {
                let decoded = hex::decode(v).unwrap();

                Some(Color::Rgb {
                    0: decoded[0],
                    1: decoded[1],
                    2: decoded[2],
                })
            }
            None => None,
        }
    }

    pub fn to_tui_style(&self) -> style::Style {
        style::Style {
            fg: Self::decode_style_value(&self.fg),
            bg: Self::decode_style_value(&self.bg),
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }
}

impl Config {
    fn new() -> Self {
        Self {
            volume: 1_f64,
            sample_interval: 15,
            level_of_detail: 1,
            bar_width: 5,
            style: Style::new(),
        }
    }

    pub fn try_create_default_config_file() {
        let directory_path = home_dir().unwrap().join(".config/mvis");
        let file_path = directory_path.join("config.json");

        if !file_path.exists() {
            create_dir_all(directory_path).unwrap();

            File::create(file_path)
                .unwrap()
                .write_all(
                    &serde_json::to_string_pretty(&Self::new())
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
            "f",
            "file",
            "The path to the audio file.",
            "FILE",
            Occur::Optional,
            None,
        );
        args.option(
            "u",
            "url",
            "The url of a video you wish to download and play.",
            "URL",
            Occur::Optional,
            None,
        );
        args.option(
            "c",
            "config",
            "The path to the config file.",
            "CONFIG",
            Occur::Optional,
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
            None,
        );
        args.option(
            "s",
            "sample-interval",
            "The interval the sample thread should take from the buffer at each step in milliseconds.",
            "SAMPLE_INTERVAL",
            Occur::Optional,
            None,
        );
        args.option(
            "l",
            "level-of-detail",
            "The level between the steps in the sample for loop.",
            "LEVEL_OF_DETAIL",
            Occur::Optional,
            None,
        );
        args.option(
            "b",
            "bar-width",
            "The width of the bars.",
            "BAR_WIDTH",
            Occur::Optional,
            None,
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

        if let Ok(volume) = args.validated_value_of(
            "volume",
            &[
                Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 0_f64)),
                Box::new(OrderValidation::new(Order::LessThanOrEqual, 1_f64)),
            ],
        ) {
            config.volume = volume;
        }

        if let Ok(sample_interval) = args.validated_value_of(
            "sample-interval",
            &[Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 1))],
        ) {
            config.sample_interval = sample_interval;
        }

        if let Ok(level_of_detail) = args.validated_value_of(
            "level-of-detail",
            &[
                Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 1)),
                Box::new(OrderValidation::new(Order::LessThanOrEqual, 1000)),
            ],
        ) {
            config.level_of_detail = level_of_detail;
        }

        if let Ok(bar_width) = args.validated_value_of(
            "bar-width",
            &[
                Box::new(OrderValidation::new(Order::GreaterThanOrEqual, 1)),
                Box::new(OrderValidation::new(Order::LessThanOrEqual, 10)),
            ],
        ) {
            config.bar_width = bar_width;
        }

        config
    }
}
