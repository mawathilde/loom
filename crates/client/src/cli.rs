use crate::config;
use crate::scanner::scan_game_dir;
use clap::Parser;
use dialoguer::{Input, Password};
use serde_json::to_string_pretty;

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
    Scan {},
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
            let config = config::get_config();
            match config {
                Ok(cfg) => {
                    if cfg.games.is_empty() {
                        println!("No games configured.");
                    } else {
                        println!("Configured games:");
                        for game in cfg.games {
                            println!("ID: {}, Name: {}, Path: {}", game.id, game.name, game.path);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        Commands::Scan {} => {
            let config = config::get_config();
            match config {
                Ok(cfg) => {
                    for game in cfg.games {
                        let path = std::path::Path::new(&game.path);
                        let game_manifest = scan_game_dir(game.id.clone(), path);
                        println!("{}", to_string_pretty(&game_manifest)?);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }

    Ok(())
}
