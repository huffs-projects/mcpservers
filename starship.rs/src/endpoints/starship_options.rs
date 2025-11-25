use crate::models::StarshipOption;
use crate::utils::logger::Logger;
use crate::utils::validation::InputValidator;
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct OptionsQuery {
    pub search_term: Option<String>,
    pub category: Option<String>,
}

pub struct OptionsEndpoint;

impl OptionsEndpoint {
    pub async fn query(params: OptionsQuery) -> Result<Vec<StarshipOption>> {
        let logger = Logger::new("starship_options");
        logger.info("Querying Starship configuration options");

        // Validate input parameters
        if let Some(ref term) = params.search_term {
            InputValidator::validate_search_term(term)
                .context("Invalid search term")?;
        }

        if let Some(ref category) = params.category {
            InputValidator::validate_category(category)
                .context("Invalid category")?;
        }

        // Use cached options
        let mut options: Vec<StarshipOption> = (*OPTIONS_CACHE).as_ref().clone();

        // Filter by search term
        if let Some(term) = &params.search_term {
            let term_lower = term.to_lowercase();
            options.retain(|opt| {
                opt.name.to_lowercase().contains(&term_lower)
                    || opt.description.to_lowercase().contains(&term_lower)
            });
        }

        // Filter by category
        if let Some(category) = &params.category {
            options.retain(|opt| opt.category == *category);
        }

        logger.info(format!("Returning {} options", options.len()));
        Ok(options)
    }

    // Keep the async version for backward compatibility, but use cache
    #[allow(dead_code)]
    async fn get_all_options() -> Result<Vec<StarshipOption>> {
        Ok((*OPTIONS_CACHE).as_ref().clone())
    }
}

// Cache options data to avoid recreating on every request
static OPTIONS_CACHE: Lazy<Arc<Vec<StarshipOption>>> = Lazy::new(|| {
    Arc::new(get_all_options_data())
});

