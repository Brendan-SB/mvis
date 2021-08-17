use serde::{Deserialize, Serialize};

use std::env;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub volume: f32,
    pub audio_file_name: String,
}

impl Config {
    fn new() -> Self {
        Config {
            volume: 1.0,
            audio_file_name: String::new(),
        }
    }

    pub fn new_from_config(path: Option<&str>) -> Self {
        match home::home_dir() {
            Some(mut p) => match path {
                Some(p) => match File::open(p) {
                    Ok(mut file) => {
                        let mut contents = String::new();

                        file.read_to_string(&mut contents).unwrap();

                        serde_json::from_str(contents.as_str()).unwrap()
                    }
                    Err(_) => panic!("Config file does not exist."),
                },
                None => {
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
            },
            None => Config::new(),
        }
    }

    pub fn update_from_arguments(config: &mut Config) {
        let args: Vec<String> = env::args().collect();

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

                *config = Self::new_from_config(Some(args[i].trim()));

                skip = true;
            } else if args[i] == "--volume" || args[i] == "-v" {
                i += 1;

                if i > args.len() {
                    break;
                }

                match args[i].trim().parse::<f32>() {
                    Ok(v) => config.volume = v,
                    Err(_) => panic!("Volume must be an integer."),
                }

                skip = true;
            }
        }
    }
}
