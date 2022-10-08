use crate::{clean_tmp_dir::clean_tmp_dir, config::Config, play::play, NO_HOME};
use anyhow::Context;
use home::home_dir;
use std::fs;

pub fn run() -> anyhow::Result<()> {
    let tmp_dir_path = home_dir().context(NO_HOME)?.join(".local/share/mvis/tmp");

    fs::create_dir_all(&tmp_dir_path)?;

    clean_tmp_dir(&tmp_dir_path)?;

    Config::try_create_default_config_file()?;

    let args = Config::create_args()?;

    if args.value_of("help")? {
        println!("{}", args.full_usage());

        return Ok(());
    }

    if args.value_of("regenerate-config")? {
        Config::print_default_config()?;

        return Ok(());
    }

    let config = Config::from_arguments(&args);

    play(&config?, &args.value_of("file")?)
}
