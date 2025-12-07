//! Zsh completion script generation

use crate::arg::Arg;
use crate::app::Command;

pub fn generate_zsh(app_name: &str, args: &[Arg], _commands: &[Command]) -> String {
    let mut script = format!("#compdef {}\n\n", app_name);
    script.push_str(&format!("_{}_completion() {{\n", app_name));
    script.push_str("    local -a opts\n");
    script.push_str("    opts=(\n");

    for arg in args {
        let help = arg.help.replace('\"', "'");
        if let Some(short) = &arg.short {
            script.push_str(&format!("        '(-{})--{}[{}]'\n", short, arg.long, help));
        } else {
            script.push_str(&format!("        '--{}[{}]'\n", arg.long, help));
        }
    }

    script.push_str("    )\n");
    script.push_str("    _arguments $opts\n");
    script.push_str("}\n\n");
    script.push_str(&format!("_{}_completion\n", app_name));

    script
}
