mod frames;
mod cli;

use clap::Parser;
use crate::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Say { text } => {
            println!("Say command with text: {}", text);
            // TODO: Implement the say functionality
        }
        Commands::Animate { variant_number } => {
            println!("Animate command with variant: {}", variant_number);
            // TODO: Implement the animate functionality
        }
        Commands::Freestyle => {
            println!("Freestyle command");
            // TODO: Implement the freestyle functionality
        }
    }
}
