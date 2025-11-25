use anyhow::Result;
use std::collections::HashSet;

pub fn compute_unified_diff(old_content: &str, new_content: &str) -> String {
    let old_lines: Vec<&str> = old_content.lines().collect();
    let new_lines: Vec<&str> = new_content.lines().collect();
    
    let mut diff = String::from("--- original\n+++ modified\n");
    
    let old_set: HashSet<&str> = old_lines.iter().cloned().collect();
    let new_set: HashSet<&str> = new_lines.iter().cloned().collect();
    
    let added: Vec<&str> = new_set.difference(&old_set).cloned().collect();
    let removed: Vec<&str> = old_set.difference(&new_set).cloned().collect();
    
    for line in removed {
        diff.push_str(&format!("-{}\n", line));
    }
    
    for line in added {
        diff.push_str(&format!("+{}\n", line));
    }
    
    diff
}

pub fn apply_patch(content: &str, patch: &str) -> Result<String> {
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let patch_lines: Vec<&str> = patch.lines().collect();
    
    for patch_line in patch_lines {
        if patch_line.starts_with('+') && !patch_line.starts_with("+++") {
            let new_line = patch_line[1..].to_string();
            lines.push(new_line);
        } else if patch_line.starts_with('-') && !patch_line.starts_with("---") {
            let remove_line = &patch_line[1..];
            lines.retain(|line| line != remove_line);
        }
    }
    
    Ok(lines.join("\n"))
}

pub fn validate_patch(patch: &str) -> Result<()> {
    let lines: Vec<&str> = patch.lines().collect();
    
    for (idx, line) in lines.iter().enumerate() {
        if line.starts_with("+++") || line.starts_with("---") {
            continue;
        }
        
        if !line.starts_with('+') && !line.starts_with('-') && !line.trim().is_empty() {
            return Err(anyhow::anyhow!(
                "Invalid patch format at line {}: expected '+' or '-' prefix",
                idx + 1
            ));
        }
    }
    
    Ok(())
}

