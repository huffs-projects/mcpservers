use crate::models::KittyOption;
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::Arc;

pub struct KittySchema {
    options: HashMap<String, KittyOption>,
}

// Global schema instance (initialized once, reused across requests)
static SCHEMA: Lazy<Arc<KittySchema>> = Lazy::new(|| {
    let mut schema = KittySchema {
        options: HashMap::new(),
    };
    schema.initialize_options();
    Arc::new(schema)
});

impl KittySchema {
    /// Get the global schema instance (singleton)
    /// 
    /// Returns a shared, cached instance of the schema that is initialized once
    /// and reused across all requests. This improves performance by avoiding
    /// repeated schema initialization.
    /// 
    /// # Example
    /// ```
    /// use kitty_mcp_server::utils::KittySchema;
    /// 
    /// let schema = KittySchema::global();
    /// let options = schema.get_all_options();
    /// ```
    pub fn global() -> Arc<KittySchema> {
        SCHEMA.clone()
    }
    
    /// Create a new schema instance
    /// 
    /// Note: For better performance, prefer using `KittySchema::global()` instead
    /// of creating new instances.
    pub fn new() -> Self {
        let mut schema = Self {
            options: HashMap::new(),
        };
        schema.initialize_options();
        schema
    }

    pub fn is_valid_option(&self, name: &str) -> bool {
        self.options.contains_key(name)
    }

    pub fn get_option(&self, name: &str) -> Option<&KittyOption> {
        self.options.get(name)
    }

    pub fn get_all_options(&self) -> Vec<&KittyOption> {
        self.options.values().collect()
    }

    pub fn search_options(&self, search_term: &str, category: Option<&str>) -> Vec<&KittyOption> {
        let search_lower = search_term.to_lowercase();
        self.options
            .values()
            .filter(|opt| {
                let matches_search = opt.name.to_lowercase().contains(&search_lower)
                    || opt.description.to_lowercase().contains(&search_lower);
                let matches_category = category
                    .map(|cat| opt.category.to_lowercase() == cat.to_lowercase())
                    .unwrap_or(true);
                matches_search && matches_category
            })
            .collect()
    }

