mod cli;
mod frames;
mod display;

use crate::cli::{Cli, Commands};
use crate::frames::{STATIC_FRAME, ANIMATE1_FRAMES, ANIMATE2_FRAMES};
use crate::display::{display_say_command, display_animation};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Say { text } => {
            display_say_command(&STATIC_FRAME, &text);
        }
        Commands::Animate {
            text,
            variant_number,
        } => {
            let frames = match variant_number {
                1 => &*ANIMATE1_FRAMES,
                2 => &*ANIMATE2_FRAMES,
                _ => {
                    eprintln!("Invalid variant number. Use 1 or 2.");
                    std::process::exit(1);
                }
            };
            
            if let Err(e) = display_animation(frames, text.as_deref()) {
                eprintln!("Error during animation: {e}");
                std::process::exit(1);
            }
        }
        Commands::Freestyle { text } => {
            // Freestyle uses animation variant 1 by default
            if let Err(e) = display_animation(&ANIMATE1_FRAMES, text.as_deref()) {
                eprintln!("Error during freestyle animation: {e}");
                std::process::exit(1);
            }
        }
    }
}
