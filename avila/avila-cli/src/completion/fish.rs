//! Fish shell completion script generation

use crate::arg::Arg;
use crate::app::Command;

pub fn generate_fish(app_name: &str, args: &[Arg], commands: &[Command]) -> String {
    let mut script = String::new();

    for arg in args {
        script.push_str(&format!("complete -c {} -l {} -d '{}'\n",
            app_name, arg.long, arg.help.replace('\'', "\\'")));

        if let Some(short) = &arg.short {
            script.push_str(&format!("complete -c {} -s {} -d '{}'\n",
                app_name, short, arg.help.replace('\'', "\\'")));
        }
    }

    for cmd in commands {
        script.push_str(&format!("complete -c {} -f -a '{}' -d '{}'\n",
            app_name, cmd.name, cmd.about.replace('\'', "\\'")));
    }

    script
}
