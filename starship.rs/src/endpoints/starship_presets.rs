use crate::models::StarshipPreset;
use crate::utils::logger::Logger;
use crate::utils::validation::InputValidator;
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct PresetsQuery {
    pub preset_name: Option<String>,
}

pub struct PresetsEndpoint;

impl PresetsEndpoint {
    pub async fn query(params: PresetsQuery) -> Result<Vec<StarshipPreset>> {
        let logger = Logger::new("starship_presets");
        logger.info("Querying Starship presets");

        // Validate input parameters
        if let Some(ref name) = params.preset_name {
            InputValidator::validate_name(name)
                .context("Invalid preset name")?;
        }

        // Use cached presets
        let mut presets: Vec<StarshipPreset> = (*PRESETS_CACHE).as_ref().clone();

        if let Some(name) = &params.preset_name {
            presets.retain(|p| p.preset_name == *name);
        }

        logger.info(format!("Returning {} presets", presets.len()));
        Ok(presets)
    }

    // Keep the async version for backward compatibility, but use cache
    #[allow(dead_code)]
    async fn get_all_presets() -> Result<Vec<StarshipPreset>> {
        Ok((*PRESETS_CACHE).as_ref().clone())
    }
}

// Cache presets data to avoid recreating on every request
static PRESETS_CACHE: Lazy<Arc<Vec<StarshipPreset>>> = Lazy::new(|| {
    Arc::new(get_all_presets_data())
});

