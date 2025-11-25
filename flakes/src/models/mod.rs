pub mod flake_input;
pub mod flake_output;
pub mod eval_result;
pub mod build_result;
pub mod scaffold_result;

pub use flake_input::FlakeInput;
pub use flake_output::FlakeOutput;
pub use eval_result::EvalResult;
pub use build_result::BuildResult;
pub use scaffold_result::{ScaffoldResult, ScaffoldType, TemplateType};

