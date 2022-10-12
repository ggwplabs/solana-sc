//! CLI Client configuration
use serde::{Deserialize, Serialize};
use std::fs::File;

/// Custom CLI Config structure
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CLIConfig {
    pub fee_payer_path: String,
    pub network: String,
    pub programs: Programs,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Programs {
    pub gpass: String,
    pub freezing: String,
    pub staking: String,
    pub distribution: String,
    pub reward_distribution: String,
    pub fighting: String,
}

impl Default for CLIConfig {
    fn default() -> Self {
        let fee_payer_path = {
            let mut fee_payer_path = dirs_next::home_dir().expect("home directory");
            fee_payer_path.extend(&[".config", "solana", "id.json"]);
            fee_payer_path.to_str().unwrap().to_string()
        };
        let network = "localnet".to_string();
        let programs = Programs {
            gpass: gpass::id().to_string(),
            freezing: freezing::id().to_string(),
            staking: staking::id().to_string(),
            distribution: distribution::id().to_string(),
            reward_distribution: reward_distribution::id().to_string(),
            fighting: fighting::id().to_string(),
        };

        Self {
            fee_payer_path,
            network,
            programs,
        }
    }
}

impl CLIConfig {
    /// Loading CLI Config from file
    pub fn load(config_file: &str) -> Result<Self, std::io::Error> {
        let file = File::open(config_file)?;
        let config: Self = serde_json::from_reader(file)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", err)))?;
        Ok(config)
    }
}
