use crate::models::TemplateOutput;
use crate::utils::logger::Logger;
use crate::utils::validation::InputValidator;
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TemplatesQuery {
    pub category: Option<String>,
    pub use_case: Option<String>,
}

pub struct TemplatesEndpoint;

impl TemplatesEndpoint {
    pub async fn query(params: TemplatesQuery) -> Result<Vec<TemplateOutput>> {
        let logger = Logger::new("starship_templates");
        logger.info("Generating Starship configuration templates");

        // Validate input parameters
        if let Some(ref category) = params.category {
            InputValidator::validate_category(category)
                .context("Invalid category")?;
        }

        if let Some(ref use_case) = params.use_case {
            InputValidator::validate_search_term(use_case)
                .context("Invalid use case")?;
        }

        let templates = Self::get_templates().await?;
        let mut filtered = templates;

        if let Some(category) = &params.category {
            filtered.retain(|t| {
                t.description.to_lowercase().contains(&category.to_lowercase())
                    || t.template_name.to_lowercase().contains(&category.to_lowercase())
            });
        }

        if let Some(use_case) = &params.use_case {
            filtered.retain(|t| {
                t.description.to_lowercase().contains(&use_case.to_lowercase())
            });
        }

        logger.info(format!("Returning {} templates", filtered.len()));
        Ok(filtered)
    }

    async fn get_templates() -> Result<Vec<TemplateOutput>> {
        Ok(vec![
            TemplateOutput {
                template_name: "minimal".to_string(),
                snippet: r#"format = "$all"

[character]
success_symbol = "[❯](purple)"
error_symbol = "[❯](red)""#
                    .to_string(),
                description: "Minimal configuration with basic prompt".to_string(),
                documentation_url: "https://starship.rs/config/".to_string(),
            },
            TemplateOutput {
                template_name: "git-focused".to_string(),
                snippet: r#"format = """
$username\
$hostname\
$directory\
$git_branch\
$git_state\
$git_status\
$character\
"""

[git_branch]
symbol = " "
style = "bold purple"

[git_status]
conflicted = "🏳 "
up_to_date = "✓ "
untracked = "🤷 "
ahead = "🏎 "
behind = "😰 "
diverged = "😵 "
stashed = "📦 "
modified = "📝 "
staged = '[++\($count\)](green)'
renamed = "👅 "
deleted = "🗑 ""#
                    .to_string(),
                description: "Git-focused configuration with detailed status indicators".to_string(),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            TemplateOutput {
                template_name: "development-env".to_string(),
                snippet: r#"format = """
$username\
$hostname\
$directory\
$nodejs\
$python\
$rust\
$git_branch\
$git_status\
$character\
"""

[nodejs]
symbol = "⬢ "
style = "bold green"

[python]
symbol = "🐍 "
style = "bold yellow"

[rust]
symbol = "🦀 "
style = "bold red"

[git_branch]
symbol = " "
style = "bold purple""#
                    .to_string(),
                description: "Development environment with runtime version indicators".to_string(),
                documentation_url: "https://starship.rs/config/".to_string(),
            },
            TemplateOutput {
                template_name: "performance-optimized".to_string(),
                snippet: r#"scan_timeout = 10
command_timeout = 100

format = """
$username\
$hostname\
$directory\
$git_branch\
$character\
"""

[git_branch]
format = "$branch"
disabled = false

[nodejs]
disabled = true

[python]
disabled = true

[rust]
disabled = true

[package]
disabled = true

[docker_context]
disabled = true""#
                    .to_string(),
                description: "Performance-optimized configuration with minimal modules".to_string(),
                documentation_url: "https://starship.rs/config/#performance".to_string(),
            },
            TemplateOutput {
                template_name: "custom-prompt".to_string(),
                snippet: r#"format = """
[╭─](bold cyan)\
$username\
[─](bold cyan)[\
$directory\
](bold cyan)[─](bold cyan)\
$git_branch\
$git_status\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
[─](bold cyan)\
$character\
"""

[username]
format = "[$user]($style) "
style_user = "bold cyan"
style_root = "bold red"

[directory]
format = "[$path]($style) "
style = "bold blue"
truncation_length = 3
truncation_symbol = "…/"

[git_branch]
format = "[$symbol$branch]($style) "
style = "bold purple"

[cmd_duration]
format = "[$duration]($style) "
style = "bold yellow"

[character]
format = "[$symbol]($style) "
style = "bold cyan"
success_symbol = "[─](bold cyan)"
error_symbol = "[─](bold red)""#
                    .to_string(),
                description: "Custom prompt with box drawing characters and colors".to_string(),
                documentation_url: "https://starship.rs/config/#format".to_string(),
            },
        ])
    }
}

