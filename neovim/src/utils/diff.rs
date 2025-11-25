
/// Unified diff generator and AST-aware diff modes
pub struct DiffGenerator;

/// Unified diff hunk
#[derive(Debug, Clone)]
struct Hunk {
    old_start: usize,
    old_count: usize,
    new_start: usize,
    new_count: usize,
    lines: Vec<HunkLine>,
}

/// Line in a diff hunk
#[derive(Debug, Clone)]
pub enum HunkLine {
    Context(String),
    Removed(String),
    Added(String),
}

impl DiffGenerator {
    /// Generate unified diff between two strings
    pub fn unified_diff(
        original: &str,
        modified: &str,
        original_path: &str,
        modified_path: &str,
    ) -> String {
        let original_lines: Vec<&str> = original.lines().collect();
        let modified_lines: Vec<&str> = modified.lines().collect();

        let mut diff = format!("--- {}\n+++ {}\n", original_path, modified_path);

        // Use a simple line-based diff algorithm
        let hunks = Self::compute_hunks(&original_lines, &modified_lines);
        
        for hunk in hunks {
            diff.push_str(&format!(
                "@@ -{},{} +{},{} @@\n",
                hunk.old_start, hunk.old_count,
                hunk.new_start, hunk.new_count
            ));
            
            for line in &hunk.lines {
                match line {
                    HunkLine::Context(s) => {
                        diff.push_str(&format!(" {}\n", s));
                    }
                    HunkLine::Removed(s) => {
                        diff.push_str(&format!("-{}\n", s));
                    }
                    HunkLine::Added(s) => {
                        diff.push_str(&format!("+{}\n", s));
                    }
                }
            }
        }

        diff
    }

    /// Compute hunks from line differences
    fn compute_hunks(original: &[&str], modified: &[&str]) -> Vec<Hunk> {
        let mut hunks = Vec::new();
        let mut i = 0;
        let mut j = 0;
        let max_iterations = original.len() + modified.len() + 100; // Safety limit
        let mut iterations = 0;

        while (i < original.len() || j < modified.len()) && iterations < max_iterations {
            iterations += 1;
            
            // Find the next difference
            let old_start = i;
            let new_start = j;

            // Skip common prefix
            while i < original.len() && j < modified.len() && original[i] == modified[j] {
                i += 1;
                j += 1;
            }

            // If we've reached the end, we're done
            if i >= original.len() && j >= modified.len() {
                break;
            }

            // Find the end of the difference by advancing until we find common lines again
            let mut old_end = i;
            let mut new_end = j;

            // Advance through the difference region
            while old_end < original.len() && new_end < modified.len() {
                if original[old_end] == modified[new_end] {
                    break; // Found common line, end of difference
                }
                old_end += 1;
                new_end += 1;
            }

            // If one side is exhausted, the difference extends to the end
            if old_end >= original.len() || new_end >= modified.len() {
                // Difference extends to end of one or both files
            }

            // Build hunk lines
            let mut lines = Vec::new();
            
            // Add context before (limit to 3 lines)
            if old_start > 0 {
                let context_start = old_start.saturating_sub(3);
                for k in context_start..old_start {
                    if k < original.len() {
                        lines.push(HunkLine::Context(original[k].to_string()));
                    }
                }
            }

            // Add removed lines
            for k in old_start..old_end {
                if k < original.len() {
                    lines.push(HunkLine::Removed(original[k].to_string()));
                }
            }

            // Add added lines
            for k in new_start..new_end {
                if k < modified.len() {
                    lines.push(HunkLine::Added(modified[k].to_string()));
                }
            }

            // Add context after (limit to 3 lines)
            if old_end < original.len() {
                let context_end = (old_end + 3).min(original.len());
                for k in old_end..context_end {
                    if k < original.len() {
                        lines.push(HunkLine::Context(original[k].to_string()));
                    }
                }
            }

            if !lines.is_empty() {
                hunks.push(Hunk {
                    old_start: old_start + 1, // 1-indexed
                    old_count: old_end - old_start,
                    new_start: new_start + 1, // 1-indexed
                    new_count: new_end - new_start,
                    lines,
                });
            }

            // Ensure we make progress
            if i == old_end && j == new_end {
                // No progress made, force advancement
                if i < original.len() {
                    i += 1;
                }
                if j < modified.len() {
                    j += 1;
                }
            } else {
                i = old_end;
                j = new_end;
            }
        }

        hunks
    }

