pub fn nixos_template(
    _name: &str,
    description: &str,
    custom_inputs: &[crate::models::scaffold_result::InputSpec],
) -> String {
    let mut inputs = String::from("    nixpkgs.url = \"github:NixOS/nixpkgs/nixos-unstable\";\n");
    
    for input in custom_inputs {
        if input.name != "nixpkgs" {
            inputs.push_str(&format!("    {}.url = \"{}\";\n", input.name, input.url));
        }
    }

    let mut input_names = vec!["self", "nixpkgs"];
    for input in custom_inputs {
        if input.name != "nixpkgs" {
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

  outputs = {{ {} }}: {{
    nixosModules.default = {{ config, pkgs, ... }}: {{
      # Your NixOS module configuration here
      options = {{
        # Define your module options
      }};
      config = {{
        # Define your module configuration
      }};
    }};
  }};
}}
"#,
        description, inputs, inputs_str
    )
}

