use anyhow::Result;

/// Parse dmenu/custom mode scripts
pub fn parse_mode_script(content: &str) -> Result<ModeScriptInfo> {
    let mut info = ModeScriptInfo {
        exec: None,
        stdin_format: None,
        stdout_format: None,
        shebang: None,
    };

    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with("#!") {
            info.shebang = Some(line.to_string());
        } else if line.contains("exec=") {
            if let Some(exec) = line.split_once('=').map(|(_, v)| v.trim()) {
                info.exec = Some(exec.to_string());
            }
        }
    }

    Ok(info)
}

#[derive(Debug, Clone)]
pub struct ModeScriptInfo {
    pub exec: Option<String>,
    pub stdin_format: Option<String>,
    pub stdout_format: Option<String>,
    pub shebang: Option<String>,
}

