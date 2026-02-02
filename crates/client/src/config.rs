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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_create_config() {
        let cfg = create_config("http://localhost".to_string(), "token123".to_string());
        assert_eq!(cfg.server_url, "http://localhost");
        assert_eq!(cfg.token, "token123");
        assert!(!cfg.device_id.is_empty());
        assert!(cfg.games.is_empty());
    }

    #[test]
    fn test_generate_device_id_unique() {
        let id1 = generate_device_id();
        let id2 = generate_device_id();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_generate_device_id_format() {
        let id = generate_device_id();
        assert!(uuid::Uuid::parse_str(&id).is_ok());
    }

    #[test]
    fn test_config_default() {
        let cfg = Config::default();
        assert!(!cfg.device_id.is_empty());
        assert_eq!(cfg.server_url, "");
        assert_eq!(cfg.token, "");
        assert!(cfg.games.is_empty());
    }

    #[test]
    fn test_config_serialization() {
        let cfg = create_config("http://test".to_string(), "token".to_string());
        let json = serde_json::to_string(&cfg).unwrap();
        let restored: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.server_url, cfg.server_url);
        assert_eq!(restored.token, cfg.token);
        assert_eq!(restored.device_id, cfg.device_id);
    }

    #[test]
    fn test_save_and_load_config() {
        let server_url = "http://localhost:8080".to_string();
        let token = "test_token".to_string();

        save_default_config(server_url.clone(), token.clone()).unwrap();
        let cfg = load_config().unwrap();

        assert_eq!(cfg.server_url, server_url);
        assert_eq!(cfg.token, token);
        assert!(cfg.games.is_empty());
    }
}
