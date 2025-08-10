use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub stealth: StealthConfig,
    pub networking: NetworkingConfig,
    pub ai: AiConfig,
    pub output: OutputConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub log_level: String,
    pub max_concurrent_tasks: u32,
    pub default_timeout_seconds: u64,
    pub user_agent: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StealthConfig {
    pub enabled: bool,
    pub ip_rotation_interval_seconds: u64,
    pub random_delays: bool,
    pub min_delay_ms: u64,
    pub max_delay_ms: u64,
    pub fingerprint_randomization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingConfig {
    pub max_retries: u32,
    pub proxy_rotation: bool,
    pub respect_robots_txt: bool,
    pub max_requests_per_minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    pub model_path: Option<String>,
    pub confidence_threshold: f32,
    pub max_context_length: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub default_format: String,
    pub default_directory: String,
    pub include_metadata: bool,
    pub compress_results: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: GeneralConfig {
                log_level: "info".to_string(),
                max_concurrent_tasks: 10,
                default_timeout_seconds: 30,
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36".to_string(),
            },
            stealth: StealthConfig {
                enabled: true,
                ip_rotation_interval_seconds: 300, // 5 minutes
                random_delays: true,
                min_delay_ms: 1000,
                max_delay_ms: 5000,
                fingerprint_randomization: true,
            },
            networking: NetworkingConfig {
                max_retries: 3,
                proxy_rotation: true,
                respect_robots_txt: true,
                max_requests_per_minute: 60,
            },
            ai: AiConfig {
                model_path: None,
                confidence_threshold: 0.7,
                max_context_length: 4096,
            },
            output: OutputConfig {
                default_format: "csv".to_string(),
                default_directory: "downloads/flash-ai".to_string(),
                include_metadata: true,
                compress_results: false,
            },
        }
    }
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        if path.as_ref().exists() {
            let content = std::fs::read_to_string(path)?;
            let config: Config = toml::from_str(&content)?;
            Ok(config)
        } else {
            // Create default config file
            let default_config = Config::default();
            let content = toml::to_string_pretty(&default_config)?;
            std::fs::write(path, content)?;
            Ok(default_config)
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
