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
            name: "neomutt-config-help".to_string(),
            description: "Get help with NeoMutt email client configuration issues".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "issue".to_string(),
                    description: "Description of the configuration issue or question".to_string(),
                    required: Some(true),
                },
                PromptArgument {
                    name: "config_snippet".to_string(),
                    description: "Optional NeoMutt config snippet to review".to_string(),
                    required: Some(false),
                },
            ]),
        },
        Prompt {
            name: "neomutt-account-setup".to_string(),
            description: "Get help setting up an email account in NeoMutt".to_string(),
            arguments: Some(vec![
                PromptArgument {
                    name: "email_provider".to_string(),
                    description: "Email provider name (e.g., 'gmail', 'outlook')".to_string(),
                    required: Some(false),
                },
            ]),
        },
    ]
}

pub async fn get_prompt(name: &str, arguments: Option<Value>) -> Result<GetPromptResult> {
    let args = arguments.unwrap_or(json!({}));

    match name {
        "neomutt-config-help" => {
            let issue = args.get("issue")
                .and_then(|v| v.as_str())
                .unwrap_or("NeoMutt configuration issue");
            
            let config_snippet = args.get("config_snippet")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            let mut prompt_text = format!(
                "I need help with a NeoMutt email client configuration issue:\n\n{}\n",
                issue
            );

            if !config_snippet.is_empty() {
                prompt_text.push_str(&format!("\nCurrent configuration:\n```\n{}\n```\n", config_snippet));
            }

            prompt_text.push_str(
                "\nPlease help me:\n\
                - Understand the issue\n\
                - Identify the root cause\n\
                - Suggest a fix or improvement\n\
                - Provide corrected configuration if applicable"
            );

            Ok(GetPromptResult {
                description: Some("Get help with NeoMutt configuration".to_string()),
                messages: vec![
                    PromptMessage {
                        role: "user".to_string(),
                        content: PromptMessageContent::Text(prompt_text),
                    },
                ],
            })
        }
        "neomutt-account-setup" => {
            let email_provider = args.get("email_provider")
                .and_then(|v| v.as_str())
                .unwrap_or("");

                let prompt_text: String = if email_provider.is_empty() {
                "Please help me set up an email account in NeoMutt. Include:\n\
                - IMAP/SMTP configuration\n\
                - Authentication setup\n\
                - Common provider settings\n\
                - Security best practices".to_string()
            } else {
                format!(
                    "Please help me set up a {} email account in NeoMutt. Include:\n\
                    - IMAP/SMTP server settings\n\
                    - Authentication configuration\n\
                    - Port numbers and security settings\n\
                    - Example configuration",
                    email_provider
                )
            };

            Ok(GetPromptResult {
                description: Some("Get help with NeoMutt account setup".to_string()),
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

