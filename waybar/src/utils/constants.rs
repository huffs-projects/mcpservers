/// Constants for Waybar configuration keys and common values

/// Module array keys in Waybar config
pub const MODULES_LEFT: &str = "modules-left";
pub const MODULES_CENTER: &str = "modules-center";
pub const MODULES_RIGHT: &str = "modules-right";

/// Other common Waybar config keys
pub const LAYER: &str = "layer";
pub const POSITION: &str = "position";
pub const HEIGHT: &str = "height";
pub const WIDTH: &str = "width";
pub const SPACING: &str = "spacing";

/// All module array keys
pub const MODULE_ARRAY_KEYS: &[&str] = &[MODULES_LEFT, MODULES_CENTER, MODULES_RIGHT];

/// All top-level config keys (excluding module definitions)
pub const TOP_LEVEL_KEYS: &[&str] = &[
    MODULES_LEFT,
    MODULES_CENTER,
    MODULES_RIGHT,
    LAYER,
    POSITION,
    HEIGHT,
    WIDTH,
    SPACING,
];

/// Default Waybar config locations (in order of preference)
pub const DEFAULT_CONFIG_PATHS: &[&str] = &[
    "~/.config/waybar/config",
    "~/.config/waybar/config.json",
];

/// Default Waybar CSS locations (in order of preference)
pub const DEFAULT_CSS_PATHS: &[&str] = &[
    "~/.config/waybar/style.css",
    "~/.config/waybar/waybar.css",
];

