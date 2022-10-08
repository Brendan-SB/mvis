use crate::{config::Config, play::play};

pub fn run() -> anyhow::Result<()> {
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
