use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub cache: CacheConfig,
    #[serde(default)]
    pub timeouts: TimeoutConfig,
    #[serde(default)]
    pub rate_limit: RateLimitConfig,
    #[serde(default)]
    pub paths: PathConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    #[serde(default = "default_cache_ttl")]
    pub ttl_seconds: u64,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            ttl_seconds: default_cache_ttl(),
        }
    }
}

fn default_cache_ttl() -> u64 { 3600 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    #[serde(default = "default_options_timeout")]
    pub options_query_seconds: u64,
    #[serde(default = "default_modules_timeout")]
    pub modules_list_seconds: u64,
    #[serde(default = "default_templates_timeout")]
    pub templates_seconds: u64,
    #[serde(default = "default_build_timeout")]
    pub build_seconds: u64,
    #[serde(default = "default_patch_timeout")]
    pub patch_seconds: u64,
    #[serde(default = "default_health_timeout")]
    pub health_seconds: u64,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            options_query_seconds: default_options_timeout(),
            modules_list_seconds: default_modules_timeout(),
            templates_seconds: default_templates_timeout(),
            build_seconds: default_build_timeout(),
            patch_seconds: default_patch_timeout(),
            health_seconds: default_health_timeout(),
        }
    }
}

fn default_options_timeout() -> u64 { 30 }
fn default_modules_timeout() -> u64 { 30 }
fn default_templates_timeout() -> u64 { 10 }
fn default_build_timeout() -> u64 { 600 }
fn default_patch_timeout() -> u64 { 30 }
fn default_health_timeout() -> u64 { 10 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_rate_limit")]
    pub requests_per_second: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            requests_per_second: default_rate_limit(),
        }
    }
}

fn default_rate_limit() -> u32 { 100 }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    pub home_manager_docs: Option<String>,
    pub home_manager_modules: Option<String>,
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            home_manager_docs: None,
            home_manager_modules: None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cache: CacheConfig {
                ttl_seconds: default_cache_ttl(),
            },
            timeouts: TimeoutConfig {
                options_query_seconds: default_options_timeout(),
                modules_list_seconds: default_modules_timeout(),
                templates_seconds: default_templates_timeout(),
                build_seconds: default_build_timeout(),
                patch_seconds: default_patch_timeout(),
                health_seconds: default_health_timeout(),
            },
            rate_limit: RateLimitConfig {
                enabled: false,
                requests_per_second: default_rate_limit(),
            },
            paths: PathConfig {
                home_manager_docs: None,
                home_manager_modules: None,
            },
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        if let Some(config_path) = path {
            if config_path.exists() {
                let content = fs::read_to_string(config_path)
                    .context("Failed to read config file")?;
                let config: Config = toml::from_str(&content)
                    .context("Failed to parse config file")?;
                return Ok(config);
            }
        }

        // Try default locations
        let default_paths = vec![
            "config.toml",
            "~/.config/home-manager-mcp/config.toml",
            "/etc/home-manager-mcp/config.toml",
        ];

        for path_str in default_paths {
            let expanded = shellexpand::full(path_str)
                .map(|s| s.into_owned())
                .unwrap_or_else(|_| path_str.to_string());
            
            let path = Path::new(&expanded);
            if path.exists() {
                let content = fs::read_to_string(path)
                    .context("Failed to read config file")?;
                let config: Config = toml::from_str(&content)
                    .context("Failed to parse config file")?;
                return Ok(config);
            }
        }

        Ok(Config::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.cache.ttl_seconds, 3600);
        assert_eq!(config.timeouts.build_seconds, 600);
        assert!(!config.rate_limit.enabled);
    }

    #[test]
    fn test_load_config_from_file() {
        let temp_file = NamedTempFile::new().unwrap();
        let content = r#"
[cache]
ttl_seconds = 7200

[timeouts]
options_query_seconds = 30
modules_list_seconds = 30
templates_seconds = 10
build_seconds = 1200
patch_seconds = 30
health_seconds = 10

[rate_limit]
enabled = true
requests_per_second = 50
"#;
        fs::write(temp_file.path(), content).unwrap();
        
        let config = Config::load(Some(temp_file.path())).unwrap();
        assert_eq!(config.cache.ttl_seconds, 7200);
        assert_eq!(config.timeouts.build_seconds, 1200);
        assert!(config.rate_limit.enabled);
        assert_eq!(config.rate_limit.requests_per_second, 50);
    }
}