    /// Parse unified diff format
    pub fn parse_unified_diff(diff: &str) -> Result<UnifiedDiff, String> {
        let lines: Vec<&str> = diff.lines().collect();
        if lines.is_empty() {
            return Err("Empty diff".to_string());
        }

        // Parse header
        let mut idx = 0;
        let old_file = if idx < lines.len() && lines[idx].starts_with("--- ") {
            let path = lines[idx][4..].trim();
            idx += 1;
            path.to_string()
        } else {
            return Err("Missing '---' header in diff".to_string());
        };

        let new_file = if idx < lines.len() && lines[idx].starts_with("+++ ") {
            let path = lines[idx][4..].trim();
            idx += 1;
            path.to_string()
        } else {
            return Err("Missing '+++' header in diff".to_string());
        };

        let mut hunks = Vec::new();

        // Parse hunks
        while idx < lines.len() {
            if lines[idx].starts_with("@@") {
                let hunk = Self::parse_hunk(&lines, &mut idx)?;
                hunks.push(hunk);
            } else {
                idx += 1;
            }
        }

        Ok(UnifiedDiff {
            old_file,
            new_file,
            hunks,
        })
    }

    /// Parse a single hunk
    fn parse_hunk(lines: &[&str], idx: &mut usize) -> Result<ParsedHunk, String> {
        let hunk_line = lines[*idx];
        if !hunk_line.starts_with("@@") || !hunk_line.ends_with("@@") {
            return Err(format!("Invalid hunk header: {}", hunk_line));
        }

        // Parse @@ -old_start,old_count +new_start,new_count @@
        let hunk_content = &hunk_line[2..hunk_line.len() - 2].trim();
        let parts: Vec<&str> = hunk_content.split_whitespace().collect();
        
        if parts.len() < 2 {
            return Err(format!("Invalid hunk format: {}", hunk_line));
        }

        let old_part = parts[0];
        let new_part = parts[1];

        let (old_start, old_count) = Self::parse_range(old_part)?;
        let (new_start, new_count) = Self::parse_range(new_part)?;

        *idx += 1;

        let mut hunk_lines = Vec::new();
        while *idx < lines.len() && !lines[*idx].starts_with("@@") {
            let line = lines[*idx];
            if line.starts_with(' ') {
                hunk_lines.push(HunkLine::Context(line[1..].to_string()));
            } else if line.starts_with('-') {
                hunk_lines.push(HunkLine::Removed(line[1..].to_string()));
            } else if line.starts_with('+') {
                hunk_lines.push(HunkLine::Added(line[1..].to_string()));
            } else if line.is_empty() {
                // Empty line, treat as context
                hunk_lines.push(HunkLine::Context(String::new()));
            } else {
                return Err(format!("Invalid diff line: {}", line));
            }
            *idx += 1;
        }

        Ok(ParsedHunk {
            old_start,
            old_count,
            new_start,
            new_count,
            lines: hunk_lines,
        })
    }

    /// Parse range like "-5,3" or "+10,2"
    fn parse_range(s: &str) -> Result<(usize, usize), String> {
        let s = s.trim_start_matches(['-', '+']);
        if let Some(comma_pos) = s.find(',') {
            let start = s[..comma_pos].parse::<usize>()
                .map_err(|_| format!("Invalid range start: {}", s))?;
            let count = s[comma_pos + 1..].parse::<usize>()
                .map_err(|_| format!("Invalid range count: {}", s))?;
            Ok((start, count))
        } else {
            // Single number means count of 1
            let start = s.parse::<usize>()
                .map_err(|_| format!("Invalid range: {}", s))?;
            Ok((start, 1))
        }
    }

