use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod config;

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
    let config_path = "./config.json";

    let config = config::read_config(config_path);
    println!("User directory: {}", config.user_dir.display());

    let cli = Cli::parse();

    if let Some(dir_path) = cli.dir.as_deref() {
        println!("Path to directory: {}", dir_path.display());
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