// Static function to populate the cache
fn get_all_options_data() -> Vec<StarshipOption> {
    // Comprehensive list of Starship configuration options
    // Organized by category with official documentation links
    vec![
            // General Configuration
            StarshipOption {
                name: "format".to_string(),
                option_type: "string".to_string(),
                default: Some("$all".to_string()),
                category: "general".to_string(),
                description: "The format string for the prompt. Use $all to include all modules.".to_string(),
                example: Some("$all".to_string()),
                documentation_url: "https://starship.rs/config/#format".to_string(),
            },
            StarshipOption {
                name: "add_newline".to_string(),
                option_type: "boolean".to_string(),
                default: Some("true".to_string()),
                category: "general".to_string(),
                description: "Add a newline before each prompt".to_string(),
                example: Some("true".to_string()),
                documentation_url: "https://starship.rs/config/#add-newline".to_string(),
            },
            StarshipOption {
                name: "scan_timeout".to_string(),
                option_type: "integer".to_string(),
                default: Some("30".to_string()),
                category: "general".to_string(),
                description: "Timeout for scanning files in milliseconds".to_string(),
                example: Some("30".to_string()),
                documentation_url: "https://starship.rs/config/#scan-timeout".to_string(),
            },
            StarshipOption {
                name: "command_timeout".to_string(),
                option_type: "integer".to_string(),
                default: Some("500".to_string()),
                category: "general".to_string(),
                description: "Timeout for commands in milliseconds".to_string(),
                example: Some("500".to_string()),
                documentation_url: "https://starship.rs/config/#command-timeout".to_string(),
            },
            StarshipOption {
                name: "palette".to_string(),
                option_type: "string".to_string(),
                default: None,
                category: "general".to_string(),
                description: "Override the default color palette".to_string(),
                example: Some("\"darkest-dark\"".to_string()),
                documentation_url: "https://starship.rs/config/#palette".to_string(),
            },
            // Git Branch Module
            StarshipOption {
                name: "git_branch.symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\" \"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown before the git branch".to_string(),
                example: Some("\" \"".to_string()),
                documentation_url: "https://starship.rs/config/#git-branch".to_string(),
            },
            StarshipOption {
                name: "git_branch.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold purple\"".to_string()),
                category: "module".to_string(),
                description: "The style for the git_branch module".to_string(),
                example: Some("\"bold purple\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-branch".to_string(),
            },
            StarshipOption {
                name: "git_branch.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"on [$symbol$branch(:$remote_branch)]($style) \"".to_string()),
                category: "module".to_string(),
                description: "The format string for the git_branch module".to_string(),
                example: Some("\"on [$symbol$branch]($style) \"".to_string()),
                documentation_url: "https://starship.rs/config/#git-branch".to_string(),
            },
            StarshipOption {
                name: "git_branch.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the git_branch module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#git-branch".to_string(),
            },
            StarshipOption {
                name: "git_branch.truncation_length".to_string(),
                option_type: "integer".to_string(),
                default: Some("9999".to_string()),
                category: "module".to_string(),
                description: "Truncates a git branch to N graphemes".to_string(),
                example: Some("20".to_string()),
                documentation_url: "https://starship.rs/config/#git-branch".to_string(),
            },
            StarshipOption {
                name: "git_branch.truncation_symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"…\"".to_string()),
                category: "module".to_string(),
                description: "The symbol used to indicate a branch name is truncated".to_string(),
                example: Some("\"…\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-branch".to_string(),
            },
            // Git Status Module
            StarshipOption {
                name: "git_status.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"([\\[$all_status$ahead_behind\\]]($style) )\"".to_string()),
                category: "module".to_string(),
                description: "The format string for the git_status module".to_string(),
                example: Some("\"[$all_status$ahead_behind]($style)\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.conflicted".to_string(),
                option_type: "string".to_string(),
                default: Some("\"=\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when there are merge conflicts".to_string(),
                example: Some("\"🏳 \"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.up_to_date".to_string(),
                option_type: "string".to_string(),
                default: Some("\"\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when the branch is up to date".to_string(),
                example: Some("\"✓\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.untracked".to_string(),
                option_type: "string".to_string(),
                default: Some("\"?\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when there are untracked files".to_string(),
                example: Some("\"🤷\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.ahead".to_string(),
                option_type: "string".to_string(),
                default: Some("\"⇡\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when the branch is ahead of the remote".to_string(),
                example: Some("\"🏎\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.behind".to_string(),
                option_type: "string".to_string(),
                default: Some("\"⇣\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when the branch is behind the remote".to_string(),
                example: Some("\"😰\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.diverged".to_string(),
                option_type: "string".to_string(),
                default: Some("\"⇕\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when the branch has diverged from the remote".to_string(),
                example: Some("\"😵\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.stashed".to_string(),
                option_type: "string".to_string(),
                default: Some("\"$\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when there are stashed changes".to_string(),
                example: Some("\"📦\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.modified".to_string(),
                option_type: "string".to_string(),
                default: Some("\"!\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when there are modified files".to_string(),
                example: Some("\"📝\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.staged".to_string(),
                option_type: "string".to_string(),
                default: Some("\"+\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when there are staged changes".to_string(),
                example: Some("\"[++$count](green)\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.renamed".to_string(),
                option_type: "string".to_string(),
                default: Some("\"»\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when there are renamed files".to_string(),
                example: Some("\"👅\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            StarshipOption {
                name: "git_status.deleted".to_string(),
                option_type: "string".to_string(),
                default: Some("\"✘\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when there are deleted files".to_string(),
                example: Some("\"🗑\"".to_string()),
                documentation_url: "https://starship.rs/config/#git-status".to_string(),
            },
            // Directory Module
            StarshipOption {
                name: "directory.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"[$path]($style)[$read_only]($read_only_style) \"".to_string()),
                category: "module".to_string(),
                description: "The format string for the directory module".to_string(),
                example: Some("\"[$path]($style) \"".to_string()),
                documentation_url: "https://starship.rs/config/#directory".to_string(),
            },
            StarshipOption {
                name: "directory.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold cyan\"".to_string()),
                category: "module".to_string(),
                description: "The style for the directory module".to_string(),
                example: Some("\"bold cyan\"".to_string()),
                documentation_url: "https://starship.rs/config/#directory".to_string(),
            },
            StarshipOption {
                name: "directory.truncation_length".to_string(),
                option_type: "integer".to_string(),
                default: Some("3".to_string()),
                category: "module".to_string(),
                description: "The number of parent path segments to show".to_string(),
                example: Some("3".to_string()),
                documentation_url: "https://starship.rs/config/#directory".to_string(),
            },
            StarshipOption {
                name: "directory.truncation_symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"…/\"".to_string()),
                category: "module".to_string(),
                description: "The symbol to prefix to truncated paths".to_string(),
                example: Some("\"…/\"".to_string()),
                documentation_url: "https://starship.rs/config/#directory".to_string(),
            },
            StarshipOption {
                name: "directory.home_symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"~\"".to_string()),
                category: "module".to_string(),
                description: "The symbol indicating home directory".to_string(),
                example: Some("\"~\"".to_string()),
                documentation_url: "https://starship.rs/config/#directory".to_string(),
            },
            StarshipOption {
                name: "directory.read_only".to_string(),
                option_type: "string".to_string(),
                default: Some("\"🔒\"".to_string()),
                category: "module".to_string(),
                description: "The symbol indicating read-only directory".to_string(),
                example: Some("\"🔒\"".to_string()),
                documentation_url: "https://starship.rs/config/#directory".to_string(),
            },
            // Node.js Module
            StarshipOption {
                name: "nodejs.symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"⬢ \"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown before the Node.js version".to_string(),
                example: Some("\"⬢ \"".to_string()),
                documentation_url: "https://starship.rs/config/#nodejs".to_string(),
            },
            StarshipOption {
                name: "nodejs.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold green\"".to_string()),
                category: "module".to_string(),
                description: "The style for the nodejs module".to_string(),
                example: Some("\"bold green\"".to_string()),
                documentation_url: "https://starship.rs/config/#nodejs".to_string(),
            },
            StarshipOption {
                name: "nodejs.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"via [$symbol($version )]($style)\"".to_string()),
                category: "module".to_string(),
                description: "The format string for the nodejs module".to_string(),
                example: Some("\"via [$symbol$version]($style)\"".to_string()),
                documentation_url: "https://starship.rs/config/#nodejs".to_string(),
            },
            StarshipOption {
                name: "nodejs.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the nodejs module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#nodejs".to_string(),
            },
            // Python Module
            StarshipOption {
                name: "python.symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"🐍 \"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown before the Python version".to_string(),
                example: Some("\"🐍 \"".to_string()),
                documentation_url: "https://starship.rs/config/#python".to_string(),
            },
            StarshipOption {
                name: "python.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"yellow bold\"".to_string()),
                category: "module".to_string(),
                description: "The style for the python module".to_string(),
                example: Some("\"yellow bold\"".to_string()),
                documentation_url: "https://starship.rs/config/#python".to_string(),
            },
            StarshipOption {
                name: "python.format".to_string(),
                option_type: "string".to_string(),
                default: Some(r#"\"via [${symbol}${pyenv_prefix}(${version} )(\($virtualenv\) )]($style)\""#.to_string()),
                category: "module".to_string(),
                description: "The format string for the python module".to_string(),
                example: Some("\"via [$symbol$version]($style)\"".to_string()),
                documentation_url: "https://starship.rs/config/#python".to_string(),
            },
            StarshipOption {
                name: "python.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the python module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#python".to_string(),
            },
            // Rust Module
            StarshipOption {
                name: "rust.symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"🦀 \"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown before the Rust version".to_string(),
                example: Some("\"🦀 \"".to_string()),
                documentation_url: "https://starship.rs/config/#rust".to_string(),
            },
            StarshipOption {
                name: "rust.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold red\"".to_string()),
                category: "module".to_string(),
                description: "The style for the rust module".to_string(),
                example: Some("\"bold red\"".to_string()),
                documentation_url: "https://starship.rs/config/#rust".to_string(),
            },
            StarshipOption {
                name: "rust.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"via [$symbol($version )]($style)\"".to_string()),
                category: "module".to_string(),
                description: "The format string for the rust module".to_string(),
                example: Some("\"via [$symbol$version]($style)\"".to_string()),
                documentation_url: "https://starship.rs/config/#rust".to_string(),
            },
            StarshipOption {
                name: "rust.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the rust module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#rust".to_string(),
            },
            // Character Module
            StarshipOption {
                name: "character.success_symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"[❯](bold green)\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when the last command succeeds".to_string(),
                example: Some("\"[❯](bold green)\"".to_string()),
                documentation_url: "https://starship.rs/config/#character".to_string(),
            },
            StarshipOption {
                name: "character.error_symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"[❯](bold red)\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when the last command fails".to_string(),
                example: Some("\"[❯](bold red)\"".to_string()),
                documentation_url: "https://starship.rs/config/#character".to_string(),
            },
            StarshipOption {
                name: "character.vicmd_symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"[❮](bold green)\"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when in vim normal mode".to_string(),
                example: Some("\"[❮](bold green)\"".to_string()),
                documentation_url: "https://starship.rs/config/#character".to_string(),
            },
            StarshipOption {
                name: "character.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"$symbol \"".to_string()),
                category: "module".to_string(),
                description: "The format string for the character module".to_string(),
                example: Some("\"$symbol \"".to_string()),
                documentation_url: "https://starship.rs/config/#character".to_string(),
            },
            // Username Module
            StarshipOption {
                name: "username.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"[$user]($style) \"".to_string()),
                category: "module".to_string(),
                description: "The format string for the username module".to_string(),
                example: Some("\"[$user]($style) \"".to_string()),
                documentation_url: "https://starship.rs/config/#username".to_string(),
            },
            StarshipOption {
                name: "username.style_root".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold red\"".to_string()),
                category: "module".to_string(),
                description: "The style for the username when root".to_string(),
                example: Some("\"bold red\"".to_string()),
                documentation_url: "https://starship.rs/config/#username".to_string(),
            },
            StarshipOption {
                name: "username.style_user".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold yellow\"".to_string()),
                category: "module".to_string(),
                description: "The style for the username when not root".to_string(),
                example: Some("\"bold yellow\"".to_string()),
                documentation_url: "https://starship.rs/config/#username".to_string(),
            },
            StarshipOption {
                name: "username.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the username module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#username".to_string(),
            },
            // Hostname Module
            StarshipOption {
                name: "hostname.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"[$hostname]($style) \"".to_string()),
                category: "module".to_string(),
                description: "The format string for the hostname module".to_string(),
                example: Some("\"[$hostname]($style) \"".to_string()),
                documentation_url: "https://starship.rs/config/#hostname".to_string(),
            },
            StarshipOption {
                name: "hostname.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold dimmed green\"".to_string()),
                category: "module".to_string(),
                description: "The style for the hostname module".to_string(),
                example: Some("\"bold dimmed green\"".to_string()),
                documentation_url: "https://starship.rs/config/#hostname".to_string(),
            },
            StarshipOption {
                name: "hostname.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the hostname module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#hostname".to_string(),
            },
            // CMD Duration Module
            StarshipOption {
                name: "cmd_duration.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"took [$duration]($style) \"".to_string()),
                category: "module".to_string(),
                description: "The format string for the cmd_duration module".to_string(),
                example: Some("\"took [$duration]($style) \"".to_string()),
                documentation_url: "https://starship.rs/config/#cmd-duration".to_string(),
            },
            StarshipOption {
                name: "cmd_duration.min_time".to_string(),
                option_type: "integer".to_string(),
                default: Some("2".to_string()),
                category: "module".to_string(),
                description: "The minimum time in milliseconds to show the duration".to_string(),
                example: Some("2".to_string()),
                documentation_url: "https://starship.rs/config/#cmd-duration".to_string(),
            },
            StarshipOption {
                name: "cmd_duration.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold yellow\"".to_string()),
                category: "module".to_string(),
                description: "The style for the cmd_duration module".to_string(),
                example: Some("\"bold yellow\"".to_string()),
                documentation_url: "https://starship.rs/config/#cmd-duration".to_string(),
            },
            StarshipOption {
                name: "cmd_duration.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the cmd_duration module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#cmd-duration".to_string(),
            },
            // Jobs Module
            StarshipOption {
                name: "jobs.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"[$symbol$number]($style) \"".to_string()),
                category: "module".to_string(),
                description: "The format string for the jobs module".to_string(),
                example: Some("\"[$symbol$number]($style) \"".to_string()),
                documentation_url: "https://starship.rs/config/#jobs".to_string(),
            },
            StarshipOption {
                name: "jobs.symbol".to_string(),
                option_type: "string".to_string(),
                default: Some("\"✦ \"".to_string()),
                category: "module".to_string(),
                description: "The symbol shown when there are background jobs".to_string(),
                example: Some("\"✦ \"".to_string()),
                documentation_url: "https://starship.rs/config/#jobs".to_string(),
            },
            StarshipOption {
                name: "jobs.number_threshold".to_string(),
                option_type: "integer".to_string(),
                default: Some("1".to_string()),
                category: "module".to_string(),
                description: "The number of jobs that must be exceeded before the module is shown".to_string(),
                example: Some("1".to_string()),
                documentation_url: "https://starship.rs/config/#jobs".to_string(),
            },
            StarshipOption {
                name: "jobs.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold blue\"".to_string()),
                category: "module".to_string(),
                description: "The style for the jobs module".to_string(),
                example: Some("\"bold blue\"".to_string()),
                documentation_url: "https://starship.rs/config/#jobs".to_string(),
            },
            StarshipOption {
                name: "jobs.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the jobs module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#jobs".to_string(),
            },
            // Time Module
            StarshipOption {
                name: "time.format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"[$time]($style) \"".to_string()),
                category: "module".to_string(),
                description: "The format string for the time module".to_string(),
                example: Some("\"[$time]($style) \"".to_string()),
                documentation_url: "https://starship.rs/config/#time".to_string(),
            },
            StarshipOption {
                name: "time.time_format".to_string(),
                option_type: "string".to_string(),
                default: Some("\"%T\"".to_string()),
                category: "module".to_string(),
                description: "The format string for displaying time (strftime format)".to_string(),
                example: Some("\"%T\"".to_string()),
                documentation_url: "https://starship.rs/config/#time".to_string(),
            },
            StarshipOption {
                name: "time.style".to_string(),
                option_type: "string".to_string(),
                default: Some("\"bold white\"".to_string()),
                category: "module".to_string(),
                description: "The style for the time module".to_string(),
                example: Some("\"bold white\"".to_string()),
                documentation_url: "https://starship.rs/config/#time".to_string(),
            },
            StarshipOption {
                name: "time.disabled".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Disables the time module".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#time".to_string(),
            },
            StarshipOption {
                name: "time.use_12hr".to_string(),
                option_type: "boolean".to_string(),
                default: Some("false".to_string()),
                category: "module".to_string(),
                description: "Enables 12-hour format".to_string(),
                example: Some("false".to_string()),
                documentation_url: "https://starship.rs/config/#time".to_string(),
            },
        ]
}
