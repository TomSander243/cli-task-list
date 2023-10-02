use clap::{Parser, Subcommand};
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
    /// Set directory
    #[arg(short, long, value_name = "DIR")]
    dir: Option<PathBuf>,

    /// Show directory
    #[arg(short, long)]
    show_dir: bool,

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
}

fn main() {
    let cli: Cli = Cli::parse();

    match &cli.dir {
        Some(dir) => {
            config::write_config(
                CONFIG_PATH,
                &config::Config {
                    user_dir: dir.clone(),
                },
            );
        }
        None => {}
    }

    match &cli.show_dir {
        true => {
            let config: config::Config = config::read_config(CONFIG_PATH);
            println!("User directory: {}", config.user_dir.display());
        }
        false => {}
    }

    match &cli.command {
        Some(Commands::Add {
            first_number,
            second_number,
        }) => {
            println!("{}", first_number + second_number);
        }
        None => {}
    }
}
