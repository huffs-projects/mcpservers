use serde_json::{json, Value};
use anyhow::Result;

#[derive(Debug, serde::Serialize)]
pub struct Prompt {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
}

#[derive(Debug, serde::Serialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetPromptResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub messages: Vec<PromptMessage>,
}

#[derive(Debug, serde::Serialize)]
pub struct PromptMessage {
    pub role: String,
    pub content: PromptMessageContent,
}

#[derive(Debug, serde::Serialize)]
#[serde(untagged)]
pub enum PromptMessageContent {
    Text(String),
    Parts(Vec<PromptMessagePart>),
}

#[derive(Debug, serde::Serialize)]
pub struct PromptMessagePart {
    #[serde(rename = "type")]
    pub part_type: String,
    pub text: String,
}

pub fn list_prompts() -> Vec<Prompt> {
    vec![
        Prompt {
            name: "fastfetch-config-help".to_string(),
            description: "Get help with fastfetch configuration issues".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "issue".to_string(),
                    description: "Description of the configuration issue or question".to_string(),
                    required: Some(true),
                },
                PromptArgument {
                    name: "config_snippet".to_string(),
                    description: "Optional fastfetch config snippet to review".to_string(),
                    required: Some(false),
                },
            ]),
        },
        Prompt {
            name: "fastfetch-module-setup".to_string(),
            description: "Get help setting up a specific fastfetch module".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "module".to_string(),
                    description: "The name of the module to set up".to_string(),
                    required: Some(true),
                },
                PromptArgument {
                    name: "use_case".to_string(),
                    description: "Optional description of what you want to achieve with this module".to_string(),
                    required: Some(false),
                },
            ]),
        },
        Prompt {
            name: "fastfetch-logo-customization".to_string(),
            description: "Get help customizing fastfetch logos".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "logo_name".to_string(),
                    description: "Optional name of the logo to customize".to_string(),
                    required: Some(false),
                },
                PromptArgument {
                    name: "customization_goal".to_string(),
                    description: "Description of what you want to customize".to_string(),
                    required: Some(false),
                },
            ]),
        },
        Prompt {
            name: "fastfetch-format-string-help".to_string(),
            description: "Get help creating format strings for fastfetch output".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "desired_output".to_string(),
                    description: "Description of the output format you want to achieve".to_string(),
                    required: Some(true),
                },
            ]),
        },
        Prompt {
            name: "fastfetch-color-configuration".to_string(),
            description: "Get help configuring colors in fastfetch".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "color_scheme".to_string(),
                    description: "Optional description of the color scheme you want to use".to_string(),
                    required: Some(false),
                },
            ]),
        },
        Prompt {
            name: "fastfetch-migrate-neofetch".to_string(),
            description: "Get help migrating from Neofetch to fastfetch".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "neofetch_config".to_string(),
                    description: "Optional Neofetch configuration to migrate".to_string(),
                    required: Some(false),
                },
            ]),
        },
    ]
}

