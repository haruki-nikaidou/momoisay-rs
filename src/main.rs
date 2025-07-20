mod cli;
mod frames;

use crate::cli::{Cli, Commands};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Say { text } => {
            println!("Say command with text: {}", text);
            // TODO: Implement the say functionality
        }
        Commands::Animate {
            text,
            variant_number,
        } => {
            println!("Animate command with variant: {}", variant_number);
            println!("Text: {:?}", text);
            // TODO: Implement the animate functionality
        }
        Commands::Freestyle => {
            println!("Freestyle command");
            // TODO: Implement the freestyle functionality
        }
    }
}
