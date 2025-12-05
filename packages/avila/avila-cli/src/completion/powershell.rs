//! PowerShell completion script generation

use crate::arg::Arg;
use crate::app::Command;

pub fn generate_powershell(app_name: &str, args: &[Arg], commands: &[Command]) -> String {
    let mut script = format!("Register-ArgumentCompleter -CommandName {} -ScriptBlock {{\n", app_name);
    script.push_str("    param($commandName, $wordToComplete, $commandAst, $fakeBoundParameter)\n\n");
    script.push_str("    $completions = @(\n");

    for arg in args {
        script.push_str(&format!("        @{{ CompletionText = '--{}'; ListItemText = '--{}'; ToolTip = '{}' }},\n",
            arg.long, arg.long, arg.help.replace('\"', "'")));
    }

    for cmd in commands {
        script.push_str(&format!("        @{{ CompletionText = '{}'; ListItemText = '{}'; ToolTip = '{}' }},\n",
            cmd.name, cmd.name, cmd.about.replace('\"', "'")));
    }

    script.push_str("    )\n\n");
    script.push_str("    $completions | Where-Object { $_.CompletionText -like \"$wordToComplete*\" } | \n");
    script.push_str("        ForEach-Object { [System.Management.Automation.CompletionResult]::new($_.CompletionText, $_.ListItemText, 'ParameterValue', $_.ToolTip) }\n");
    script.push_str("}\n");

    script
}
