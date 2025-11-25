use crate::models::KittyKeybinding;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KeybindingsQuery {
    pub action: Option<String>,
}

pub async fn handle_kitty_keybindings(query: KeybindingsQuery) -> Vec<KittyKeybinding> {
    let keybindings = get_kitty_keybindings();
    
    if let Some(action) = &query.action {
        keybindings
            .into_iter()
            .filter(|k| k.action.to_lowercase() == action.to_lowercase())
            .collect()
    } else {
        keybindings
    }
}

fn get_kitty_keybindings() -> Vec<KittyKeybinding> {
    vec![
        KittyKeybinding {
            action: "new_window".to_string(),
            args: None,
            modifiers: vec!["ctrl".to_string(), "shift".to_string()],
            keys: "enter".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.action".to_string(),
        },
        KittyKeybinding {
            action: "new_tab".to_string(),
            args: None,
            modifiers: vec!["ctrl".to_string(), "shift".to_string()],
            keys: "t".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.action".to_string(),
        },
        KittyKeybinding {
            action: "close_window".to_string(),
            args: None,
            modifiers: vec!["ctrl".to_string(), "shift".to_string()],
            keys: "w".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.action".to_string(),
        },
        KittyKeybinding {
            action: "next_window".to_string(),
            args: None,
            modifiers: vec!["ctrl".to_string(), "shift".to_string()],
            keys: "j".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.action".to_string(),
        },
        KittyKeybinding {
            action: "previous_window".to_string(),
            args: None,
            modifiers: vec!["ctrl".to_string(), "shift".to_string()],
            keys: "k".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.action".to_string(),
        },
        KittyKeybinding {
            action: "resize_window".to_string(),
            args: Some(vec!["taller".to_string()]),
            modifiers: vec!["ctrl".to_string(), "shift".to_string()],
            keys: "up".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.action".to_string(),
        },
        KittyKeybinding {
            action: "goto_layout".to_string(),
            args: Some(vec!["tall".to_string()]),
            modifiers: vec!["ctrl".to_string(), "shift".to_string()],
            keys: "l".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/conf/#opt-kitty.action".to_string(),
        },
        KittyKeybinding {
            action: "kitten".to_string(),
            args: Some(vec!["hyperlinked_grep".to_string()]),
            modifiers: vec!["ctrl".to_string(), "shift".to_string()],
            keys: "e".to_string(),
            documentation_url: "https://sw.kovidgoyal.net/kitty/kittens/".to_string(),
        },
    ]
}

