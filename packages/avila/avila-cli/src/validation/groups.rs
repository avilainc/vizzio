//! Argument group validation logic

use crate::arg::ArgGroup;
use crate::matches::Matches;

/// Validate argument groups
///
/// Checks:
/// - Required groups have at least one argument present
/// - Mutually exclusive groups have only one argument present
pub fn validate_groups(groups: &[ArgGroup], matches: &Matches, colored: bool) -> Result<(), String> {
    for group in groups {
        let present_args: Vec<String> = group.args.iter()
            .filter(|arg_name| matches.is_present(arg_name))
            .map(|s| s.clone())
            .collect();

        // Check if required group has at least one arg
        if group.required && present_args.is_empty() {
            let msg = if colored {
                format!(
                    "Error: at least one of {} is required",
                    crate::colors::colorize(&format!("[{}]", group.args.join(", ")), crate::colors::YELLOW)
                )
            } else {
                format!("Error: at least one of [{}] is required", group.args.join(", "))
            };
            return Err(msg);
        }

        // Check mutual exclusion (only one arg allowed)
        if !group.multiple && present_args.len() > 1 {
            let msg = if colored {
                format!(
                    "Error: arguments {} are mutually exclusive",
                    crate::colors::colorize(&present_args.join(", "), crate::colors::RED)
                )
            } else {
                format!("Error: arguments {} are mutually exclusive", present_args.join(", "))
            };
            return Err(msg);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_group_validation() {
        let group = ArgGroup::new("test")
            .args(&["arg1", "arg2"])
            .required(true);

        let mut matches = Matches::new();

        // Should fail when no args present
        assert!(validate_groups(&[group.clone()], &matches, false).is_err());

        // Should pass when one arg present
        matches.args.insert("arg1".to_string(), None);
        assert!(validate_groups(&[group], &matches, false).is_ok());
    }

    #[test]
    fn test_mutual_exclusion() {
        let group = ArgGroup::new("test")
            .args(&["arg1", "arg2"])
            .multiple(false);

        let mut matches = Matches::new();
        matches.args.insert("arg1".to_string(), None);
        matches.args.insert("arg2".to_string(), None);

        // Should fail when multiple args present
        assert!(validate_groups(&[group], &matches, false).is_err());
    }
}
