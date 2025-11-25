pub fn package_template(
    name: &str,
    description: &str,
    version: &str,
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

  outputs = {{ {} }}:
    let
      forAllSystems = nixpkgs.lib.genAttrs nixpkgs.lib.platforms.all;
    in
    {{
      packages = forAllSystems (system: {{
        default = nixpkgs.legacyPackages.${{system}}.stdenv.mkDerivation {{
          pname = "{}";
          version = "{}";
          src = ./.;
          buildPhase = "echo 'Build phase'";
          installPhase = "mkdir -p $out/bin && echo 'Install phase'";
        }};
      }});
    }};
}}
"#,
        description, inputs, inputs_str, name, version
    )
}

