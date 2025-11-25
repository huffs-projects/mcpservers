use anyhow::Result;
use regex::Regex;

/// Abstract Syntax Tree representation of a Zsh configuration file.
/// 
/// This structure is currently unused but reserved for future advanced parsing features.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZshConfigAst {
    pub options: Vec<ZshOptionStatement>,
    pub functions: Vec<ZshFunction>,
    pub aliases: Vec<ZshAlias>,
    pub bindings: Vec<ZshBinding>,
    pub modules: Vec<String>,
    pub variables: Vec<ZshVariable>,
}

/// Represents a Zsh option statement (setopt/unsetopt).
/// 
/// Reserved for future use in advanced parsing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZshOptionStatement {
    pub name: String,
    pub enabled: bool,
    pub line_number: usize,
}

/// Represents a Zsh function definition.
/// 
/// Reserved for future use in advanced parsing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZshFunction {
    pub name: String,
    pub body: String,
    pub line_number: usize,
}

/// Represents a Zsh alias definition.
/// 
/// Reserved for future use in advanced parsing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZshAlias {
    pub name: String,
    pub value: String,
    pub line_number: usize,
}

/// Represents a Zsh key binding (bindkey).
/// 
/// Reserved for future use in advanced parsing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZshBinding {
    pub key: String,
    pub command: String,
    pub line_number: usize,
}

/// Represents a Zsh variable assignment.
/// 
/// Reserved for future use in advanced parsing.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ZshVariable {
    pub name: String,
    pub value: String,
    pub line_number: usize,
}

/// Parses a Zsh configuration file into an AST.
/// 
/// This function is currently unused but reserved for future advanced parsing features.
/// 
/// # Arguments
/// 
/// * `content` - The Zsh configuration file content
/// 
/// # Returns
/// 
/// A `ZshConfigAst` containing parsed options, functions, aliases, bindings, modules, and variables.
#[allow(dead_code)]
pub fn parse_zsh_config(content: &str) -> Result<ZshConfigAst> {
    let lines: Vec<&str> = content.lines().collect();
    let mut ast = ZshConfigAst {
        options: Vec::new(),
        functions: Vec::new(),
        aliases: Vec::new(),
        bindings: Vec::new(),
        modules: Vec::new(),
        variables: Vec::new(),
    };

    let setopt_re = Regex::new(r"(?i)^\s*(setopt|unsetopt)\s+([A-Z_][A-Z0-9_]*)\s*$").unwrap();
    let alias_re = Regex::new(r"(?i)^\s*alias\s+([A-Za-z_][A-Za-z0-9_]*)=(\S+)\s*$").unwrap();
    let bindkey_re = Regex::new(r"(?i)^\s*bindkey\s+(\S+)\s+(.+)$").unwrap();
    let autoload_re = Regex::new(r"(?i)^\s*autoload\s+([A-Za-z_][A-Za-z0-9_/]*)\s*$").unwrap();
    let module_re = Regex::new(r"(?i)^\s*zmodload\s+(?:-a\s+)?([A-Za-z_][A-Za-z0-9_/]*)\s*$").unwrap();
    let var_re = Regex::new(r"^\s*([A-Z_][A-Z0-9_]*)=(\S+)\s*$").unwrap();
    let function_re = Regex::new(r"(?i)^\s*(?:function\s+)?([A-Za-z_][A-Za-z0-9_]*)\(\)\s*\{").unwrap();

    let mut in_function = false;
    let mut function_name = String::new();
    let mut function_body = String::new();
    let mut function_start_line = 0;

    for (line_num, line) in lines.iter().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();

        if in_function {
            function_body.push_str(line);
            function_body.push('\n');
            if trimmed == "}" || trimmed.ends_with("}") {
                ast.functions.push(ZshFunction {
                    name: function_name.clone(),
                    body: function_body.clone(),
                    line_number: function_start_line,
                });
                in_function = false;
                function_name.clear();
                function_body.clear();
            }
            continue;
        }

        if let Some(caps) = function_re.captures(trimmed) {
            in_function = true;
            function_name = caps.get(1).unwrap().as_str().to_string();
            function_body = line.to_string() + "\n";
            function_start_line = line_num;
            continue;
        }

        if let Some(caps) = setopt_re.captures(trimmed) {
            let is_set = caps.get(1).unwrap().as_str().to_lowercase() == "setopt";
            let opt_name = caps.get(2).unwrap().as_str().to_string();
            ast.options.push(ZshOptionStatement {
                name: opt_name,
                enabled: is_set,
                line_number: line_num,
            });
        }

        if let Some(caps) = alias_re.captures(trimmed) {
            ast.aliases.push(ZshAlias {
                name: caps.get(1).unwrap().as_str().to_string(),
                value: caps.get(2).unwrap().as_str().to_string(),
                line_number: line_num,
            });
        }

        if let Some(caps) = bindkey_re.captures(trimmed) {
            ast.bindings.push(ZshBinding {
                key: caps.get(1).unwrap().as_str().to_string(),
                command: caps.get(2).unwrap().as_str().to_string(),
                line_number: line_num,
            });
        }

        if let Some(caps) = autoload_re.captures(trimmed) {
            ast.modules.push(caps.get(1).unwrap().as_str().to_string());
        }

        if let Some(caps) = module_re.captures(trimmed) {
            ast.modules.push(caps.get(1).unwrap().as_str().to_string());
        }

        if let Some(caps) = var_re.captures(trimmed) {
            ast.variables.push(ZshVariable {
                name: caps.get(1).unwrap().as_str().to_string(),
                value: caps.get(2).unwrap().as_str().to_string(),
                line_number: line_num,
            });
        }
    }

    Ok(ast)
}

