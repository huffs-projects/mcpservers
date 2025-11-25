use crate::models::WaybarModuleOption;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub struct WaybarSchema;

// Cache the schema map to avoid rebuilding on every call
static SCHEMA_CACHE: Lazy<HashMap<String, Vec<WaybarModuleOption>>> = Lazy::new(|| {
    WaybarSchema::build_all_modules()
});

impl WaybarSchema {
    /// Get all modules with cached schema (computed once at startup)
    pub fn get_all_modules() -> &'static HashMap<String, Vec<WaybarModuleOption>> {
        &SCHEMA_CACHE
    }

    /// Build the module schema map (called once by lazy static)
    fn build_all_modules() -> HashMap<String, Vec<WaybarModuleOption>> {
        let mut modules = HashMap::new();

        // Battery module
        modules.insert("battery".to_string(), Self::battery_options());
        
        // CPU module
        modules.insert("cpu".to_string(), Self::cpu_options());
        
        // Memory module
        modules.insert("memory".to_string(), Self::memory_options());
        
        // Network module
        modules.insert("network".to_string(), Self::network_options());
        
        // Clock module
        modules.insert("clock".to_string(), Self::clock_options());
        
        // Tray module
        modules.insert("tray".to_string(), Self::tray_options());
        
        // Custom module
        modules.insert("custom".to_string(), Self::custom_options());
        
        // Exec module
        modules.insert("exec".to_string(), Self::exec_options());
        
        // Idle inhibitor module
        modules.insert("idle_inhibitor".to_string(), Self::idle_inhibitor_options());
        
        // Pulseaudio module
        modules.insert("pulseaudio".to_string(), Self::pulseaudio_options());
        
        // Backlight module
        modules.insert("backlight".to_string(), Self::backlight_options());
        
        // Disk module
        modules.insert("disk".to_string(), Self::disk_options());
        
        // Temperature module
        modules.insert("temperature".to_string(), Self::temperature_options());
        
        // Window module (for Sway/Wayland)
        modules.insert("window".to_string(), Self::window_options());
        
        // Workspaces module (for Sway/Wayland)
        modules.insert("workspaces".to_string(), Self::workspaces_options());
        
        // MPD module
        modules.insert("mpd".to_string(), Self::mpd_options());
        
        // Bluetooth module
        modules.insert("bluetooth".to_string(), Self::bluetooth_options());

        modules
    }

    fn battery_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "battery".to_string(),
                "bat".to_string(),
                "string".to_string(),
                false,
                "Battery name (e.g., 'BAT0')".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "battery".to_string(),
                "adapter".to_string(),
                "string".to_string(),
                false,
                "Adapter name".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "battery".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for battery display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{capacity}%".to_string()),
            WaybarModuleOption::new(
                "battery".to_string(),
                "format-alt".to_string(),
                "string".to_string(),
                false,
                "Alternative format string".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "battery".to_string(),
                "states".to_string(),
                "object".to_string(),
                false,
                "State-specific format strings".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "battery".to_string(),
                "interval".to_string(),
                "integer".to_string(),
                false,
                "Update interval in seconds".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("60".to_string()),
        ]
    }

    fn cpu_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "cpu".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for CPU display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{usage}%".to_string()),
            WaybarModuleOption::new(
                "cpu".to_string(),
                "interval".to_string(),
                "integer".to_string(),
                false,
                "Update interval in seconds".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("2".to_string()),
        ]
    }

    fn memory_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "memory".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for memory display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{}%".to_string()),
            WaybarModuleOption::new(
                "memory".to_string(),
                "interval".to_string(),
                "integer".to_string(),
                false,
                "Update interval in seconds".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("2".to_string()),
        ]
    }

    fn network_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "network".to_string(),
                "interface".to_string(),
                "string".to_string(),
                false,
                "Network interface name".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "network".to_string(),
                "format-wifi".to_string(),
                "string".to_string(),
                false,
                "Format string for WiFi display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{essid} ({signalStrength}%)".to_string()),
            WaybarModuleOption::new(
                "network".to_string(),
                "format-ethernet".to_string(),
                "string".to_string(),
                false,
                "Format string for Ethernet display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{ifname}: {ipaddr}".to_string()),
            WaybarModuleOption::new(
                "network".to_string(),
                "format-disconnected".to_string(),
                "string".to_string(),
                false,
                "Format string when disconnected".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("Disconnected".to_string()),
            WaybarModuleOption::new(
                "network".to_string(),
                "interval".to_string(),
                "integer".to_string(),
                false,
                "Update interval in seconds".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("5".to_string()),
        ]
    }

    fn clock_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "clock".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for clock display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{:%Y-%m-%d %H:%M}".to_string()),
            WaybarModuleOption::new(
                "clock".to_string(),
                "format-alt".to_string(),
                "string".to_string(),
                false,
                "Alternative format string".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "clock".to_string(),
                "timezone".to_string(),
                "string".to_string(),
                false,
                "Timezone for clock display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "clock".to_string(),
                "interval".to_string(),
                "integer".to_string(),
                false,
                "Update interval in seconds".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("60".to_string()),
        ]
    }

    fn tray_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "tray".to_string(),
                "icon-size".to_string(),
                "integer".to_string(),
                false,
                "Size of tray icons in pixels".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("21".to_string()),
            WaybarModuleOption::new(
                "tray".to_string(),
                "spacing".to_string(),
                "integer".to_string(),
                false,
                "Spacing between tray icons".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("10".to_string()),
        ]
    }

    fn custom_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "custom".to_string(),
                "exec".to_string(),
                "string".to_string(),
                true,
                "Command to execute".to_string(),
                "https://waybar.org/can-i-add-custom-scripts-to-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "custom".to_string(),
                "interval".to_string(),
                "integer".to_string(),
                false,
                "Update interval in seconds (repeating)".to_string(),
                "https://waybar.org/can-i-add-custom-scripts-to-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "custom".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for output".to_string(),
                "https://waybar.org/can-i-add-custom-scripts-to-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "custom".to_string(),
                "return-type".to_string(),
                "string".to_string(),
                false,
                "Return type: 'json' or 'text'".to_string(),
                "https://waybar.org/can-i-add-custom-scripts-to-waybar/".to_string(),
            ).with_default("json".to_string()),
            WaybarModuleOption::new(
                "custom".to_string(),
                "tooltip".to_string(),
                "boolean".to_string(),
                false,
                "Enable tooltip".to_string(),
                "https://waybar.org/can-i-add-custom-scripts-to-waybar/".to_string(),
            ).with_default("true".to_string()),
        ]
    }

    fn exec_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "exec".to_string(),
                "exec".to_string(),
                "string".to_string(),
                true,
                "Command to execute once".to_string(),
                "https://waybar.org/can-i-add-custom-scripts-to-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "exec".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for output".to_string(),
                "https://waybar.org/can-i-add-custom-scripts-to-waybar/".to_string(),
            ),
        ]
    }

    fn idle_inhibitor_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "idle_inhibitor".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string when active".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "idle_inhibitor".to_string(),
                "format-icons".to_string(),
                "object".to_string(),
                false,
                "Icons for different states".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
        ]
    }

    fn pulseaudio_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "pulseaudio".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for volume display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{volume}%".to_string()),
            WaybarModuleOption::new(
                "pulseaudio".to_string(),
                "format-muted".to_string(),
                "string".to_string(),
                false,
                "Format string when muted".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("Muted".to_string()),
            WaybarModuleOption::new(
                "pulseaudio".to_string(),
                "format-source".to_string(),
                "string".to_string(),
                false,
                "Format string for source".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "pulseaudio".to_string(),
                "format-icons".to_string(),
                "object".to_string(),
                false,
                "Icons for different volume levels".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "pulseaudio".to_string(),
                "scroll-step".to_string(),
                "integer".to_string(),
                false,
                "Volume change step on scroll".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("5".to_string()),
        ]
    }

    fn backlight_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "backlight".to_string(),
                "device".to_string(),
                "string".to_string(),
                false,
                "Backlight device name".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "backlight".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for backlight display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{percent}%".to_string()),
            WaybarModuleOption::new(
                "backlight".to_string(),
                "format-icons".to_string(),
                "object".to_string(),
                false,
                "Icons for different brightness levels".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
        ]
    }

    fn disk_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "disk".to_string(),
                "path".to_string(),
                "string".to_string(),
                false,
                "Disk path to monitor".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("/".to_string()),
            WaybarModuleOption::new(
                "disk".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for disk display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{used} / {total} ({percentage}%)".to_string()),
            WaybarModuleOption::new(
                "disk".to_string(),
                "interval".to_string(),
                "integer".to_string(),
                false,
                "Update interval in seconds".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("30".to_string()),
        ]
    }

    fn temperature_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "temperature".to_string(),
                "thermal-zone".to_string(),
                "integer".to_string(),
                false,
                "Thermal zone number".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("0".to_string()),
            WaybarModuleOption::new(
                "temperature".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for temperature display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{temperatureC}°C".to_string()),
            WaybarModuleOption::new(
                "temperature".to_string(),
                "critical-threshold".to_string(),
                "integer".to_string(),
                false,
                "Critical temperature threshold".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("80".to_string()),
        ]
    }

    fn window_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "window".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for window title".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{}".to_string()),
            WaybarModuleOption::new(
                "window".to_string(),
                "max-length".to_string(),
                "integer".to_string(),
                false,
                "Maximum length of window title".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("50".to_string()),
        ]
    }

    fn workspaces_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "workspaces".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for workspace display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{name}".to_string()),
            WaybarModuleOption::new(
                "workspaces".to_string(),
                "format-icons".to_string(),
                "object".to_string(),
                false,
                "Icons for different workspace states".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ),
            WaybarModuleOption::new(
                "workspaces".to_string(),
                "disable-scroll".to_string(),
                "boolean".to_string(),
                false,
                "Disable scrolling through workspaces".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("false".to_string()),
        ]
    }

    fn mpd_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "mpd".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for MPD display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{stateIcon} {artist} - {title}".to_string()),
            WaybarModuleOption::new(
                "mpd".to_string(),
                "format-disconnected".to_string(),
                "string".to_string(),
                false,
                "Format string when disconnected".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("Disconnected".to_string()),
            WaybarModuleOption::new(
                "mpd".to_string(),
                "interval".to_string(),
                "integer".to_string(),
                false,
                "Update interval in seconds".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("2".to_string()),
        ]
    }

    fn bluetooth_options() -> Vec<WaybarModuleOption> {
        vec![
            WaybarModuleOption::new(
                "bluetooth".to_string(),
                "format".to_string(),
                "string".to_string(),
                false,
                "Format string for Bluetooth display".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("{num_connections} connected".to_string()),
            WaybarModuleOption::new(
                "bluetooth".to_string(),
                "format-disabled".to_string(),
                "string".to_string(),
                false,
                "Format string when disabled".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("Disabled".to_string()),
            WaybarModuleOption::new(
                "bluetooth".to_string(),
                "format-off".to_string(),
                "string".to_string(),
                false,
                "Format string when off".to_string(),
                "https://waybar.org/what-modules-come-built-in-with-waybar/".to_string(),
            ).with_default("Off".to_string()),
        ]
    }
}