    /// Apply a unified diff to a string
    pub fn apply_diff(original: &str, diff: &str) -> Result<String, String> {
        let parsed = Self::parse_unified_diff(diff)?;
        let original_lines: Vec<&str> = original.lines().collect();
        let mut result_lines = Vec::new();
        let mut original_idx = 0;

        for hunk in &parsed.hunks {
            // Add lines before hunk
            let hunk_start = hunk.old_start.saturating_sub(1); // Convert to 0-indexed
            while original_idx < hunk_start && original_idx < original_lines.len() {
                result_lines.push(original_lines[original_idx].to_string());
                original_idx += 1;
            }

            // Process hunk
            let mut hunk_original_idx = hunk_start;
            for line in &hunk.lines {
                match line {
                    HunkLine::Context(s) => {
                        // Verify context matches
                        if hunk_original_idx < original_lines.len()
                            && original_lines[hunk_original_idx] == s.as_str()
                        {
                            result_lines.push(s.clone());
                            hunk_original_idx += 1;
                            original_idx += 1;
                        } else {
                            return Err(format!(
                                "Context mismatch at line {}: expected '{}', found '{}'",
                                hunk_original_idx + 1,
                                s,
                                original_lines.get(hunk_original_idx).unwrap_or(&"<EOF>")
                            ));
                        }
                    }
                    HunkLine::Removed(s) => {
                        // Verify removed line matches
                        if hunk_original_idx < original_lines.len()
                            && original_lines[hunk_original_idx] == s.as_str()
                        {
                            // Don't add to result (line is removed)
                            hunk_original_idx += 1;
                            original_idx += 1;
                        } else {
                            return Err(format!(
                                "Removed line mismatch at line {}: expected '{}', found '{}'",
                                hunk_original_idx + 1,
                                s,
                                original_lines.get(hunk_original_idx).unwrap_or(&"<EOF>")
                            ));
                        }
                    }
                    HunkLine::Added(s) => {
                        // Add new line
                        result_lines.push(s.clone());
                        // Don't advance original_idx (this is a new line)
                    }
                }
            }

            // Skip remaining lines in hunk range
            while hunk_original_idx < hunk_start + hunk.old_count
                && hunk_original_idx < original_lines.len()
            {
                hunk_original_idx += 1;
                original_idx += 1;
            }
        }

        // Add remaining lines
        while original_idx < original_lines.len() {
            result_lines.push(original_lines[original_idx].to_string());
            original_idx += 1;
        }

        Ok(result_lines.join("\n"))
    }

    /// Generate AST-aware diff (simplified - would compare AST nodes)
    pub fn ast_aware_diff(
        original_ast: &str,
        modified_ast: &str,
    ) -> String {
        // In a full implementation, this would compare AST nodes
        // For now, fall back to unified diff
        Self::unified_diff(original_ast, modified_ast, "original", "modified")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_diff_generation() {
        let original = "line1\nline2\nline3";
        let modified = "line1\nline2_modified\nline3";
        let diff = DiffGenerator::unified_diff(original, modified, "old", "new");
        
        assert!(diff.contains("--- old"), "Diff should contain '--- old'. Diff:\n{}", diff);
        assert!(diff.contains("+++ new"), "Diff should contain '+++ new'. Diff:\n{}", diff);
        // The diff might have context lines, so check for the removed line pattern
        assert!(diff.contains("-line2") || diff.contains("\n-line2\n"), 
                "Diff should contain '-line2'. Diff:\n{}", diff);
        assert!(diff.contains("+line2_modified") || diff.contains("\n+line2_modified\n"), 
                "Diff should contain '+line2_modified'. Diff:\n{}", diff);
    }

    #[test]
    fn test_parse_unified_diff() {
        let diff = "--- old.txt\n+++ new.txt\n@@ -1,2 +1,2 @@\n line1\n-line2\n+line2_modified\n";
        let parsed = DiffGenerator::parse_unified_diff(diff);
        assert!(parsed.is_ok(), "Should parse valid unified diff");
        
        let parsed = parsed.unwrap();
        assert_eq!(parsed.old_file, "old.txt");
        assert_eq!(parsed.new_file, "new.txt");
        assert_eq!(parsed.hunks.len(), 1);
    }

    #[test]
    fn test_apply_diff() {
        let original = "line1\nline2\nline3";
        // Diff needs context lines for proper application
        let diff = "--- old.txt\n+++ new.txt\n@@ -1,3 +1,3 @@\n line1\n-line2\n+line2_modified\n line3\n";
        let result = DiffGenerator::apply_diff(original, diff);
        
        assert!(result.is_ok(), "Should apply diff successfully. Error: {:?}", result);
        let result = result.unwrap();
        assert!(result.contains("line2_modified"), "Result should contain line2_modified. Result: {}", result);
        assert!(!result.contains("\nline2\n"), "Result should not contain line2. Result: {}", result);
    }

    #[test]
    fn test_apply_diff_with_context() {
        let original = "line1\nline2\nline3\nline4";
        let diff = "--- old.txt\n+++ new.txt\n@@ -2,2 +2,2 @@\n line2\n-line3\n+line3_modified\n";
        let result = DiffGenerator::apply_diff(original, diff);
        
        assert!(result.is_ok(), "Should apply diff with context");
        let result = result.unwrap();
        assert!(result.contains("line3_modified"));
        assert!(!result.contains("\nline3\n"));
    }
}

/// Parsed unified diff structure
#[derive(Debug, Clone)]
pub struct UnifiedDiff {
    pub old_file: String,
    pub new_file: String,
    pub hunks: Vec<ParsedHunk>,
}

/// Parsed hunk structure
#[derive(Debug, Clone)]
pub struct ParsedHunk {
    pub old_start: usize,
    pub old_count: usize,
    pub new_start: usize,
    pub new_count: usize,
    pub lines: Vec<HunkLine>,
}
