use clap::{Parser, Subcommand};
use serde::de::value::BoolDeserializer;
use std::path::{Path, PathBuf};

mod config;

const CONFIG_PATH: &str = "./config.json";

#[derive(Parser)]
#[command(
    name = "CLI Task List",
    version = "1.0",
    about = "CLI task tracker built with rust",
    author = "Tom"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Adds two numbers
    Add {
        /// First Number
        first_number: i32,

        /// Second Number
        second_number: i32,
    },

    /// Changes user directory, or prints current directory if no argument is given
    Dir {
        /// New user directory
        dir: Option<PathBuf>,
    },
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.command {
        Some(Commands::Add {
            first_number,
            second_number,
        }) => {
            println!("{}", first_number + second_number);
        }
        Some(Commands::Dir { dir }) => {
            if let Some(dir) = dir {
                config::write_config(
                    CONFIG_PATH,
                    &config::Config {
                        user_dir: dir.clone(),
                    },
                );
            } else {
                let config: config::Config = config::read_config(CONFIG_PATH);
                println!("User directory: {}", config.user_dir.display());
            }
        }
        None => {}
    }
}
