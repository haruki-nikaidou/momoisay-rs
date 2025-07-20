use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "momoisay")]
#[command(about = "Momoisay is a CLI program like cowsay, but instead of a talking cow, it's Saiba Momoi from Blue Archive!")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Display Momoi saying the provided text
    Say {
        /// The text for Momoi to say
        text: String,
    },
    
    /// Display an animated Momoi with no text bubble (variant 1 or 2)
    Animate {
        /// Animation variant number (1 or 2, default: 1)
        #[arg(default_value = "1")]
        variant_number: u8,
    },
    
    /// Display Momoi in freestyle mode. Pretty cool for ricing btw.
    Freestyle,
}
