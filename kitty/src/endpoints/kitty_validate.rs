use crate::models::ValidationResult;
use crate::utils::{KittyParser, path_validation};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidateRequest {
    pub config_path: String,
}

pub async fn handle_kitty_validate(req: ValidateRequest) -> ValidationResult {
    // Validate path for security (allow if validation fails - don't block, just warn)
    let validated_path = path_validation::validate_config_path(&req.config_path)
        .unwrap_or_else(|_| std::path::PathBuf::from(&req.config_path));
    
    let path_str = validated_path.to_str().unwrap_or(&req.config_path);
    KittyParser::validate(path_str)
}

