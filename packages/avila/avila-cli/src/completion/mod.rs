//! Shell completion generation
//!
//! Provides functionality to generate shell completion scripts
//! for various shells including Bash, Zsh, Fish, and PowerShell.

mod bash;
mod zsh;
mod fish;
mod powershell;

pub use bash::generate_bash;
pub use zsh::generate_zsh;
pub use fish::generate_fish;
pub use powershell::generate_powershell;

/// Shell completion type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
}

/// Generate completion script for the specified shell
pub fn generate(shell: Shell, app_name: &str, args: &[crate::arg::Arg], commands: &[crate::app::Command]) -> String {
    match shell {
        Shell::Bash => bash::generate_bash(app_name, args, commands),
        Shell::Zsh => zsh::generate_zsh(app_name, args, commands),
        Shell::Fish => fish::generate_fish(app_name, args, commands),
        Shell::PowerShell => powershell::generate_powershell(app_name, args, commands),
    }
}
