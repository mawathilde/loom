use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub device_id: String,
    pub server_url: String,
    pub token: String,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: String,
    pub path: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let path = confy::get_configuration_file_path("loom", "config")?;
    if !path.exists() {
        return Err(format!("Config file not found at {:?}", path).into());
    }
    let cfg: Config = confy::load_path(&path)?;
    Ok(cfg)
}

pub fn save_default_config(
    server_url: String,
    token: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let cfg = create_config(server_url, token);
    confy::store("loom", "config", cfg)?;
    Ok(())
}

fn create_config(server_url: String, token: String) -> Config {
    Config {
        device_id: generate_device_id(),
        server_url,
        token,
        games: vec![],
    }
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Config {
            device_id: generate_device_id(),
            server_url: "".to_string(),
            token: "".to_string(),
            games: vec![],
        }
    }
}

fn generate_device_id() -> String {
    Uuid::new_v4().to_string()
}
