use crate::utils::nix;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub nix_installed: bool,
    pub home_manager_installed: bool,
    pub version: String,
}

pub async fn check_health() -> Result<HealthStatus> {
    let nix_check = timeout(Duration::from_secs(5), nix::check_nix_installed())
        .await
        .unwrap_or(false);
    
    let hm_check = timeout(Duration::from_secs(5), nix::check_home_manager_installed())
        .await
        .unwrap_or(false);

    let status = if nix_check && hm_check {
        "healthy".to_string()
    } else {
        "degraded".to_string()
    };

    Ok(HealthStatus {
        status,
        nix_installed: nix_check,
        home_manager_installed: hm_check,
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_health() {
        let health = check_health().await.unwrap();
        assert!(!health.status.is_empty());
        assert_eq!(health.version, env!("CARGO_PKG_VERSION"));
    }
}

