use crate::utils::config_locator;

/// Returns Wofi config search paths in priority order
pub fn get_config_locations() -> Vec<String> {
    config_locator::get_config_locations()
        .into_iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect()
}

