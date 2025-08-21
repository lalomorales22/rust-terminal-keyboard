use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod app;
mod audio;
mod config;
mod file_dialog;
mod midi;
mod piano;
mod ui;
mod effects;

use app::App;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    /// Load and play a MIDI file
    #[arg(short, long)]
    file: Option<PathBuf>,
    
    /// Enable debug mode
    #[arg(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Play a MIDI file
    Play {
        /// Path to MIDI file
        file: PathBuf,
    },
    /// Configure the application
    Config {
        /// Show current configuration
        #[arg(short, long)]
        show: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    let mut app = App::new(cli.debug).await?;
    
    match cli.command {
        Some(Commands::Play { file }) => {
            app.load_midi_file(file).await?;
        }
        Some(Commands::Config { show }) => {
            if show {
                app.show_config()?;
                return Ok(());
            }
        }
        None => {}
    }
    
    if let Some(file) = cli.file {
        app.load_midi_file(file).await?;
    }
    
    app.run().await
}