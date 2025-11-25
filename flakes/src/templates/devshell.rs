pub fn devshell_template(
    name: &str,
    description: &str,
    custom_inputs: &[crate::models::scaffold_result::InputSpec],
) -> String {
    let mut inputs = String::from("    nixpkgs.url = \"github:NixOS/nixpkgs/nixos-unstable\";\n    flake-utils.url = \"github:numtide/flake-utils\";\n");
    
    let mut has_flake_utils = false;
    for input in custom_inputs {
        if input.name == "flake-utils" {
            has_flake_utils = true;
        }
        if input.name != "nixpkgs" && input.name != "flake-utils" {
            inputs.push_str(&format!("    {}.url = \"{}\";\n", input.name, input.url));
        }
    }

    let mut input_names = vec!["self", "nixpkgs"];
    if has_flake_utils || custom_inputs.iter().any(|i| i.name == "flake-utils") {
        input_names.push("flake-utils");
    }
    for input in custom_inputs {
        if input.name != "nixpkgs" && input.name != "flake-utils" {
            input_names.push(&input.name);
        }
    }
    let inputs_str = input_names.join(", ");

    format!(
        r#"{{
  description = "{}";

  inputs = {{
{}
  }};

  outputs = {{ {} }}:
    flake-utils.lib.eachDefaultSystem (system: {{
      devShells.default = nixpkgs.legacyPackages.${{system}}.mkShell {{
        name = "{}";
        buildInputs = with nixpkgs.legacyPackages.${{system}}; [
          # Add your development dependencies here
        ];
        shellHook = '''';
          echo "Welcome to {} development shell"
        '';
      }};
    }});
}}
"#,
        description, inputs, inputs_str, name, name
    )
}

