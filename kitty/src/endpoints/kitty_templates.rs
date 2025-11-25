use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TemplatesQuery {
    pub category: Option<String>,
    pub use_case: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Template {
    pub template_name: String,
    pub snippet: String,
    pub description: String,
    pub documentation_url: String,
}

pub async fn handle_kitty_templates(query: TemplatesQuery) -> Vec<Template> {
    let templates = get_kitty_templates();
    
    templates
        .into_iter()
        .filter(|t| {
            let matches_category = query.category
                .as_ref()
                .map(|cat| t.template_name.to_lowercase().contains(&cat.to_lowercase()))
                .unwrap_or(true);
            let matches_use_case = query.use_case
                .as_ref()
                .map(|uc| t.description.to_lowercase().contains(&uc.to_lowercase()))
                .unwrap_or(true);
            matches_category && matches_use_case
        })
        .collect()
}

fn get_kitty_templates() -> Vec<Template> {
    vec![
        Template {
            template_name: "Font Configuration".to_string(),
            snippet: r#"# Font configuration
font_family      JetBrains Mono
font_size        12.0
bold_font        auto
italic_font      auto
bold_italic_font auto"#.to_string(),
            description: "Font and Unicode settings template".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.font_family".to_string(),
        },
        Template {
            template_name: "Performance Tuning".to_string(),
            snippet: r#"# Performance tuning
repaint_delay    10
input_delay      3
sync_to_monitor  yes
window_margin_width 0"#.to_string(),
            description: "Performance tuning template for repaint_delay, sync_to_monitor, input delay".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.repaint_delay".to_string(),
        },
        Template {
            template_name: "Layout Management".to_string(),
            snippet: r#"# Layout management
enabled_layouts tall,stack,fat,grid

# Layout switching
map ctrl+shift+l next_layout"#.to_string(),
            description: "Layout management template for stack, tall, fat, grid".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.enabled_layouts".to_string(),
        },
        Template {
            template_name: "Kittens Configuration".to_string(),
            snippet: r#"# Kittens configuration
# Hyperlinked URLs
map ctrl+shift+e kitten hyperlinked_grep

# Image preview
map ctrl+shift+i kitten icat

# Diff viewer
map ctrl+shift+d kitten diff"#.to_string(),
            description: "Kittens template for hyperlinked URLs, image preview, diff viewer".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/kittens/".to_string(),
        },
        Template {
            template_name: "Keybindings".to_string(),
            snippet: r#"# Keybindings
map ctrl+shift+enter new_window
map ctrl+shift+t new_tab
map ctrl+shift+w close_window
map ctrl+shift+j next_window
map ctrl+shift+k previous_window"#.to_string(),
            description: "Keybindings template for window and tab management".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.action".to_string(),
        },
        Template {
            template_name: "Window Defaults".to_string(),
            snippet: r#"# Window/tab/session defaults
window_padding_width 5
window_margin_width 10
remember_window_size yes
initial_window_width 640
initial_window_height 400"#.to_string(),
            description: "Window/tab/session defaults template".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.window_padding_width".to_string(),
        },
    ]
}

