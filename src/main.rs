mod cli;
mod frames;
mod display;

use crate::cli::{Cli, Commands};
use crate::frames::{STATIC_FRAME, ANIMATE1_FRAMES, ANIMATE2_FRAMES};
use crate::display::{display_say_command, display_animation_once, check_terminal_size, setup_terminal, cleanup_terminal};
use clap::Parser;
use rand::Rng;

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
            
            if !check_terminal_size().unwrap_or(false) {
                println!("your terminal is too small for momoi");
                return;
            }
            
            if let Err(e) = setup_terminal() {
                eprintln!("Error setting up terminal: {}", e);
                std::process::exit(1);
            }
            
            loop {
                match display_animation_once(frames, text.as_deref()) {
                    Ok(should_exit) => {
                        if should_exit {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error during animation: {}", e);
                        break;
                    }
                }
            }
            
            if let Err(e) = cleanup_terminal() {
                eprintln!("Error cleaning up terminal: {}", e);
                std::process::exit(1);
            }
        }
        Commands::Freestyle { text } => {
            if !check_terminal_size().unwrap_or(false) {
                println!("your terminal is too small for momoi");
                return;
            }
            
            if let Err(e) = setup_terminal() {
                eprintln!("Error setting up terminal: {}", e);
                std::process::exit(1);
            }
            
            let mut rng = rand::thread_rng();
            
            loop {
                // Randomly select animation variant (1 or 2)
                let variant = rng.gen_range(1..=2);
                let frames = match variant {
                    1 => &*ANIMATE1_FRAMES,
                    2 => &*ANIMATE2_FRAMES,
                    _ => unreachable!(),
                };
                
                match display_animation_once(frames, text.as_deref()) {
                    Ok(should_exit) => {
                        if should_exit {
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("Error during freestyle animation: {}", e);
                        break;
                    }
                }
            }
            
            if let Err(e) = cleanup_terminal() {
                eprintln!("Error cleaning up terminal: {}", e);
                std::process::exit(1);
            }
        }
    }
}
