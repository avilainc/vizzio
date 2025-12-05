//! Bash completion script generation

use crate::arg::Arg;
use crate::app::Command;

pub fn generate_bash(app_name: &str, args: &[Arg], commands: &[Command]) -> String {
    let mut script = format!("_{}_completion() {{\n", app_name);
    script.push_str("    local cur prev opts\n");
    script.push_str("    COMPREPLY=()\n");
    script.push_str("    cur=\"${COMP_WORDS[COMP_CWORD]}\"\n");
    script.push_str("    prev=\"${COMP_WORDS[COMP_CWORD-1]}\"\n\n");

    // Add options
    script.push_str("    opts=\"");
    for arg in args {
        script.push_str(&format!("--{} ", arg.long));
        if let Some(short) = &arg.short {
            script.push_str(&format!("-{} ", short));
        }
    }
    script.push_str("\"\n\n");

    // Add subcommands
    if !commands.is_empty() {
        script.push_str("    local commands=\"");
        for cmd in commands {
            script.push_str(&format!("{} ", cmd.name));
        }
        script.push_str("\"\n\n");
    }

    script.push_str("    COMPREPLY=( $(compgen -W \"${opts} ${commands}\" -- ${cur}) )\n");
    script.push_str("    return 0\n");
    script.push_str("}\n\n");
    script.push_str(&format!("complete -F _{}_completion {}\n", app_name, app_name));

    script
}
