use crate::models::TemplateResult;
use crate::utils::validation;
use anyhow::{Context, Result};
use tracing::debug;

pub async fn generate_template(
    program_name: Option<&str>,
    use_case: Option<&str>,
) -> Result<Vec<TemplateResult>> {
    debug!("Generating template: program_name={:?}, use_case={:?}", program_name, use_case);

    if let Some(name) = program_name {
        validation::validate_string_param(name, Some(100))
            .context("Invalid program name")?;
    }

    if let Some(uc) = use_case {
        validation::validate_string_param(uc, Some(200))
            .context("Invalid use case")?;
    }

    let templates = get_available_templates();

    let filtered: Vec<TemplateResult> = templates
        .into_iter()
        .filter(|t| {
            program_name
                .map(|name| t.program_name.to_lowercase().contains(&name.to_lowercase()))
                .unwrap_or(true)
        })
        .collect();

    Ok(filtered)
}

fn get_available_templates() -> Vec<TemplateResult> {
    vec![
        TemplateResult {
            program_name: "git".to_string(),
            snippet: r#"
  programs.git = {
    enable = true;
    userName = "Your Name";
    userEmail = "your.email@example.com";
    extraConfig = {
      init.defaultBranch = "main";
    };
  };
"#
            .trim()
            .to_string(),
            description: "Basic Git configuration with user name and email".to_string(),
            required_options: vec![
                "programs.git.enable".to_string(),
                "programs.git.userName".to_string(),
                "programs.git.userEmail".to_string(),
            ],
            documentation_url: "https://nix-community.github.io/home-manager/options.html#opt-programs.git.enable".to_string(),
        },
        TemplateResult {
            program_name: "vim".to_string(),
            snippet: r#"
  programs.vim = {
    enable = true;
    plugins = [ "vim-airline" "vim-fugitive" ];
    extraConfig = ''
      set number
      set relativenumber
    '';
  };
"#
            .trim()
            .to_string(),
            description: "Vim configuration with plugins and basic settings".to_string(),
            required_options: vec![
                "programs.vim.enable".to_string(),
            ],
            documentation_url: "https://nix-community.github.io/home-manager/options.html#opt-programs.vim.enable".to_string(),
        },
        TemplateResult {
            program_name: "zsh".to_string(),
            snippet: r#"
  programs.zsh = {
    enable = true;
    enableCompletion = true;
    enableAutosuggestions = true;
    syntaxHighlighting.enable = true;
    ohMyZsh = {
      enable = true;
      plugins = [ "git" "docker" "kubectl" ];
      theme = "robbyrussell";
    };
  };
"#
            .trim()
            .to_string(),
            description: "Zsh configuration with Oh My Zsh".to_string(),
            required_options: vec![
                "programs.zsh.enable".to_string(),
            ],
            documentation_url: "https://nix-community.github.io/home-manager/options.html#opt-programs.zsh.enable".to_string(),
        },
        TemplateResult {
            program_name: "tmux".to_string(),
            snippet: r#"
  programs.tmux = {
    enable = true;
    shortcut = "a";
    baseIndex = 1;
    mouse = true;
    extraConfig = ''
      bind | split-window -h
      bind - split-window -v
    '';
  };
"#
            .trim()
            .to_string(),
            description: "Tmux configuration with custom key bindings".to_string(),
            required_options: vec![
                "programs.tmux.enable".to_string(),
            ],
            documentation_url: "https://nix-community.github.io/home-manager/options.html#opt-programs.tmux.enable".to_string(),
        },
        TemplateResult {
            program_name: "direnv".to_string(),
            snippet: r#"
  programs.direnv = {
    enable = true;
    enableZshIntegration = true;
    nix-direnv.enable = true;
  };
"#
            .trim()
            .to_string(),
            description: "Direnv configuration with Nix integration".to_string(),
            required_options: vec![
                "programs.direnv.enable".to_string(),
            ],
            documentation_url: "https://nix-community.github.io/home-manager/options.html#opt-programs.direnv.enable".to_string(),
        },
        TemplateResult {
            program_name: "alacritty".to_string(),
            snippet: r##"
  programs.alacritty = {
    enable = true;
    settings = {
      window.padding = { x = 5; y = 5; };
      font.size = 12.0;
      colors.primary.background = "#1e1e1e";
      colors.primary.foreground = "#d4d4d4";
    };
  };
"##
            .trim()
            .to_string(),
            description: "Alacritty terminal emulator configuration".to_string(),
            required_options: vec![
                "programs.alacritty.enable".to_string(),
            ],
            documentation_url: "https://nix-community.github.io/home-manager/options.html#opt-programs.alacritty.enable".to_string(),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate_template_all() {
        let templates = generate_template(None, None).await.unwrap();
        assert!(!templates.is_empty());
        assert!(templates.len() >= 6);
    }

    #[tokio::test]
    async fn test_generate_template_filter_by_name() {
        let templates = generate_template(Some("git"), None).await.unwrap();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.program_name == "git"));
    }

    #[tokio::test]
    async fn test_generate_template_case_insensitive() {
        let templates = generate_template(Some("GIT"), None).await.unwrap();
        assert!(!templates.is_empty());
        assert!(templates.iter().any(|t| t.program_name.to_lowercase() == "git"));
    }

    #[tokio::test]
    async fn test_generate_template_no_match() {
        let templates = generate_template(Some("nonexistent"), None).await.unwrap();
        assert!(templates.is_empty());
    }

    #[test]
    fn test_get_available_templates() {
        let templates = get_available_templates();
        assert!(!templates.is_empty());
        
        let git_template = templates.iter().find(|t| t.program_name == "git").unwrap();
        assert!(git_template.snippet.contains("programs.git"));
        assert!(!git_template.required_options.is_empty());
    }

    #[test]
    fn test_template_structure() {
        let templates = get_available_templates();
        
        for template in templates {
            assert!(!template.program_name.is_empty());
            assert!(!template.snippet.is_empty());
            assert!(!template.description.is_empty());
            assert!(!template.documentation_url.is_empty());
        }
    }
}
