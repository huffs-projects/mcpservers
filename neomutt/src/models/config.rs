use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigOption {
    pub name: String,
    pub description: String,
    pub r#type: OptionType,
    pub default: Option<String>,
    pub example: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionType {
    Boolean,
    String,
    Number,
    Path,
    QuadOption, // yes, no, ask-yes, ask-no
    Enum(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigCommand {
    pub command: String,
    pub option: Option<String>,
    pub value: Option<String>,
    pub line_number: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAccount {
    pub email: String,
    pub real_name: Option<String>,
    pub imap_server: String,
    pub imap_port: u16,
    pub imap_user: Option<String>,
    pub imap_pass: Option<String>,
    pub smtp_server: String,
    pub smtp_port: u16,
    pub smtp_user: Option<String>,
    pub smtp_pass: Option<String>,
    pub use_ssl: bool,
    pub use_starttls: bool,
}

impl EmailAccount {
    pub fn to_muttrc(&self) -> String {
        let mut config = String::with_capacity(512); // Pre-allocate capacity
        
        config.push_str(&format!("set from = \"{}\"\n", self.email));
        if let Some(ref name) = self.real_name {
            config.push_str(&format!("set real_name = \"{}\"\n", name));
        }
        
        config.push_str(&format!("set imap_user = \"{}\"\n", 
            self.imap_user.as_ref().unwrap_or(&self.email)));
        
        // Security warning for passwords
        if self.imap_pass.is_some() {
            config.push_str("# WARNING: Plain text passwords are insecure!\n");
            config.push_str("# Consider using: set imap_pass = \"`gpg --batch -q --decrypt ~/.neomutt/pass.gpg`\"\n");
            config.push_str("# Or use an external password manager like 'pass'\n");
            // Don't include actual password in output for security
        }
        
        config.push_str(&format!("set folder = \"imap://{}:{}\"\n", 
            self.imap_server, self.imap_port));
        
        config.push_str(&format!("set smtp_url = \"smtp://{}@{}:{}/\"\n",
            self.smtp_user.as_ref().unwrap_or(&self.email),
            self.smtp_server,
            self.smtp_port));
        
        // Security warning for SMTP passwords
        if self.smtp_pass.is_some() {
            config.push_str("# WARNING: Plain text passwords are insecure!\n");
            config.push_str("# Consider using: set smtp_pass = \"`gpg --batch -q --decrypt ~/.neomutt/smtp-pass.gpg`\"\n");
            // Don't include actual password in output for security
        }
        
        if self.use_ssl {
            config.push_str("set ssl_force_tls = yes\n");
        }
        if self.use_starttls {
            config.push_str("set ssl_starttls = yes\n");
        }
        
        config
    }
}

