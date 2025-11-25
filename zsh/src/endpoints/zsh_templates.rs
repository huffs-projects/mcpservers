use crate::models::ZshTemplate;

pub fn generate_templates(use_case: Option<String>) -> Vec<ZshTemplate> {
    let mut templates = Vec::new();
    
    let use_case_lower = use_case.as_ref().map(|s| s.to_lowercase());
    
    if use_case_lower.is_none() || use_case_lower.as_ref().unwrap() == "prompt" {
        templates.push(ZshTemplate {
            template_name: "powerline_prompt".to_string(),
            snippet: r#"# Powerline-style prompt
autoload -Uz vcs_info
precmd() { vcs_info }
zstyle ':vcs_info:git:*' formats ' %b'
setopt PROMPT_SUBST
PROMPT='%F{blue}%n@%m%f %F{green}%~%f%F{red}${vcs_info_msg_0_}%f %# '"#.to_string(),
            description: "A colorful prompt with git branch information".to_string(),
            uses_options: vec!["PROMPT_SUBST".to_string()],
        });
        
        templates.push(ZshTemplate {
            template_name: "minimal_prompt".to_string(),
            snippet: r#"# Minimal prompt
PROMPT='%# '"#.to_string(),
            description: "A minimal prompt showing only the prompt character".to_string(),
            uses_options: vec![],
        });
    }
    
    if use_case_lower.is_none() || use_case_lower.as_ref().unwrap() == "completion" {
        templates.push(ZshTemplate {
            template_name: "basic_completion".to_string(),
            snippet: r#"# Enable completion system
autoload -Uz compinit
compinit

# Completion options
setopt AUTO_LIST
setopt AUTO_MENU
setopt COMPLETE_IN_WORD
setopt LIST_TYPES"#.to_string(),
            description: "Basic completion system setup with common options".to_string(),
            uses_options: vec![
                "AUTO_LIST".to_string(),
                "AUTO_MENU".to_string(),
                "COMPLETE_IN_WORD".to_string(),
                "LIST_TYPES".to_string(),
            ],
        });
        
        templates.push(ZshTemplate {
            template_name: "advanced_completion".to_string(),
            snippet: r#"# Advanced completion system
autoload -Uz compinit
compinit -d ~/.zcompdump-$ZSH_VERSION

# Completion options
setopt AUTO_LIST
setopt AUTO_MENU
setopt AUTO_PARAM_SLASH
setopt COMPLETE_ALIASES
setopt COMPLETE_IN_WORD
setopt GLOB_COMPLETE
setopt LIST_AMBIGUOUS
setopt LIST_PACKED
setopt LIST_ROWS_FIRST
setopt LIST_TYPES
setopt MENU_COMPLETE
setopt REC_EXACT"#.to_string(),
            description: "Advanced completion with all useful options enabled".to_string(),
            uses_options: vec![
                "AUTO_LIST".to_string(),
                "AUTO_MENU".to_string(),
                "AUTO_PARAM_SLASH".to_string(),
                "COMPLETE_ALIASES".to_string(),
                "COMPLETE_IN_WORD".to_string(),
                "GLOB_COMPLETE".to_string(),
                "LIST_AMBIGUOUS".to_string(),
                "LIST_PACKED".to_string(),
                "LIST_ROWS_FIRST".to_string(),
                "LIST_TYPES".to_string(),
                "MENU_COMPLETE".to_string(),
                "REC_EXACT".to_string(),
            ],
        });
    }
    
    if use_case_lower.is_none() || use_case_lower.as_ref().unwrap() == "vi-mode" {
        templates.push(ZshTemplate {
            template_name: "vi_mode".to_string(),
            snippet: r#"# Vi mode key bindings
bindkey -v

# Vi mode options
setopt BEEP

# Key bindings for vi mode
bindkey '^R' history-incremental-search-backward
bindkey '^S' history-incremental-search-forward
bindkey '^P' history-search-backward
bindkey '^N' history-search-forward"#.to_string(),
            description: "Vi mode key bindings with history search".to_string(),
            uses_options: vec!["BEEP".to_string()],
        });
    }
    
    if use_case_lower.is_none() || use_case_lower.as_ref().unwrap() == "keybindings" {
        templates.push(ZshTemplate {
            template_name: "emacs_keybindings".to_string(),
            snippet: r#"# Emacs-style key bindings
bindkey -e

# History navigation
bindkey '^R' history-incremental-search-backward
bindkey '^S' history-incremental-search-forward
bindkey '^P' up-line-or-history
bindkey '^N' down-line-or-history

# Word navigation
bindkey '^F' forward-word
bindkey '^B' backward-word"#.to_string(),
            description: "Emacs-style key bindings with history and word navigation".to_string(),
            uses_options: vec![],
        });
    }
    
    if use_case_lower.is_none() || use_case_lower.as_ref().unwrap() == "history" {
        templates.push(ZshTemplate {
            template_name: "history_config".to_string(),
            snippet: r#"# History configuration
HISTFILE=~/.zsh_history
HISTSIZE=10000
SAVEHIST=10000

# History options
setopt APPEND_HISTORY
setopt EXTENDED_HISTORY
setopt HIST_EXPIRE_DUPS_FIRST
setopt HIST_IGNORE_DUPS
setopt HIST_IGNORE_SPACE
setopt HIST_VERIFY
setopt INC_APPEND_HISTORY
setopt SHARE_HISTORY"#.to_string(),
            description: "Comprehensive history configuration with sharing and deduplication".to_string(),
            uses_options: vec![
                "EXTENDED_HISTORY".to_string(),
                "HIST_EXPIRE_DUPS_FIRST".to_string(),
                "HIST_IGNORE_DUPS".to_string(),
                "HIST_IGNORE_SPACE".to_string(),
                "HIST_VERIFY".to_string(),
                "INC_APPEND_HISTORY".to_string(),
            ],
        });
    }
    
    if use_case_lower.is_none() || use_case_lower.as_ref().unwrap() == "modules" {
        templates.push(ZshTemplate {
            template_name: "common_modules".to_string(),
            snippet: r#"# Load common Zsh modules
zmodload zsh/complist
zmodload zsh/computil
zmodload zsh/mapfile
zmodload zsh/mathfunc
zmodload zsh/terminfo
zmodload zsh/zle
zmodload zsh/zpty
zmodload zsh/zutil"#.to_string(),
            description: "Load commonly useful Zsh modules".to_string(),
            uses_options: vec![],
        });
    }
    
    templates
}

