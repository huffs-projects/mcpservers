use anyhow::{Context, Result};
use std::env;
use std::path::PathBuf;

pub struct ConfigFinder;

impl ConfigFinder {
    /// Find default Waybar config file in common locations
    /// Checks in order:
    /// 1. ~/.config/waybar/config
    /// 2. ~/.config/waybar/config.json
    /// 3. $XDG_CONFIG_HOME/waybar/config
    /// 4. $XDG_CONFIG_HOME/waybar/config.json
    pub fn find_default_config() -> Result<Option<PathBuf>> {
        let home = env::var("HOME")
            .context("HOME environment variable not set")?;
        
        let xdg_config = env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", home));
        
        let candidates = vec![
            format!("{}/.config/waybar/config", home),
            format!("{}/.config/waybar/config.json", home),
            format!("{}/waybar/config", xdg_config),
            format!("{}/waybar/config.json", xdg_config),
        ];
        
        for candidate in candidates {
            let path = PathBuf::from(&candidate);
            if path.exists() && path.is_file() {
                return Ok(Some(path));
            }
        }
        
        Ok(None)
    }

    /// Find default Waybar CSS file in common locations
    /// Checks in order:
    /// 1. ~/.config/waybar/style.css
    /// 2. ~/.config/waybar/waybar.css
    /// 3. $XDG_CONFIG_HOME/waybar/style.css
    /// 4. $XDG_CONFIG_HOME/waybar/waybar.css
    pub fn find_default_css() -> Result<Option<PathBuf>> {
        let home = env::var("HOME")
            .context("HOME environment variable not set")?;
        
        let xdg_config = env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", home));
        
        let candidates = vec![
            format!("{}/.config/waybar/style.css", home),
            format!("{}/.config/waybar/waybar.css", home),
            format!("{}/waybar/style.css", xdg_config),
            format!("{}/waybar/waybar.css", xdg_config),
        ];
        
        for candidate in candidates {
            let path = PathBuf::from(&candidate);
            if path.exists() && path.is_file() {
                return Ok(Some(path));
            }
        }
        
        Ok(None)
    }

    /// List all available Waybar config files in common locations
    pub fn list_config_files() -> Result<Vec<PathBuf>> {
        let mut configs = Vec::new();
        
        let home = env::var("HOME")
            .context("HOME environment variable not set")?;
        
        let xdg_config = env::var("XDG_CONFIG_HOME")
            .unwrap_or_else(|_| format!("{}/.config", home));
        
        let search_dirs = vec![
            format!("{}/.config/waybar", home),
            format!("{}/waybar", xdg_config),
        ];
        
        for dir in search_dirs {
            let dir_path = PathBuf::from(&dir);
            if let Ok(entries) = std::fs::read_dir(&dir_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        let name = path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        if name == "config" || name.ends_with(".json") || name.ends_with(".css") {
                            configs.push(path);
                        }
                    }
                }
            }
        }
        
        Ok(configs)
    }
}

