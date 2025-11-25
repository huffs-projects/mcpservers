pub fn multi_template(
    name: &str,
    description: &str,
    version: &str,
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
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${{system}};
      in
      {{
        packages = {{
          default = pkgs.stdenv.mkDerivation {{
            pname = "{}";
            version = "{}";
            src = ./.;
            buildPhase = "echo 'Build phase'";
            installPhase = "mkdir -p $out/bin && echo 'Install phase'";
          }};
        }};

        apps = {{
          default = flake-utils.lib.mkApp {{
            drv = self.packages.${{system}}.default;
          }};
        }};

        devShells.default = pkgs.mkShell {{
          name = "{}";
          buildInputs = with pkgs; [
            # Add your development dependencies here
          ];
          shellHook = '''';
            echo "Welcome to {} development shell"
          '';
        }};

        lib = {{
          # Add library functions here
        }};
      }});
}}
"#,
        description, inputs, inputs_str, name, version, name, name
    )
}

