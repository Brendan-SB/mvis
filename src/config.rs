use serde::{Deserialize, Serialize};

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub volume: Option<f32>,
    pub audio_file_name: Option<String>,
    pub config_file_name: Option<String>,
}

impl Config {
    fn new() -> Self {
        Self {
            volume: Some(1.0),
            audio_file_name: Some(String::new()),
            config_file_name: Some(String::new()),
        }
    }

    fn new_blank() -> Self {
        Self {
            volume: None,
            audio_file_name: None,
            config_file_name: None,
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

    pub fn update_from_arguments(config: &mut Config) {
        let args: Vec<String> = env::args().collect();

        let load_from_json = false;
        let mut config_updated = Self::new_blank();

        {
            let mut skip = false;

            for mut i in 0..args.len() {
                if skip {
                    skip = false;

                    continue;
                }

                if args[i] == "--config" || args[i] == "-c" {
                    i += 1;

                    if i > args.len() {
                        break;
                    }

                    config_updated.config_file_name = Some(args[i].clone());

                    skip = true;
                } else if args[i] == "--volume" || args[i] == "-v" {
                    i += 1;

                    if i > args.len() {
                        break;
                    }

                    match args[i].trim().parse::<f32>() {
                        Ok(v) => config_updated.volume = Some(v),
                        Err(_) => panic!("Volume must be an integer."),
                    }

                    skip = true;
                }
            }

            if load_from_json {
                *config =
                    Self::new_from_config(config_updated.config_file_name.as_ref().unwrap());
            }

            match config_updated.volume {
                Some(_) => config.volume = config_updated.volume,
                None => {},
            }

            match config_updated.audio_file_name {
                Some(_) => config.audio_file_name = config_updated.audio_file_name.clone(),
                None => {},
            }

            match config_updated.config_file_name {
                Some(_) => config.config_file_name = config_updated.config_file_name.clone(),
                None => {},
            }
        }
    }
}
