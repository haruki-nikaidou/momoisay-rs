use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "momoisay")]
#[command(
    about = "Momoisay is a CLI program like cowsay, but instead of a talking cow, it's Saiba Momoi from Blue Archive!"
)]
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

    /// Display an animated Momoi (variant 1 or 2)
    Animate {
        /// The text for Momoi to say
        text: Option<String>,
        /// Animation variant number (1 or 2, default: 1)
        #[arg(short, long, default_value = "1")]
        variant_number: u8,
    },

    /// Display Momoi in freestyle mode. Pretty cool for ricing btw.
    Freestyle {
        /// The text for Momoi to say
        text: Option<String>,
    },
}