pub async fn get_prompt(name: &str, arguments: Option<Value>) -> Result<GetPromptResult> {
    let args = arguments.unwrap_or(json!({}));

    match name {
        "fastfetch-config-help" => {
            let issue = args.get("issue")
                .and_then(|v| v.as_str())
                .unwrap_or("fastfetch configuration issue");
            
            let config_snippet = args.get("config_snippet")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let mut prompt_text = format!(
                "I need help with a fastfetch configuration issue:\n\n{}\n",
                issue
            );

            if !config_snippet.is_empty() {
                prompt_text.push_str(&format!("\nCurrent configuration:\n```jsonc\n{}\n```\n", config_snippet));
            }

            prompt_text.push_str(
                "\nPlease help me:\n\
                - Understand the issue\n\
                - Identify the root cause\n\
                - Suggest a fix or improvement\n\
                - Provide corrected configuration if applicable\n\n\
                Reference the fastfetch documentation:\n\
                - Configuration Guide: https://github.com/fastfetch-cli/fastfetch/wiki/Configuration\n\
                - JSON Schema: https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-root"
            );

            Ok(GetPromptResult {
                description: Some("Get help with fastfetch configuration".to_string()),
                messages: vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent::Text(prompt_text),
                    },
                ],
            })
        }
        "fastfetch-module-setup" => {
            let module = args.get("module")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            let use_case = args.get("use_case")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let mut prompt_text = format!(
                "Please help me set up the '{}' module in fastfetch.",
                module
            );

            if !use_case.is_empty() {
                prompt_text.push_str(&format!("\n\nI want to: {}", use_case));
            }

            prompt_text.push_str(
                "\n\nPlease include:\n\
                - Module configuration options\n\
                - Example configuration\n\
                - Common use cases\n\
                - Troubleshooting tips\n\n\
                Reference: https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-modules"
            );

            Ok(GetPromptResult {
                description: Some("Get help setting up a fastfetch module".to_string()),
                messages: vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent::Text(prompt_text),
                    },
                ],
            })
        }
        "fastfetch-logo-customization" => {
            let logo_name = args.get("logo_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            
            let customization_goal = args.get("customization_goal")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let mut prompt_text = "Please help me customize fastfetch logos.".to_string();

            if !logo_name.is_empty() {
                prompt_text.push_str(&format!("\n\nLogo name: {}", logo_name));
            }

            if !customization_goal.is_empty() {
                prompt_text.push_str(&format!("\n\nI want to: {}", customization_goal));
            }

            prompt_text.push_str(
                "\n\nPlease include:\n\
                - Logo configuration options\n\
                - How to customize logo appearance\n\
                - Available logo options\n\
                - Example configurations\n\n\
                Reference:\n\
                - Logo Options: https://github.com/fastfetch-cli/fastfetch/wiki/Logo-options\n\
                - Logo Schema: https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-logo"
            );

            Ok(GetPromptResult {
                description: Some("Get help customizing fastfetch logos".to_string()),
                messages: vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent::Text(prompt_text),
                    },
                ],
            })
        }
        "fastfetch-format-string-help" => {
            let desired_output = args.get("desired_output")
                .and_then(|v| v.as_str())
                .unwrap_or("custom fastfetch output format");

            let prompt_text = format!(
                "Please help me create format strings for fastfetch to achieve the following output:\n\n{}\n\n\
                Please include:\n\
                - Format string syntax\n\
                - Available format specifiers\n\
                - Example format strings\n\
                - Best practices\n\n\
                Reference: https://github.com/fastfetch-cli/fastfetch/wiki/Format-String-Guide",
                desired_output
            );

            Ok(GetPromptResult {
                description: Some("Get help with fastfetch format strings".to_string()),
                messages: vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent::Text(prompt_text),
                    },
                ],
            })
        }
        "fastfetch-color-configuration" => {
            let color_scheme = args.get("color_scheme")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let mut prompt_text = "Please help me configure colors in fastfetch.".to_string();

            if !color_scheme.is_empty() {
                prompt_text.push_str(&format!("\n\nI want to use: {}", color_scheme));
            }

            prompt_text.push_str(
                "\n\nPlease include:\n\
                - Color format specifications\n\
                - How to set colors in configuration\n\
                - Available color options\n\
                - Example color configurations\n\n\
                Reference: https://github.com/fastfetch-cli/fastfetch/wiki/Color-Format-Specification"
            );

            Ok(GetPromptResult {
                description: Some("Get help configuring colors in fastfetch".to_string()),
                messages: vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent::Text(prompt_text),
                    },
                ],
            })
        }
        "fastfetch-migrate-neofetch" => {
            let neofetch_config = args.get("neofetch_config")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let mut prompt_text = "Please help me migrate from Neofetch to fastfetch.".to_string();

            if !neofetch_config.is_empty() {
                prompt_text.push_str(&format!("\n\nMy Neofetch configuration:\n```bash\n{}\n```", neofetch_config));
            }

            prompt_text.push_str(
                "\n\nPlease help me:\n\
                - Convert Neofetch configuration to fastfetch format\n\
                - Understand differences between Neofetch and fastfetch\n\
                - Migrate logo configurations\n\
                - Preserve customization preferences\n\n\
                Reference: https://github.com/fastfetch-cli/fastfetch/wiki/Migrate-Neofetch-Logo-To-Fastfetch"
            );

            Ok(GetPromptResult {
                description: Some("Get help migrating from Neofetch to fastfetch".to_string()),
                messages: vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent::Text(prompt_text),
                    },
                ],
            })
        }
        _ => {
            Err(anyhow::anyhow!("Unknown prompt: {}", name))
        }
    }
}
