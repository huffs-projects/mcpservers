/// Diagnostic severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Info,
    Hint,
}

/// LSP-like diagnostic for configuration errors
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub range: (usize, usize), // (start, end) byte offsets
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub code: Option<String>,
}

impl Diagnostic {
    pub fn error(message: String) -> Self {
        Self {
            range: (0, 0),
            severity: DiagnosticSeverity::Error,
            message,
            code: None,
        }
    }

    pub fn warning(message: String) -> Self {
        Self {
            range: (0, 0),
            severity: DiagnosticSeverity::Warning,
            message,
            code: None,
        }
    }

    pub fn with_range(mut self, start: usize, end: usize) -> Self {
        self.range = (start, end);
        self
    }

    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
}

/// Collection of diagnostics with helper methods
#[derive(Debug, Default)]
pub struct DiagnosticCollection {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCollection {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn add_error(&mut self, message: String) {
        self.add(Diagnostic::error(message));
    }

    pub fn add_warning(&mut self, message: String) {
        self.add(Diagnostic::warning(message));
    }

    pub fn errors(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Error))
            .collect()
    }

    pub fn warnings(&self) -> Vec<&Diagnostic> {
        self.diagnostics
            .iter()
            .filter(|d| matches!(d.severity, DiagnosticSeverity::Warning))
            .collect()
    }

    pub fn all(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|d| matches!(d.severity, DiagnosticSeverity::Error))
    }

    pub fn is_empty(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