// Static function to populate the cache
fn get_all_presets_data() -> Vec<StarshipPreset> {
    vec![
            StarshipPreset {
                preset_name: "no-runtime-versions".to_string(),
                snippet: r#"[git_branch]
format = "$symbol$branch"

[nodejs]
disabled = true

[python]
disabled = true

[rust]
disabled = true"#
                    .to_string(),
                description: "Hide runtime version information".to_string(),
                documentation_url: "https://starship.rs/presets/#no-runtime-versions".to_string(),
            },
            StarshipPreset {
                preset_name: "no-empty-icons".to_string(),
                snippet: r#"[git_branch]
format = "$branch"

[git_status]
format = "$conflicted$stashed$modified$renamed$deleted$staged$untracked""#
                    .to_string(),
                description: "Remove icons when modules are empty".to_string(),
                documentation_url: "https://starship.rs/presets/#no-empty-icons".to_string(),
            },
            StarshipPreset {
                preset_name: "no-nerd-font".to_string(),
                snippet: r#"[git_branch]
symbol = ""

[nodejs]
symbol = ""

[python]
symbol = ""

[rust]
symbol = ""#
                    .to_string(),
                description: "Remove Nerd Font symbols".to_string(),
                documentation_url: "https://starship.rs/presets/#no-nerd-font".to_string(),
            },
            StarshipPreset {
                preset_name: "brackets-segments".to_string(),
                snippet: r#"[format]
format = """
$os\
$username\
$hostname\
$localip\
$shlvl\
$directory\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$docker_context\
$package\
$cmake\
$cobol\
$daml\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$guix_shell\
$haskell\
$helm\
$java\
$julia\
$kotlin\
$gradle\
$lua\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
$vagrant\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$container\
$shell\
$character\
"""

[os]
format = "[$symbol]($style) "
style = "bold white"

[username]
format = "[$user]($style) "
style_user = "bold white"
style_root = "bold red"

[hostname]
format = "[$hostname]($style) "
style = "bold white"

[directory]
format = "[$path]($style) "
style = "bold cyan"
truncation_length = 3
truncation_symbol = "…/"

[git_branch]
format = "[$symbol$branch(:$remote_branch)]($style) "
style = "bold purple"

[git_status]
format = "[$all_status$ahead_behind]($style) "
style = "bold yellow"

[cmd_duration]
format = "[$duration]($style) "
style = "bold yellow"

[jobs]
format = "[$symbol$number]($style) "
style = "bold blue"

[battery]
format = "[$symbol$percentage]($style) "
style = "bold green"

[time]
format = "[$time]($style) "
style = "bold white"

[status]
format = "[$symbol$common_meaning$signal_name$maybe_int]($style) "
style = "bold blue"

[container]
format = "[$symbol$name]($style) "
style = "bold red"

[shell]
format = "[$indicator]($style) "
style = "bold white"

[character]
format = "[$symbol]($style) "
style = "bold green"
success_symbol = "[❯](bold green)"
error_symbol = "[❯](bold red)"
vicmd_symbol = "[❮](bold green)""#
                    .to_string(),
                description: "Brackets Segments preset with bracket-style formatting".to_string(),
                documentation_url: "https://starship.rs/presets/#brackets-segments".to_string(),
            },
            StarshipPreset {
                preset_name: "pastel-powerline".to_string(),
                snippet: r#"[format]
format = """
[╭─](bold green)\
$os\
$username\
[─](bold green)[\
$directory\
](bold green)[─](bold green)\
$git_branch\
$git_state\
$git_metrics\
$git_status\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$container\
$shell\
[─](bold green)\
$character\
"""

[os]
format = "[$symbol]($style) "
style = "bold white"

[username]
format = "[$user]($style) "
style_user = "bold white"
style_root = "bold red"

[directory]
format = "[$path]($style) "
style = "bold cyan"
truncation_length = 3
truncation_symbol = "…/"

[git_branch]
format = "[$symbol$branch(:$remote_branch)]($style) "
style = "bold yellow"

[git_status]
format = "[$all_status$ahead_behind]($style) "
style = "bold yellow"

[cmd_duration]
format = "[$duration]($style) "
style = "bold yellow"

[jobs]
format = "[$symbol$number]($style) "
style = "bold blue"

[battery]
format = "[$symbol$percentage]($style) "
style = "bold green"

[time]
format = "[$time]($style) "
style = "bold white"

[status]
format = "[$symbol$common_meaning$signal_name$maybe_int]($style) "
style = "bold blue"

[container]
format = "[$symbol$name]($style) "
style = "bold red"

[shell]
format = "[$indicator]($style) "
style = "bold white"

[character]
format = "[$symbol]($style) "
style = "bold green"
success_symbol = "[─](bold green)"
error_symbol = "[─](bold red)"
vicmd_symbol = "[─](bold green)""#
                    .to_string(),
                description: "Pastel Powerline preset with rounded corners".to_string(),
                documentation_url: "https://starship.rs/presets/#pastel-powerline".to_string(),
            },
            StarshipPreset {
                preset_name: "tokyo-night".to_string(),
                snippet: r#"[format]
format = """
[╭─](bold blue)\
$os\
$username\
[─](bold blue)[\
$directory\
](bold blue)[─](bold blue)\
$git_branch\
$git_state\
$git_metrics\
$git_status\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$container\
$shell\
[─](bold blue)\
$character\
"""

[os]
format = "[$symbol]($style) "
style = "bold white"

[username]
format = "[$user]($style) "
style_user = "bold white"
style_root = "bold red"

[directory]
format = "[$path]($style) "
style = "bold cyan"
truncation_length = 3
truncation_symbol = "…/"

[git_branch]
format = "[$symbol$branch(:$remote_branch)]($style) "
style = "bold yellow"

[git_status]
format = "[$all_status$ahead_behind]($style) "
style = "bold yellow"

[cmd_duration]
format = "[$duration]($style) "
style = "bold yellow"

[jobs]
format = "[$symbol$number]($style) "
style = "bold blue"

[battery]
format = "[$symbol$percentage]($style) "
style = "bold green"

[time]
format = "[$time]($style) "
style = "bold white"

[status]
format = "[$symbol$common_meaning$signal_name$maybe_int]($style) "
style = "bold blue"

[container]
format = "[$symbol$name]($style) "
style = "bold red"

[shell]
format = "[$indicator]($style) "
style = "bold white"

[character]
format = "[$symbol]($style) "
style = "bold blue"
success_symbol = "[─](bold blue)"
error_symbol = "[─](bold red)"
vicmd_symbol = "[─](bold blue)""#
                    .to_string(),
                description: "Tokyo Night color scheme preset".to_string(),
                documentation_url: "https://starship.rs/presets/#tokyo-night".to_string(),
            },
            StarshipPreset {
                preset_name: "pure-preset".to_string(),
                snippet: r#"[format]
format = """
$username\
$hostname\
$localip\
$shlvl\
$directory\
$git_branch\
$git_commit\
$git_state\
$git_metrics\
$git_status\
$docker_context\
$package\
$cmake\
$cobol\
$daml\
$deno\
$dotnet\
$elixir\
$elm\
$erlang\
$golang\
$guix_shell\
$haskell\
$helm\
$java\
$julia\
$kotlin\
$gradle\
$lua\
$nim\
$nodejs\
$ocaml\
$opa\
$perl\
$php\
$pulumi\
$purescript\
$python\
$raku\
$rlang\
$red\
$ruby\
$rust\
$scala\
$swift\
$terraform\
$vlang\
$vagrant\
$nix_shell\
$conda\
$memory_usage\
$aws\
$gcloud\
$openstack\
$env_var\
$crystal\
$custom\
$sudo\
$cmd_duration\
$line_break\
$jobs\
$battery\
$time\
$status\
$container\
$shell\
$character\
"""

[username]
format = "[$user]($style) "
style_user = "bold yellow"
style_root = "bold red"

[hostname]
format = "[@$hostname]($style) "
style = "bold yellow"

[directory]
format = "[$path]($style) "
style = "bold cyan"
truncation_length = 3
truncation_symbol = "…/"

[git_branch]
format = "[$symbol$branch(:$remote_branch)]($style) "
style = "bold purple"

[git_status]
format = "[$all_status$ahead_behind]($style) "
style = "bold yellow"

[cmd_duration]
format = "[$duration]($style) "
style = "bold yellow"

[jobs]
format = "[$symbol$number]($style) "
style = "bold blue"

[battery]
format = "[$symbol$percentage]($style) "
style = "bold green"

[time]
format = "[$time]($style) "
style = "bold white"

[status]
format = "[$symbol$common_meaning$signal_name$maybe_int]($style) "
style = "bold blue"

[container]
format = "[$symbol$name]($style) "
style = "bold red"

[shell]
format = "[$indicator]($style) "
style = "bold white"

[character]
format = "[$symbol]($style) "
style = "bold green"
success_symbol = "[❯](bold green)"
error_symbol = "[❯](bold red)"
vicmd_symbol = "[❮](bold green)""#
                    .to_string(),
                description: "Pure preset - minimal and clean design".to_string(),
                documentation_url: "https://starship.rs/presets/#pure-preset".to_string(),
            },
            StarshipPreset {
                preset_name: "nerd-font-symbols".to_string(),
                snippet: r#"[git_branch]
symbol = " "

[git_status]
conflicted = "🏳 "
up_to_date = "✓ "
untracked = "🤷 "
ahead = "🏎 "
behind = "😰 "
diverged = "😵 "
stashed = "📦 "
modified = "📝 "
staged = "[++($count)](green)"
renamed = "👅 "
deleted = "🗑 "

[directory]
read_only = "🔒"

[nodejs]
symbol = "⬢ "

[python]
symbol = "🐍 "

[rust]
symbol = "🦀 "

[golang]
symbol = "🐹 "

[java]
symbol = "☕ "

[php]
symbol = "🐘 "

[scala]
symbol = "🆂 "

[ruby]
symbol = "💎 "

[swift]
symbol = "🐦 "

[elixir]
symbol = "💧 "

[docker_context]
symbol = "🐳 "

[package]
symbol = "📦 "

[battery]
full_symbol = "🔋 "
charging_symbol = "⚡️ "
discharging_symbol = "💀 "

[status]
symbol = "✖"
symbol_sudo = "⚡"
symbol_int = "ℹ"
symbol_segv = "🗲"
symbol_fpe = "⁇"
symbol_ill = "⚠"
symbol_bus = "🗲"
symbol_abrt = "✖"
symbol_unknown = "?"

[jobs]
symbol = "✦ "

[time]
format = "[$time]($style) "
time_format = "%T"
use_12hr = false"#
                    .to_string(),
                description: "Nerd Font symbols preset with emoji and icons".to_string(),
                documentation_url: "https://starship.rs/presets/#nerd-font-symbols".to_string(),
            },
            StarshipPreset {
                preset_name: "plain-text-symbols".to_string(),
                snippet: r##"[git_branch]
symbol = ""

[git_status]
conflicted = "="
up_to_date = ""
untracked = "?"
ahead = ">"
behind = "<"
diverged = "<>"
stashed = "$"
modified = "!"
staged = "+"
renamed = "»"
deleted = "✘"

[directory]
read_only = "RO"

[nodejs]
symbol = ""

[python]
symbol = ""

[rust]
symbol = ""

[golang]
symbol = ""

[java]
symbol = ""

[php]
symbol = ""

[scala]
symbol = ""

[ruby]
symbol = ""

[swift]
symbol = ""

[elixir]
symbol = ""

[docker_context]
symbol = ""

[package]
symbol = ""

[battery]
full_symbol = ""
charging_symbol = ""
discharging_symbol = ""

[status]
symbol = "x"
symbol_sudo = "#"
symbol_int = "i"
symbol_segv = "!"
symbol_fpe = "?"
symbol_ill = "!"
symbol_bus = "!"
symbol_abrt = "x"
symbol_unknown = "?"

[jobs]
symbol = ""##
                    .to_string(),
                description: "Plain text symbols preset - no special characters".to_string(),
                documentation_url: "https://starship.rs/presets/#plain-text-symbols".to_string(),
            },
        ]
}