    fn initialize_options(&mut self) {
        // Font options
        self.add_option(KittyOption {
            name: "font_family".to_string(),
            option_type: "string".to_string(),
            default: Some("monospace".to_string()),
            category: "Fonts".to_string(),
            description: "Font family to use".to_string(),
            example: Some("JetBrains Mono".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.font_family".to_string(),
        });

        self.add_option(KittyOption {
            name: "font_size".to_string(),
            option_type: "float".to_string(),
            default: Some("11.0".to_string()),
            category: "Fonts".to_string(),
            description: "Font size in points".to_string(),
            example: Some("12.0".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.font_size".to_string(),
        });

        // Window options
        self.add_option(KittyOption {
            name: "window_padding_width".to_string(),
            option_type: "float".to_string(),
            default: Some("0.0".to_string()),
            category: "Window".to_string(),
            description: "Padding around window content".to_string(),
            example: Some("5.0".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.window_padding_width".to_string(),
        });

        self.add_option(KittyOption {
            name: "window_margin_width".to_string(),
            option_type: "float".to_string(),
            default: Some("0.0".to_string()),
            category: "Window".to_string(),
            description: "Margin around window".to_string(),
            example: Some("10.0".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.window_margin_width".to_string(),
        });

        // Performance options
        self.add_option(KittyOption {
            name: "repaint_delay".to_string(),
            option_type: "integer".to_string(),
            default: Some("10".to_string()),
            category: "Performance".to_string(),
            description: "Delay between repaints in milliseconds".to_string(),
            example: Some("8".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.repaint_delay".to_string(),
        });

        self.add_option(KittyOption {
            name: "sync_to_monitor".to_string(),
            option_type: "boolean".to_string(),
            default: Some("yes".to_string()),
            category: "Performance".to_string(),
            description: "Sync rendering to monitor refresh rate".to_string(),
            example: Some("yes".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.sync_to_monitor".to_string(),
        });

        // Layout options
        self.add_option(KittyOption {
            name: "enabled_layouts".to_string(),
            option_type: "list".to_string(),
            default: Some("tall,stack".to_string()),
            category: "Layouts".to_string(),
            description: "Enabled layout algorithms".to_string(),
            example: Some("tall,stack,fat,grid".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.enabled_layouts".to_string(),
        });

        // Mouse options
        self.add_option(KittyOption {
            name: "mouse_hide_wait".to_string(),
            option_type: "float".to_string(),
            default: Some("3.0".to_string()),
            category: "Mouse".to_string(),
            description: "Hide mouse cursor after specified seconds".to_string(),
            example: Some("2.0".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.mouse_hide_wait".to_string(),
        });

        self.add_option(KittyOption {
            name: "url_color".to_string(),
            option_type: "color".to_string(),
            default: Some("#0087bd".to_string()),
            category: "Colors".to_string(),
            description: "Color for URLs".to_string(),
            example: Some("#0066cc".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.url_color".to_string(),
        });

        self.add_option(KittyOption {
            name: "url_style".to_string(),
            option_type: "string".to_string(),
            default: Some("curly".to_string()),
            category: "Colors".to_string(),
            description: "Style for URLs (curly, straight, double, dotted, dashed)".to_string(),
            example: Some("straight".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.url_style".to_string(),
        });

        // Tab bar options
        self.add_option(KittyOption {
            name: "tab_bar_edge".to_string(),
            option_type: "string".to_string(),
            default: Some("bottom".to_string()),
            category: "Tabs".to_string(),
            description: "Tab bar position (top, bottom, left, right)".to_string(),
            example: Some("top".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.tab_bar_edge".to_string(),
        });

        self.add_option(KittyOption {
            name: "tab_bar_style".to_string(),
            option_type: "string".to_string(),
            default: Some("fade".to_string()),
            category: "Tabs".to_string(),
            description: "Tab bar style (fade, separator, powerline, hidden)".to_string(),
            example: Some("powerline".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.tab_bar_style".to_string(),
        });

        // Scrollback options
        self.add_option(KittyOption {
            name: "scrollback_lines".to_string(),
            option_type: "integer".to_string(),
            default: Some("2000".to_string()),
            category: "Scrollback".to_string(),
            description: "Number of lines of scrollback to keep".to_string(),
            example: Some("5000".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.scrollback_lines".to_string(),
        });

        self.add_option(KittyOption {
            name: "scrollback_pager".to_string(),
            option_type: "string".to_string(),
            default: Some("less".to_string()),
            category: "Scrollback".to_string(),
            description: "Program to use for viewing scrollback".to_string(),
            example: Some("less -R".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.scrollback_pager".to_string(),
        });

        // Bell options
        self.add_option(KittyOption {
            name: "enable_audio_bell".to_string(),
            option_type: "boolean".to_string(),
            default: Some("yes".to_string()),
            category: "Bell".to_string(),
            description: "Enable audio bell".to_string(),
            example: Some("no".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.enable_audio_bell".to_string(),
        });

        self.add_option(KittyOption {
            name: "visual_bell_duration".to_string(),
            option_type: "float".to_string(),
            default: Some("0.0".to_string()),
            category: "Bell".to_string(),
            description: "Visual bell duration in seconds".to_string(),
            example: Some("0.5".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.visual_bell_duration".to_string(),
        });

        // Window title options
        self.add_option(KittyOption {
            name: "window_title".to_string(),
            option_type: "string".to_string(),
            default: None,
            category: "Window".to_string(),
            description: "Window title template".to_string(),
            example: Some("{title}".to_string()),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.window_title".to_string(),
        });

        // Add more options as needed - this is a representative sample
        // In a full implementation, you would load from official Kitty documentation
    }

    fn add_option(&mut self, option: KittyOption) {
        self.options.insert(option.name.clone(), option);
    }
}

impl Default for KittySchema {
    fn default() -> Self {
        Self::new()
    }
}

