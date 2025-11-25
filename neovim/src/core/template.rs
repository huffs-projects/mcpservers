use crate::core::model::NvimTemplate;
use std::collections::HashMap;

/// Template provider for generating Neovim config snippets
pub struct TemplateProvider {
    templates: HashMap<String, NvimTemplate>,
}

impl TemplateProvider {
    pub fn new() -> Self {
        let mut provider = Self {
            templates: HashMap::new(),
        };
        provider.initialize_templates();
        provider
    }

    fn initialize_templates(&mut self) {
        // LazyVim keymap template
        self.templates.insert("lazyvim_keymap".to_string(), NvimTemplate {
            template_name: "lazyvim_keymap".to_string(),
            language: "lua".to_string(),
            snippet: r#"return {
  {
    "<leader>xx",
    function()
      -- Your keymap code here
    end,
    desc = "Description",
  },
}"#.to_string(),
            description: "LazyVim keymap specification".to_string(),
            tags: vec!["lazyvim".to_string(), "keymap".to_string()],
            related_options: vec![],
        });

        // LazyVim plugin spec template
        self.templates.insert("lazyvim_plugin".to_string(), NvimTemplate {
            template_name: "lazyvim_plugin".to_string(),
            language: "lua".to_string(),
            snippet: r#"return {
  "plugin-name/plugin-repo",
  event = "VeryLazy",
  config = function()
    -- Plugin configuration
  end,
  dependencies = {
    -- Dependencies
  },
}"#.to_string(),
            description: "LazyVim plugin specification".to_string(),
            tags: vec!["lazyvim".to_string(), "plugin".to_string()],
            related_options: vec![],
        });

        // LSP config template
        self.templates.insert("lsp_config".to_string(), NvimTemplate {
            template_name: "lsp_config".to_string(),
            language: "lua".to_string(),
            snippet: r#"return {
  {
    "neovim/nvim-lspconfig",
    opts = {
      servers = {
        lsp_name = {
          -- LSP server configuration
        },
      },
    },
  },
}"#.to_string(),
            description: "LSP server configuration for LazyVim".to_string(),
            tags: vec!["lazyvim".to_string(), "lsp".to_string()],
            related_options: vec![],
        });

        // Treesitter config template
        self.templates.insert("treesitter_config".to_string(), NvimTemplate {
            template_name: "treesitter_config".to_string(),
            language: "lua".to_string(),
            snippet: r#"return {
  {
    "nvim-treesitter/nvim-treesitter",
    opts = {
      ensure_installed = {
        "lua",
        "vim",
        "bash",
      },
    },
  },
}"#.to_string(),
            description: "Treesitter parser configuration".to_string(),
            tags: vec!["lazyvim".to_string(), "treesitter".to_string()],
            related_options: vec![],
        });

        // Basic vim.opt template
        self.templates.insert("vim_opt_basic".to_string(), NvimTemplate {
            template_name: "vim_opt_basic".to_string(),
            language: "lua".to_string(),
            snippet: r#"vim.opt.tabstop = 4
vim.opt.shiftwidth = 4
vim.opt.expandtab = true
vim.opt.number = true
vim.opt.relativenumber = true"#.to_string(),
            description: "Basic Neovim options configuration".to_string(),
            tags: vec!["options".to_string(), "basic".to_string()],
            related_options: vec!["tabstop".to_string(), "shiftwidth".to_string(), "expandtab".to_string(), "number".to_string()],
        });

        // Telescope config template
        self.templates.insert("telescope_config".to_string(), NvimTemplate {
            template_name: "telescope_config".to_string(),
            language: "lua".to_string(),
            snippet: r#"return {
  {
    "nvim-telescope/telescope.nvim",
    opts = {
      defaults = {
        mappings = {
          i = {
            -- Custom mappings
          },
        },
      },
    },
  },
}"#.to_string(),
            description: "Telescope configuration for LazyVim".to_string(),
            tags: vec!["lazyvim".to_string(), "telescope".to_string()],
            related_options: vec![],
        });
    }

    /// Get a template by use case
    pub fn get_template(&self, use_case: &str) -> Option<&NvimTemplate> {
        self.templates.get(use_case)
    }

    /// Get templates matching tags or use case
    pub fn search_templates(&self, use_case: &str, parameters: Option<&HashMap<String, String>>) -> Vec<NvimTemplate> {
        let use_case_lower = use_case.to_lowercase();
        let mut results = Vec::new();

        // Direct match
        if let Some(template) = self.templates.get(&use_case_lower) {
            results.push(template.clone());
        }

        // Search by tags
        for template in self.templates.values() {
            if template.tags.iter().any(|tag| tag.to_lowercase().contains(&use_case_lower)) {
                if !results.iter().any(|t| t.template_name == template.template_name) {
                    results.push(template.clone());
                }
            }
        }

        // Apply parameters if provided
        if let Some(params) = parameters {
            for template in &mut results {
                let mut snippet = template.snippet.clone();
                for (key, value) in params {
                    snippet = snippet.replace(&format!("{{{}}}", key), value);
                    snippet = snippet.replace(&format!("{{{{{}}}}}", key), value);
                }
                template.snippet = snippet;
            }
        }

        results
    }

    /// Get all templates
    pub fn get_all_templates(&self) -> Vec<&NvimTemplate> {
        self.templates.values().collect()
    }
}

impl Default for TemplateProvider {
    fn default() -> Self {
        Self::new()
    }
}

