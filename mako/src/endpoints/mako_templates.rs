use crate::models::MakoTemplate;
use crate::utils::logger::EndpointLogger;

/// Generate Mako config snippets for common use cases
pub fn get_templates(use_case: Option<&str>) -> Vec<MakoTemplate> {
    let _logger = EndpointLogger::new("mako_templates");
    
    let all_templates = vec![
        MakoTemplate {
            template_name: "minimal".to_string(),
            snippet: "[default]\nfont=monospace 10\nbackground-color=#285577\ntext-color=#ffffff\n".to_string(),
            description: "Minimal configuration with basic colors and font".to_string(),
        },
        MakoTemplate {
            template_name: "persistent".to_string(),
            snippet: "[default]\nfont=monospace 10\nbackground-color=#285577\ntext-color=#ffffff\ndefault-timeout=0\nignore-timeout=1\nhistory=1\n".to_string(),
            description: "Notifications that persist until manually dismissed, with history enabled".to_string(),
        },
        MakoTemplate {
            template_name: "colored".to_string(),
            snippet: "[default]\nfont=monospace 12\nbackground-color=#1e1e2e\ntext-color=#cdd6f4\nborder-color=#89b4fa\nborder-size=2\nborder-radius=10\nprogress-color=over #89b4fa #89b4fa\n".to_string(),
            description: "Colorful configuration with rounded borders and custom colors".to_string(),
        },
        MakoTemplate {
            template_name: "positional".to_string(),
            snippet: "[default]\nfont=monospace 10\nbackground-color=#285577\ntext-color=#ffffff\nanchor=bottom-right\nmargin=20\nlayer=overlay\n".to_string(),
            description: "Positioned at bottom-right with custom margin and overlay layer".to_string(),
        },
        MakoTemplate {
            template_name: "compact".to_string(),
            snippet: "[default]\nfont=monospace 9\nbackground-color=#2d2d2d\ntext-color=#ffffff\nwidth=250\nheight=80\npadding=8\nmargin=10\nmax-visible=3\n".to_string(),
            description: "Compact notifications with smaller dimensions and fewer visible".to_string(),
        },
        MakoTemplate {
            template_name: "grouped".to_string(),
            snippet: "[default]\nfont=monospace 10\nbackground-color=#285577\ntext-color=#ffffff\ngroup-by=app-name\nmax-visible=5\n".to_string(),
            description: "Notifications grouped by application name".to_string(),
        },
        MakoTemplate {
            template_name: "no-markup".to_string(),
            snippet: "[default]\nfont=monospace 10\nbackground-color=#285577\ntext-color=#ffffff\nmarkup=0\n".to_string(),
            description: "Disable markup parsing for plain text notifications".to_string(),
        },
        MakoTemplate {
            template_name: "full-featured".to_string(),
            snippet: "[default]\nfont=monospace 11\nbackground-color=#1e1e2e\ntext-color=#cdd6f4\nborder-color=#89b4fa\nborder-size=2\nborder-radius=12\nwidth=350\nheight=120\npadding=12\nmargin=15\nmax-visible=5\ndefault-timeout=5000\nicons=1\nmax-icon-size=64\nhistory=1\ngroup-by=app-name\nanchor=top-right\nlayer=overlay\n".to_string(),
            description: "Full-featured configuration with all common options enabled".to_string(),
        },
    ];

    if let Some(case) = use_case {
        let case_lower = case.to_lowercase();
        all_templates
            .into_iter()
            .filter(|t| t.template_name.to_lowercase() == case_lower)
            .collect()
    } else {
        all_templates
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_templates_no_filter() {
        let templates = get_templates(None);
        assert!(!templates.is_empty());
        assert!(templates.len() >= 5);
    }

    #[test]
    fn test_get_templates_with_use_case() {
        let templates = get_templates(Some("minimal"));
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].template_name, "minimal");
    }

    #[test]
    fn test_get_templates_case_insensitive() {
        let templates_lower = get_templates(Some("minimal"));
        let templates_upper = get_templates(Some("MINIMAL"));
        assert_eq!(templates_lower.len(), templates_upper.len());
    }

    #[test]
    fn test_get_templates_nonexistent() {
        let templates = get_templates(Some("nonexistent"));
        assert!(templates.is_empty());
    }

    #[test]
    fn test_template_structure() {
        let templates = get_templates(Some("minimal"));
        let template = &templates[0];

        assert_eq!(template.template_name, "minimal");
        assert!(!template.snippet.is_empty());
        assert!(!template.description.is_empty());
        assert!(template.snippet.contains("[default]"));
    }

    #[test]
    fn test_all_templates_have_valid_snippets() {
        let templates = get_templates(None);
        for template in templates {
            assert!(template.snippet.contains("[default]"));
            assert!(!template.snippet.trim().is_empty());
        }
    }
}
