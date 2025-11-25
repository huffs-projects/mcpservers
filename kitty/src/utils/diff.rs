pub fn generate_unified_diff(old: &str, new: &str, old_label: &str, new_label: &str) -> String {
    let mut diff_output = String::new();
    diff_output.push_str(&format!("--- {}\n", old_label));
    diff_output.push_str(&format!("+++ {}\n", new_label));
    
    let diff = diff::lines(old, new);
    let mut old_line_num = 1;
    let mut new_line_num = 1;
    let mut hunk_lines: Vec<(char, String)> = Vec::new();
    let mut hunk_start_old = 1;
    let mut hunk_start_new = 1;
    let mut in_hunk = false;
    
    for change in diff {
        match change {
            diff::Result::Both(_line, _) => {
                if in_hunk {
                    // End current hunk
                    diff_output.push_str(&format_hunk(
                        hunk_start_old,
                        old_line_num - 1,
                        hunk_start_new,
                        new_line_num - 1,
                        &hunk_lines,
                    ));
                    hunk_lines.clear();
                    in_hunk = false;
                }
                old_line_num += 1;
                new_line_num += 1;
            }
            diff::Result::Left(line) => {
                if !in_hunk {
                    hunk_start_old = old_line_num;
                    hunk_start_new = new_line_num;
                    in_hunk = true;
                }
                hunk_lines.push(('-', line.to_string()));
                old_line_num += 1;
            }
            diff::Result::Right(line) => {
                if !in_hunk {
                    hunk_start_old = old_line_num;
                    hunk_start_new = new_line_num;
                    in_hunk = true;
                }
                hunk_lines.push(('+', line.to_string()));
                new_line_num += 1;
            }
        }
    }
    
    if in_hunk {
        diff_output.push_str(&format_hunk(
            hunk_start_old,
            old_line_num - 1,
            hunk_start_new,
            new_line_num - 1,
            &hunk_lines,
        ));
    }
    
    diff_output
}

fn format_hunk(
    old_start: usize,
    old_end: usize,
    new_start: usize,
    new_end: usize,
    lines: &[(char, String)],
) -> String {
    let mut output = String::new();
    let old_count = if old_end >= old_start { old_end - old_start + 1 } else { 0 };
    let new_count = if new_end >= new_start { new_end - new_start + 1 } else { 0 };
    output.push_str(&format!("@@ -{},{} +{},{} @@\n", old_start, old_count, new_start, new_count));
    
    for (sign, line) in lines {
        output.push_str(&format!("{}{}\n", sign, line));
    }
    
    output
}

