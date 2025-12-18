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
            uri: "neomutt-docs://*".to_string(),
            name: "NeoMutt Documentation".to_string(),
            description: Some("Access NeoMutt email client documentation".to_string()),
            mime_type: Some("text/plain".to_string()),
        },
        Resource {
            uri: "neomutt-options://*".to_string(),
            name: "NeoMutt Options Reference".to_string(),
            description: Some("Access detailed information about NeoMutt configuration options".to_string()),
            mime_type: Some("text/plain".to_string()),
        },
    ]
}

pub async fn read_resource(uri: &str) -> Result<ReadResourceResult> {
    if uri.starts_with("neomutt-docs://") {
        let path = uri.strip_prefix("neomutt-docs://").unwrap_or("");
        
        Ok(ReadResourceResult {
            contents: vec![Content {
                content_type: "text".to_string(),
                text: format!("NeoMutt documentation for: {}\n\n(Content would be fetched from NeoMutt documentation sources)", path),
            }],
        })
    } else if uri.starts_with("neomutt-options://") {
        let option_name = uri.strip_prefix("neomutt-options://").unwrap_or("");
        
        if option_name.is_empty() {
            return Err(anyhow::anyhow!("Invalid neomutt-options URI: missing option name"));
        }

        Ok(ReadResourceResult {
            contents: vec![Content {
                content_type: "text".to_string(),
                text: format!("NeoMutt option: {}\n\n(Detailed option information would be fetched from NeoMutt sources)", option_name),
            }],
        })
    } else {
        Err(anyhow::anyhow!("Unknown resource URI scheme: {}", uri))
    }
}

