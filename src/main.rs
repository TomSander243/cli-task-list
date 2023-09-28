use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
        second_number: i32   
    },
}

fn main() {
    let cli = Cli::parse();

    if let Some(dir_path) = cli.dir.as_deref() {
        println!("Path to directory: {}", dir_path.display());
    }

    match &cli.command {
        Some(Commands::Add { first_number, second_number}) => {
            println!("{}", first_number + second_number);
        }
        None => {}
    }
}
