use crate::config;
use clap::Parser;
use dialoguer::{Input, Password};

#[derive(Parser)]
#[command(name = "loom")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    Init {},
    Games {},
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {} => {
            let server_url: String = Input::new()
                .with_prompt("Enter server URL")
                .interact_text()?;

            let token: String = Password::new()
                .with_prompt("Enter your access token")
                .interact()?;

            match config::save_default_config(server_url, token) {
                Ok(_) => {
                    println!("Configuration file created successfully.");
                }
                Err(e) => {
                    eprintln!("Failed to create configuration file: {}", e);
                }
            }
        }
        Commands::Games {} => {
            let config = config::load_config();
            match config {
                Ok(cfg) => {
                    if cfg.games.is_empty() {
                        println!("No games configured.");
                    } else {
                        println!("Configured games:");
                        for game in cfg.games {
                            println!("ID: {}, Path: {}", game.id, game.path);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Please run 'loom init' to create a configuration file first. Error: {}",
                        e
                    );
                }
            }
        }
    }

    Ok(())
}
