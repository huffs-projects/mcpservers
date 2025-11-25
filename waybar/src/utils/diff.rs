use diff::Result as DiffResult;

pub struct DiffGenerator;

impl DiffGenerator {
    pub fn generate_json_diff(old: &str, new: &str) -> String {
        let old_lines: Vec<&str> = old.lines().collect();
        let new_lines: Vec<&str> = new.lines().collect();
        
        let mut diff = String::new();
        for result in diff::slice(&old_lines, &new_lines) {
            match result {
                DiffResult::Left(line) => {
                    diff.push_str(&format!("-{}\n", line));
                }
                DiffResult::Both(line, _) => {
                    diff.push_str(&format!(" {}\n", line));
                }
                DiffResult::Right(line) => {
                    diff.push_str(&format!("+{}\n", line));
                }
            }
        }
        diff
    }

    pub fn generate_css_diff(old: &str, new: &str) -> String {
        Self::generate_json_diff(old, new)
    }
}

