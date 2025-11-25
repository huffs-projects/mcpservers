pub mod kitty_options;
pub mod kitty_theming;
pub mod kitty_keybindings;
pub mod kitty_templates;
pub mod kitty_validate;
pub mod kitty_apply;

pub use kitty_options::handle_kitty_options;
pub use kitty_theming::handle_kitty_theming;
pub use kitty_keybindings::handle_kitty_keybindings;
pub use kitty_templates::handle_kitty_templates;
pub use kitty_validate::handle_kitty_validate;
pub use kitty_apply::handle_kitty_apply;

