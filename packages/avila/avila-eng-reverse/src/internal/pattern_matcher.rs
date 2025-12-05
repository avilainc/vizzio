// Pattern Matcher nativo 100% Rust - substitui aho-corasick e yara
// Implementação eficiente usando Boyer-Moore-Horspool

use std::collections::HashMap;

pub struct PatternMatcher {
    patterns: Vec<Pattern>,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub id: String,
    pub bytes: Vec<u8>,
    pub mask: Option<Vec<u8>>, // Para wildcards: 0 = ignore, 1 = must match
}

#[derive(Debug, Clone)]
pub struct Match {
    pub pattern_id: String,
    pub offset: usize,
}

impl PatternMatcher {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, id: String, bytes: Vec<u8>) {
        self.patterns.push(Pattern {
            id,
            bytes,
            mask: None,
        });
    }

    pub fn add_pattern_with_mask(&mut self, id: String, bytes: Vec<u8>, mask: Vec<u8>) {
        self.patterns.push(Pattern {
            id,
            bytes,
            mask: Some(mask),
        });
    }

    pub fn find_all(&self, data: &[u8]) -> Vec<Match> {
        let mut matches = Vec::new();

        for pattern in &self.patterns {
            let pattern_matches = self.find_pattern(data, pattern);
            for offset in pattern_matches {
                matches.push(Match {
                    pattern_id: pattern.id.clone(),
                    offset,
                });
            }
        }

        matches
    }

    fn find_pattern(&self, data: &[u8], pattern: &Pattern) -> Vec<usize> {
        let mut matches = Vec::new();

        if pattern.bytes.is_empty() || data.len() < pattern.bytes.len() {
            return matches;
        }

        // Boyer-Moore-Horspool algorithm
        let pattern_len = pattern.bytes.len();
        let bad_char_table = self.build_bad_char_table(&pattern.bytes);

        let mut pos = 0;
        while pos <= data.len() - pattern_len {
            let mut matched = true;

            // Verificar pattern com mask se existir
            for i in 0..pattern_len {
                if let Some(ref mask) = pattern.mask {
                    if mask[i] == 1 && data[pos + i] != pattern.bytes[i] {
                        matched = false;
                        break;
                    }
                } else if data[pos + i] != pattern.bytes[i] {
                    matched = false;
                    break;
                }
            }

            if matched {
                matches.push(pos);
                pos += 1; // Continuar procurando overlapping matches
            } else {
                // Usar bad character rule
                let last_char = data[pos + pattern_len - 1];
                let shift = *bad_char_table.get(&last_char).unwrap_or(&pattern_len);
                pos += shift;
            }
        }

        matches
    }

    fn build_bad_char_table(&self, pattern: &[u8]) -> HashMap<u8, usize> {
        let mut table = HashMap::new();
        let pattern_len = pattern.len();

        for i in 0..pattern_len - 1 {
            table.insert(pattern[i], pattern_len - 1 - i);
        }

        table
    }

    // Multi-pattern search sequencial
    pub fn find_all_sequential(&self, data: &[u8]) -> Vec<Match> {
        let mut all_matches = Vec::new();

        for pattern in &self.patterns {
            let pattern_matches = self.find_pattern(data, pattern);
            for offset in pattern_matches {
                all_matches.push(Match {
                    pattern_id: pattern.id.clone(),
                    offset,
                });
            }
        }

        all_matches
    }

    // Helper para criar pattern de hex string
    pub fn pattern_from_hex(id: String, hex_string: &str) -> Result<Pattern, String> {
        let hex_clean = hex_string.replace(" ", "").replace("?", "00");

        // Decodificar hex manualmente
        let bytes = Self::decode_hex(&hex_clean)
            .ok_or_else(|| "Erro ao decodificar hex".to_string())?;

        // Detectar wildcards
        let has_wildcards = hex_string.contains('?');
        let mask = if has_wildcards {
            let mut mask_vec = Vec::new();
            for chunk in hex_string.split_whitespace() {
                if chunk == "??" || chunk == "?" {
                    mask_vec.push(0);
                } else {
                    mask_vec.push(1);
                }
            }
            Some(mask_vec)
        } else {
            None
        };

        Ok(Pattern { id, bytes, mask })
    }

    /// Decodificar string hexadecimal manualmente
    fn decode_hex(s: &str) -> Option<Vec<u8>> {
        if s.len() % 2 != 0 {
            return None;
        }

        let mut bytes = Vec::new();
        for i in (0..s.len()).step_by(2) {
            let byte_str = &s[i..i+2];
            let byte = u8::from_str_radix(byte_str, 16).ok()?;
            bytes.push(byte);
        }

        Some(bytes)
    }
}

impl Default for PatternMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pattern() {
        let mut matcher = PatternMatcher::new();
        matcher.add_pattern("test".to_string(), vec![0x48, 0x65, 0x6C, 0x6C, 0x6F]); // "Hello"

        let data = b"Hello World Hello";
        let matches = matcher.find_all(data);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].offset, 0);
        assert_eq!(matches[1].offset, 12);
    }

    #[test]
    fn test_pattern_with_mask() {
        let mut matcher = PatternMatcher::new();
        // Pattern: 0x48 ?? 0x6C 0x6C 0x6F (? = wildcard)
        matcher.add_pattern_with_mask(
            "test".to_string(),
            vec![0x48, 0x00, 0x6C, 0x6C, 0x6F],
            vec![1, 0, 1, 1, 1],
        );

        let data = b"HxlloWorld";
        let matches = matcher.find_all(data);

        assert_eq!(matches.len(), 1);
    }
}
