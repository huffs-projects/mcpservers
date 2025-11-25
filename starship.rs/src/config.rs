use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

/// Server configuration loaded from environment variables
/// 
/// All configuration values can be set via environment variables:
/// - `PORT`: Server port (default: 8080)
/// - `CACHE_TTL_SECONDS`: Cache TTL in seconds (default: 3600)
/// - `REQUEST_TIMEOUT_SECONDS`: Request timeout in seconds (default: 30)
/// - `ALLOWED_DIRECTORIES`: Comma-separated list of allowed directories for file operations
/// - `CORS_ALLOWED_ORIGINS`: Comma-separated list of allowed CORS origins (empty = allow all)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server port
    pub port: u16,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// Request timeout in seconds
    pub request_timeout_seconds: u64,
    /// Allowed directories for file operations
    pub allowed_directories: Vec<String>,
    /// CORS allowed origins (empty = allow all)
    pub cors_allowed_origins: Vec<String>,
}

impl Config {
    /// Load configuration from environment variables
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing the `Config` if successful, or an error if
    /// the `PORT` environment variable contains an invalid value.
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use starship_mcp_server::config::Config;
    /// 
    /// let config = Config::from_env()?;
    /// println!("Server will run on port {}", config.port);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn from_env() -> Result<Self> {
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .context("Invalid PORT value")?;

        let cache_ttl_seconds = env::var("CACHE_TTL_SECONDS")
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<u64>()
            .unwrap_or(3600);

        let request_timeout_seconds = env::var("REQUEST_TIMEOUT_SECONDS")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .unwrap_or(30);

        let allowed_directories = env::var("ALLOWED_DIRECTORIES")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        Ok(Config {
            port,
            cache_ttl_seconds,
            request_timeout_seconds,
            allowed_directories,
            cors_allowed_origins,
        })
    }

    /// Get cache TTL as `Duration`
    /// 
    /// # Returns
    /// 
    /// Returns the cache TTL as a `std::time::Duration`
    #[allow(dead_code)]
    pub fn cache_ttl(&self) -> Duration {
        Duration::from_secs(self.cache_ttl_seconds)
    }

    /// Get request timeout as `Duration`
    /// 
    /// # Returns
    /// 
    /// Returns the request timeout as a `std::time::Duration`
    #[allow(dead_code)]
    pub fn request_timeout(&self) -> Duration {
        Duration::from_secs(self.request_timeout_seconds)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            cache_ttl_seconds: 3600,
            request_timeout_seconds: 30,
            allowed_directories: Vec::new(),
            cors_allowed_origins: Vec::new(),
        }
    }
}

