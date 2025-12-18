use anyhow::Result;

#[derive(Debug, serde::Serialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "mimeType")]
    pub mime_type: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct ReadResourceResult {
    pub contents: Vec<Content>,
}

#[derive(Debug, serde::Serialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: String,
}

pub fn list_resources() -> Vec<Resource> {
    vec![
        Resource {
            uri: "fastfetch://wiki/configuration".to_string(),
            name: "Fastfetch Configuration Guide".to_string(),
            description: Some("Complete guide to configuring fastfetch".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/json-schema-root".to_string(),
            name: "Fastfetch JSON Schema - Root".to_string(),
            description: Some("Root JSON schema documentation for fastfetch configuration".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/json-schema-schema".to_string(),
            name: "Fastfetch JSON Schema - $schema".to_string(),
            description: Some("Documentation for the $schema field in fastfetch configuration".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/json-schema-logo".to_string(),
            name: "Fastfetch JSON Schema - Logo".to_string(),
            description: Some("Documentation for logo configuration in fastfetch".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/json-schema-general".to_string(),
            name: "Fastfetch JSON Schema - General".to_string(),
            description: Some("Documentation for general configuration options in fastfetch".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/json-schema-display".to_string(),
            name: "Fastfetch JSON Schema - Display".to_string(),
            description: Some("Documentation for display configuration in fastfetch".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/json-schema-modules".to_string(),
            name: "Fastfetch JSON Schema - Modules".to_string(),
            description: Some("Documentation for module configuration in fastfetch".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/logo-options".to_string(),
            name: "Fastfetch Logo Options".to_string(),
            description: Some("Documentation for logo options and customization".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/format-string-guide".to_string(),
            name: "Fastfetch Format String Guide".to_string(),
            description: Some("Complete guide to format strings in fastfetch".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/color-format-specification".to_string(),
            name: "Fastfetch Color Format Specification".to_string(),
            description: Some("Documentation for color format specifications in fastfetch".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/migrate-neofetch-logo".to_string(),
            name: "Migrate Neofetch Logo to Fastfetch".to_string(),
            description: Some("Guide for migrating Neofetch logo configurations to fastfetch".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/dependencies".to_string(),
            name: "Fastfetch Dependencies".to_string(),
            description: Some("Documentation about fastfetch dependencies and requirements".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://wiki/building".to_string(),
            name: "Building Fastfetch".to_string(),
            description: Some("Guide for building fastfetch from source".to_string()),
            mime_type: Some("text/markdown".to_string()),
        },
        Resource {
            uri: "fastfetch://manpage".to_string(),
            name: "Fastfetch Manual Page".to_string(),
            description: Some("Arch Linux manual page for fastfetch".to_string()),
            mime_type: Some("text/plain".to_string()),
        },
        Resource {
            uri: "fastfetch://github".to_string(),
            name: "Fastfetch GitHub Repository".to_string(),
            description: Some("Link to the fastfetch GitHub repository".to_string()),
            mime_type: Some("text/plain".to_string()),
        },
    ]
}

pub async fn read_resource(uri: &str) -> Result<ReadResourceResult> {
    let content = match uri {
        "fastfetch://wiki/configuration" => {
            "Fastfetch Configuration Guide\n\n\
            Complete guide to configuring fastfetch:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Configuration\n\n\
            This guide covers all aspects of configuring fastfetch, including:\n\
            - Configuration file location and format\n\
            - Basic configuration options\n\
            - Advanced customization\n\
            - Module configuration\n\
            - Logo customization"
                .to_string()
        }
        "fastfetch://wiki/json-schema-root" => {
            "Fastfetch JSON Schema - Root\n\n\
            Root JSON schema documentation:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-root\n\n\
            This documentation covers the root structure of the fastfetch JSON schema."
                .to_string()
        }
        "fastfetch://wiki/json-schema-schema" => {
            "Fastfetch JSON Schema - $schema\n\n\
            Documentation for the $schema field:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-%24schema\n\n\
            Learn about the $schema field and how to use it in your configuration."
                .to_string()
        }
        "fastfetch://wiki/json-schema-logo" => {
            "Fastfetch JSON Schema - Logo\n\n\
            Logo configuration documentation:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-logo\n\n\
            Complete guide to configuring logos in fastfetch."
                .to_string()
        }
        "fastfetch://wiki/json-schema-general" => {
            "Fastfetch JSON Schema - General\n\n\
            General configuration options:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-general\n\n\
            Documentation for general fastfetch configuration options."
                .to_string()
        }
        "fastfetch://wiki/json-schema-display" => {
            "Fastfetch JSON Schema - Display\n\n\
            Display configuration documentation:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-display\n\n\
            Learn how to configure display options in fastfetch."
                .to_string()
        }
        "fastfetch://wiki/json-schema-modules" => {
            "Fastfetch JSON Schema - Modules\n\n\
            Module configuration documentation:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Json-Schema-modules\n\n\
            Complete guide to configuring modules in fastfetch."
                .to_string()
        }
        "fastfetch://wiki/logo-options" => {
            "Fastfetch Logo Options\n\n\
            Logo options and customization:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Logo-options\n\n\
            Learn about all available logo options and how to customize them."
                .to_string()
        }
        "fastfetch://wiki/format-string-guide" => {
            "Fastfetch Format String Guide\n\n\
            Complete format string guide:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Format-String-Guide\n\n\
            Learn how to use format strings in fastfetch to customize your output."
                .to_string()
        }
        "fastfetch://wiki/color-format-specification" => {
            "Fastfetch Color Format Specification\n\n\
            Color format documentation:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Color-Format-Specification\n\n\
            Complete guide to color format specifications in fastfetch."
                .to_string()
        }
        "fastfetch://wiki/migrate-neofetch-logo" => {
            "Migrate Neofetch Logo to Fastfetch\n\n\
            Migration guide:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Migrate-Neofetch-Logo-To-Fastfetch\n\n\
            Step-by-step guide for migrating Neofetch logo configurations to fastfetch."
                .to_string()
        }
        "fastfetch://wiki/dependencies" => {
            "Fastfetch Dependencies\n\n\
            Dependencies documentation:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Dependencies\n\n\
            Information about fastfetch dependencies and requirements."
                .to_string()
        }
        "fastfetch://wiki/building" => {
            "Building Fastfetch\n\n\
            Building from source:\n\
            https://github.com/fastfetch-cli/fastfetch/wiki/Building\n\n\
            Complete guide for building fastfetch from source."
                .to_string()
        }
        "fastfetch://manpage" => {
            "Fastfetch Manual Page\n\n\
            Arch Linux manual page:\n\
            https://man.archlinux.org/man/extra/fastfetch/fastfetch.1.en\n\n\
            Official manual page for fastfetch command-line options and usage."
                .to_string()
        }
        "fastfetch://github" => {
            "Fastfetch GitHub Repository\n\n\
            Main repository:\n\
            https://github.com/LierB/fastfetch\n\n\
            Official fastfetch GitHub repository with source code, issues, and releases."
                .to_string()
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown resource URI: {}", uri));
        }
    };

    Ok(ReadResourceResult {
        contents: vec![Content {
            content_type: "text".to_string(),
            text: content,
        }],
    })
}