pub fn validate_syntax(content: &str) -> Result<Vec<String>> {
    let mut errors = Vec::new();
    
    let lines: Vec<&str> = content.lines().collect();
    let setopt_re = Regex::new(r"(?i)^\s*(setopt|unsetopt)\s+([A-Z_][A-Z0-9_]*)\s*$").unwrap();
    let bad_dollar_re = Regex::new(r"\$_([^A-Za-z_])").unwrap();
    
    for (line_num, line) in lines.iter().enumerate() {
        let line_num = line_num + 1;
        let trimmed = line.trim();
        
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        
        if let Some(caps) = setopt_re.captures(trimmed) {
            let opt_name = caps.get(2).unwrap().as_str();
            if !is_valid_option(opt_name) {
                errors.push(format!("Line {}: Unrecognized option '{}'", line_num, opt_name));
            }
        }
        
        if bad_dollar_re.is_match(trimmed) {
            errors.push(format!("Line {}: Suspicious use of $_ variable", line_num));
        }
        
        if trimmed.contains("$_") && !trimmed.contains("$_=") {
            errors.push(format!("Line {}: Potential misuse of $_ variable", line_num));
        }
    }
    
    Ok(errors)
}

fn is_valid_option(name: &str) -> bool {
    let valid_options = [
        "AUTOCD", "AUTO_LIST", "AUTO_MENU", "AUTO_NAME_DIRS", "AUTO_PARAM_SLASH",
        "AUTO_PUSHD", "AUTO_REMOVE_SLASH", "AUTO_RESUME", "BANG_HIST", "BARE_GLOB_QUAL",
        "BASH_AUTO_LIST", "BEEP", "BRACE_CCL", "BSD_ECHO", "CDABLE_VARS", "CD_SILENT",
        "CHASE_DOTS", "CHASE_LINKS", "CHECK_JOBS", "CLOBBER", "COMPLETE_ALIASES",
        "COMPLETE_IN_WORD", "CORRECT", "CORRECT_ALL", "CSH_JUNKIE_HISTORY", "CSH_JUNKIE_LOOPS",
        "CSH_JUNKIE_QUOTES", "CSH_NULLCMD", "DIRSTACKSIZE", "DOT_GLOB", "EQUALS",
        "ERR_EXIT", "EXEC", "EXTENDED_GLOB", "EXTENDED_HISTORY", "FLOW_CONTROL",
        "GLOB", "GLOB_COMPLETE", "GLOB_DOTS", "GLOB_STAR_SHORT", "GLOB_SUBST",
        "HASH_CMDS", "HASH_DIRS", "HASH_LIST_ALL", "HIST_ALLOW_CLOBBER", "HIST_BEEP",
        "HIST_EXPIRE_DUPS_FIRST", "HIST_FIND_NO_DUPS", "HIST_IGNORE_ALL_DUPS",
        "HIST_IGNORE_DUPS", "HIST_IGNORE_SPACE", "HIST_LEX_WORDS", "HIST_NO_FUNCTIONS",
        "HIST_NO_STORE", "HIST_REDUCE_BLANKS", "HIST_SAVE_BY_COPY", "HIST_SAVE_NO_DUPS",
        "HIST_VERIFY", "HISTORY_IGNORE", "HUP", "IGNORE_BRACES", "IGNORE_EOF",
        "INC_APPEND_HISTORY", "INTERACTIVE_COMMENTS", "KSH_ARRAYS", "KSH_AUTOLOAD",
        "KSH_GLOB", "KSH_OPTION_PRINT", "KSH_TYPESET", "KSH_ZERO_SUBSCRIPT",
        "LIST_AMBIGUOUS", "LIST_BEEP", "LIST_PACKED", "LIST_ROWS_FIRST", "LIST_TYPES",
        "LOCAL_OPTIONS", "LOCAL_TRAPS", "LOGIN", "LONG_LIST_JOBS", "MAGIC_EQUAL_SUBST",
        "MAIL_WARNING", "MARK_DIRS", "MENU_COMPLETE", "MONITOR", "MULTIOS", "NOMATCH",
        "NOTIFY", "NULL_GLOB", "NUMERIC_GLOB_SORT", "OVERSTRIKE", "PATH_DIRS",
        "PATH_SCRIPT", "POSIX_ALIASES", "POSIX_BUILTINS", "POSIX_CD", "POSIX_IDENTIFIERS",
        "POSIX_STRINGS", "POSIX_TRAPS", "PRINT_EIGHT_BIT", "PRINT_EXIT_VALUE",
        "PRIVILEGED", "PROMPT_BANG", "PROMPT_CR", "PROMPT_PERCENT", "PROMPT_SP",
        "PROMPT_SUBST", "PUSHD_IGNORE_DUPS", "PUSHD_MINUS", "PUSHD_SILENT",
        "PUSHD_TO_HOME", "RC_EXPAND_PARAM", "RC_QUOTES", "REC_EXACT", "RESTRICTED",
        "RM_STAR_SILENT", "RM_STAR_WAIT", "SH_FILE_EXPANSION", "SH_GLOB", "SH_NULLCMD",
        "SH_OPTION_LETTERS", "SH_WORD_SPLIT", "SINGLE_COMMAND", "SINGLE_LINE_ZLE",
        "SUN_KEYBOARD_HACK", "TRANSIENT_RPROMPT", "TYPESET_SILENT", "UNSET", "VERBOSE",
        "WARN_CREATE_GLOBAL", "WARN_NESTED_VAR", "XTRACE", "ZLE",
    ];
    valid_options.contains(&name)
}

