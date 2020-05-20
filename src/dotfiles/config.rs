use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    version: u8,
    api_key: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            version: 3,
            api_key: "default".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn load_config() {
        let cfg: Config = confy::load_path("./test_assets/config.toml")
            .expect("could not load dotfiles config");

        assert_eq!(cfg.version, 1);
        assert_eq!(cfg.api_key, "config");
    }

    #[test]
    fn store_config() {
        let cfg: Config = confy::load_path("./test_assets/config.toml")
            .expect("could not load dotfiles config");

        confy::store_path("./test_assets/stored_config.toml", &cfg)
            .expect("could not load dotfiles config");

        let cfg_stored: Config = confy::load_path("./test_assets/stored_config.toml")
            .expect("could not load dotfiles config");

        assert_eq!(cfg_stored.version, cfg.version);
        assert_eq!(cfg_stored.api_key, cfg.api_key);
    }

    #[test]
    fn default_config() {
        let cfg: Config = confy::load_path("./test_assets/default_config.toml")
            .expect("could not load dotfiles config");

        fs::remove_file("./test_assets/default_config.toml")
            .expect("could not remove file");

        assert_eq!(cfg.version, 3);
        assert_eq!(cfg.api_key, "default");
    }
}