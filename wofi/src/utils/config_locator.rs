use std::path::{Path, PathBuf};
use anyhow::Result;

/// Returns Wofi config search paths in priority order based on official sources:
/// - $XDG_CONFIG_HOME/wofi/config
/// - ~/.config/wofi/config
/// - /etc/xdg/wofi/config
/// - /usr/share/wofi/config (fallback)
pub fn get_config_locations() -> Vec<PathBuf> {
    let mut locations = Vec::new();

    // $XDG_CONFIG_HOME/wofi/config
    if let Ok(xdg_config) = std::env::var("XDG_CONFIG_HOME") {
        locations.push(PathBuf::from(xdg_config).join("wofi").join("config"));
    }

    // ~/.config/wofi/config
    if let Ok(home) = std::env::var("HOME") {
        locations.push(PathBuf::from(home).join(".config").join("wofi").join("config"));
    }

    // /etc/xdg/wofi/config
    locations.push(PathBuf::from("/etc/xdg/wofi/config"));

    // /usr/share/wofi/config (fallback)
    locations.push(PathBuf::from("/usr/share/wofi/config"));

    locations
}

/// Find the first existing config file in the search path
pub fn find_config() -> Option<PathBuf> {
    get_config_locations()
        .into_iter()
        .find(|path| path.exists())
}

/// Get CSS file path for a given config path
pub fn get_css_path(config_path: &Path) -> PathBuf {
    config_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("style.css")
}

