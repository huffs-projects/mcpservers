use diff;

/// Generate unified diff between two strings
pub fn generate_diff(old: &str, new: &str, filename: &str) -> String {
    let old_lines: Vec<&str> = old.lines().collect();
    let new_lines: Vec<&str> = new.lines().collect();
    
    let mut diff_output = format!("--- {}\n+++ {}\n", filename, filename);
    
    let diffs = diff::slice(&old_lines, &new_lines);
    
    for diff in diffs {
        match diff {
            diff::Result::Left(line) => {
                diff_output.push_str(&format!("-{}\n", line));
            }
            diff::Result::Both(_, _) => {
                // Unchanged lines - could be included for context if needed
            }
            diff::Result::Right(line) => {
                diff_output.push_str(&format!("+{}\n", line));
            }
        }
    }
    
    diff_output
}

