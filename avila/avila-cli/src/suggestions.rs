//! Error suggestions using Levenshtein distance
//!
//! Provides "Did you mean?" functionality for typos in arguments, commands, and values.

/// Calculate Levenshtein distance between two strings
///
/// Returns the minimum number of single-character edits (insertions, deletions, or substitutions)
/// required to change one string into the other.
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    let a_len = a_chars.len();
    let b_len = b_chars.len();

    if a_len == 0 {
        return b_len;
    }
    if b_len == 0 {
        return a_len;
    }

    // Create distance matrix
    let mut matrix = vec![vec![0; b_len + 1]; a_len + 1];

    // Initialize first column and row
    for i in 0..=a_len {
        matrix[i][0] = i;
    }
    for j in 0..=b_len {
        matrix[0][j] = j;
    }

    // Fill matrix
    for i in 1..=a_len {
        for j in 1..=b_len {
            let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1,      // deletion
                    matrix[i][j - 1] + 1       // insertion
                ),
                matrix[i - 1][j - 1] + cost    // substitution
            );
        }
    }

    matrix[a_len][b_len]
}

/// Find similar strings from a list of candidates
///
/// Returns strings that have a Levenshtein distance <= max_distance,
/// sorted by similarity (closest first).
///
/// # Arguments
/// * `input` - The input string (potentially misspelled)
/// * `candidates` - List of valid strings to compare against
/// * `max_distance` - Maximum edit distance to consider (typically 1-3)
///
/// # Example
/// ```
/// use avila_cli::suggestions::find_similar;
///
/// let input = "naem";
/// let candidates = vec!["name", "age", "email", "port"];
/// let suggestions = find_similar(input, &candidates, 2);
/// assert_eq!(suggestions, vec!["name"]);
/// ```
pub fn find_similar(input: &str, candidates: &[&str], max_distance: usize) -> Vec<String> {
    let mut matches: Vec<(String, usize)> = candidates
        .iter()
        .map(|&candidate| {
            let distance = levenshtein_distance(input, candidate);
            (candidate.to_string(), distance)
        })
        .filter(|(_, distance)| *distance <= max_distance)
        .collect();

    // Sort by distance (closest first)
    matches.sort_by_key(|(_, distance)| *distance);

    matches.into_iter().map(|(candidate, _)| candidate).collect()
}

/// Find similar argument names (with or without -- prefix)
///
/// Handles both long arguments (--name) and short arguments (-n)
pub fn find_similar_args(input: &str, arg_names: &[&str], max_distance: usize) -> Vec<String> {
    // Strip leading dashes from input
    let clean_input = input.trim_start_matches('-');

    // Check if input is short form (single char)
    let is_short = clean_input.len() == 1 && input.starts_with('-') && !input.starts_with("--");

    if is_short {
        // For short args, do exact match only
        arg_names
            .iter()
            .filter(|name| name.len() == 1 && name.chars().next() == clean_input.chars().next())
            .map(|s| format!("-{}", s))
            .collect()
    } else {
        // For long args, use fuzzy matching
        let suggestions = find_similar(clean_input, arg_names, max_distance);
        suggestions.into_iter().map(|s| format!("--{}", s)).collect()
    }
}

/// Find similar command names
pub fn find_similar_commands(input: &str, commands: &[&str], max_distance: usize) -> Vec<String> {
    find_similar(input, commands, max_distance)
}

/// Find similar values from possible_values list
pub fn find_similar_values(input: &str, values: &[&str], max_distance: usize) -> Vec<String> {
    find_similar(input, values, max_distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("cat", "cat"), 0);
        assert_eq!(levenshtein_distance("cat", "cats"), 1);
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
    }

    #[test]
    fn test_find_similar() {
        let candidates = vec!["name", "age", "email", "port", "verbose"];

        // Exact typo: naem -> name
        let suggestions = find_similar("naem", &candidates, 2);
        assert_eq!(suggestions, vec!["name"]);

        // Close typo: verbos -> verbose
        let suggestions = find_similar("verbos", &candidates, 2);
        assert_eq!(suggestions, vec!["verbose"]);

        // Multiple candidates within distance
        let suggestions = find_similar("ag", &candidates, 2);
        assert!(suggestions.contains(&"age".to_string()));
    }

    #[test]
    fn test_find_similar_args() {
        let args = vec!["name", "age", "email", "verbose"];

        // Long argument with typo
        let suggestions = find_similar_args("--naem", &args, 2);
        assert_eq!(suggestions, vec!["--name"]);

        // Without dashes
        let suggestions = find_similar_args("naem", &args, 2);
        assert_eq!(suggestions, vec!["--name"]);
    }

    #[test]
    fn test_find_similar_commands() {
        let commands = vec!["install", "remove", "list", "update"];

        // Common typo
        let suggestions = find_similar_commands("isntall", &commands, 2);
        assert_eq!(suggestions, vec!["install"]);

        let suggestions = find_similar_commands("lst", &commands, 2);
        assert_eq!(suggestions, vec!["list"]);
    }

    #[test]
    fn test_find_similar_values() {
        let values = vec!["json", "yaml", "toml", "xml"];

        // Typo
        let suggestions = find_similar_values("josn", &values, 2);
        assert_eq!(suggestions, vec!["json"]);

        let suggestions = find_similar_values("yml", &values, 2);
        // yml tem distância 1 de xml, 2 de yaml e toml (mas toml está mais perto de "toml" = 2)
        assert!(suggestions.len() >= 2);
        assert!(suggestions.contains(&"yaml".to_string()) || suggestions.contains(&"xml".to_string()));
    }

    #[test]
    fn test_no_similar_found() {
        let candidates = vec!["apple", "banana", "cherry"];
        let suggestions = find_similar("xyz", &candidates, 2);
        assert!(suggestions.is_empty());
    }

    #[test]
    fn test_distance_threshold() {
        let candidates = vec!["test"];

        // Within threshold
        let suggestions = find_similar("tset", &candidates, 2);
        assert_eq!(suggestions, vec!["test"]);

        // Beyond threshold
        let suggestions = find_similar("abcd", &candidates, 2);
        assert!(suggestions.is_empty());
    }
}
