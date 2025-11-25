use crate::models::KittyTheme;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ThemingQuery {
    pub theme_name: Option<String>,
}

pub async fn handle_kitty_theming(query: ThemingQuery) -> Vec<KittyTheme> {
    let themes = get_kitty_themes();
    
    if let Some(name) = &query.theme_name {
        themes
            .into_iter()
            .filter(|t| t.theme_name.to_lowercase() == name.to_lowercase())
            .collect()
    } else {
        themes
    }
}

fn get_kitty_themes() -> Vec<KittyTheme> {
    vec![
        KittyTheme {
            theme_name: "Default Dark".to_string(),
            snippet: r#"# Default Dark Theme
background #1e1e1e
foreground #d4d4d4
cursor #aeafad
selection_background #264f78
color0 #000000
color1 #cd3131
color2 #0dbc79
color3 #e5e510
color4 #2472c8
color5 #bc3fbc
color6 #11a8cd
color7 #e5e5e5
color8 #666666
color9 #f14c4c
color10 #23d18b
color11 #f5f543
color12 #3b8eea
color13 #d670d6
color14 #29b8db
color15 #e5e5e5"#.to_string(),
            description: "Default dark theme with good contrast".to_string(),
            palette: {
                let mut map = HashMap::new();
                map.insert("background".to_string(), "#1e1e1e".to_string());
                map.insert("foreground".to_string(), "#d4d4d4".to_string());
                map.insert("cursor".to_string(), "#aeafad".to_string());
                map
            },
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#color-scheme".to_string(),
        },
        KittyTheme {
            theme_name: "Solarized Dark".to_string(),
            snippet: r#"# Solarized Dark Theme
background #002b36
foreground #839496
cursor #839496
selection_background #073642
color0 #073642
color1 #dc322f
color2 #859900
color3 #b58900
color4 #268bd2
color5 #d33682
color6 #2aa198
color7 #eee8d5
color8 #002b36
color9 #cb4b16
color10 #586e75
color11 #657b83
color12 #839496
color13 #6c71c4
color14 #93a1a1
color15 #fdf6e3"#.to_string(),
            description: "Solarized dark color scheme".to_string(),
            palette: {
                let mut map = HashMap::new();
                map.insert("background".to_string(), "#002b36".to_string());
                map.insert("foreground".to_string(), "#839496".to_string());
                map.insert("cursor".to_string(), "#839496".to_string());
                map
            },
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#color-scheme".to_string(),
        },
        KittyTheme {
            theme_name: "Nord".to_string(),
            snippet: r#"# Nord Theme
background #2e3440
foreground #d8dee9
cursor #d8dee9
selection_background #3b4252
color0 #3b4252
color1 #bf616a
color2 #a3be8c
color3 #ebcb8b
color4 #81a1c1
color5 #b48ead
color6 #8fbcbb
color7 #e5e9f0
color8 #4c566a
color9 #bf616a
color10 #a3be8c
color11 #ebcb8b
color12 #81a1c1
color13 #b48ead
color14 #8fbcbb
color15 #eceff4"#.to_string(),
            description: "Nord color scheme".to_string(),
            palette: {
                let mut map = HashMap::new();
                map.insert("background".to_string(), "#2e3440".to_string());
                map.insert("foreground".to_string(), "#d8dee9".to_string());
                map.insert("cursor".to_string(), "#d8dee9".to_string());
                map
            },
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#color-scheme".to_string(),
        },
    ]
}

