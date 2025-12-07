//! Pattern matching algorithms for buffer searching
//! Pure Rust implementations without external dependencies

use alloc::vec::Vec;

/// Naive pattern search (for reference)
pub struct NaiveSearch;

impl NaiveSearch {
    /// Finds all occurrences of pattern in text
    pub fn search(text: &[u8], pattern: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();

        if pattern.is_empty() || text.len() < pattern.len() {
            return matches;
        }

        for i in 0..=(text.len() - pattern.len()) {
            if text[i..i + pattern.len()] == *pattern {
                matches.push(i);
            }
        }

        matches
    }

    /// Finds first occurrence
    pub fn find(text: &[u8], pattern: &[u8]) -> Option<usize> {
        if pattern.is_empty() || text.len() < pattern.len() {
            return None;
        }

        for i in 0..=(text.len() - pattern.len()) {
            if text[i..i + pattern.len()] == *pattern {
                return Some(i);
            }
        }

        None
    }
}

/// Boyer-Moore pattern search algorithm
/// Efficient for larger patterns
pub struct BoyerMoore {
    bad_char: [isize; 256],
    pattern: Vec<u8>,
}

impl BoyerMoore {
    /// Creates Boyer-Moore searcher for pattern
    pub fn new(pattern: &[u8]) -> Self {
        let mut bad_char = [-1isize; 256];

        // Build bad character table
        for (i, &byte) in pattern.iter().enumerate() {
            bad_char[byte as usize] = i as isize;
        }

        Self {
            bad_char,
            pattern: pattern.to_vec(),
        }
    }

    /// Searches for pattern in text
    pub fn search(&self, text: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();
        let m = self.pattern.len();
        let n = text.len();

        if m == 0 || n < m {
            return matches;
        }

        let mut s = 0; // Shift of the pattern

        while s <= n - m {
            let mut j = (m - 1) as isize;

            // Match from right to left
            while j >= 0 && self.pattern[j as usize] == text[s + j as usize] {
                j -= 1;
            }

            if j < 0 {
                // Pattern found
                matches.push(s);

                // Shift past this match
                s += if s + m < n {
                    m - self.bad_char[text[s + m] as usize] as usize - 1
                } else {
                    1
                };
            } else {
                // Shift based on bad character
                let bad_char_shift = j - self.bad_char[text[s + j as usize] as usize];
                s += 1.max(bad_char_shift as usize);
            }
        }

        matches
    }

    /// Finds first occurrence
    pub fn find(&self, text: &[u8]) -> Option<usize> {
        self.search(text).first().copied()
    }
}

/// Knuth-Morris-Pratt (KMP) algorithm
/// Efficient for patterns with repeated substrings
pub struct Kmp {
    lps: Vec<usize>, // Longest Proper Prefix which is also Suffix
    pattern: Vec<u8>,
}

impl Kmp {
    /// Creates KMP searcher for pattern
    pub fn new(pattern: &[u8]) -> Self {
        let lps = Self::compute_lps(pattern);

        Self {
            lps,
            pattern: pattern.to_vec(),
        }
    }

    /// Computes LPS array
    fn compute_lps(pattern: &[u8]) -> Vec<usize> {
        let m = pattern.len();
        let mut lps = vec![0; m];
        let mut len = 0;
        let mut i = 1;

        while i < m {
            if pattern[i] == pattern[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else if len != 0 {
                len = lps[len - 1];
            } else {
                lps[i] = 0;
                i += 1;
            }
        }

        lps
    }

    /// Searches for pattern in text
    pub fn search(&self, text: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();
        let m = self.pattern.len();
        let n = text.len();

        if m == 0 || n < m {
            return matches;
        }

        let mut i = 0; // Index for text
        let mut j = 0; // Index for pattern

        while i < n {
            if self.pattern[j] == text[i] {
                i += 1;
                j += 1;
            }

            if j == m {
                matches.push(i - j);
                j = self.lps[j - 1];
            } else if i < n && self.pattern[j] != text[i] {
                if j != 0 {
                    j = self.lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }

        matches
    }

    /// Finds first occurrence
    pub fn find(&self, text: &[u8]) -> Option<usize> {
        self.search(text).first().copied()
    }
}

/// Rabin-Karp algorithm using rolling hash
/// Good for multiple pattern searching
pub struct RabinKarp {
    pattern_hash: u64,
    pattern_len: usize,
    prime: u64,
    base: u64,
}

impl RabinKarp {
    const PRIME: u64 = 101;
    const BASE: u64 = 256;

    /// Creates Rabin-Karp searcher
    pub fn new(pattern: &[u8]) -> Self {
        let pattern_hash = Self::hash(pattern, pattern.len());

        Self {
            pattern_hash,
            pattern_len: pattern.len(),
            prime: Self::PRIME,
            base: Self::BASE,
        }
    }

    /// Computes hash of data
    fn hash(data: &[u8], len: usize) -> u64 {
        let mut h = 0u64;
        for i in 0..len.min(data.len()) {
            h = h.wrapping_mul(Self::BASE).wrapping_add(data[i] as u64);
        }
        h % Self::PRIME
    }

    /// Searches for pattern in text
    pub fn search(&self, text: &[u8], pattern: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();
        let n = text.len();
        let m = self.pattern_len;

        if m == 0 || n < m {
            return matches;
        }

        // Calculate h = base^(m-1) % prime
        let mut h = 1u64;
        for _ in 0..m - 1 {
            h = (h * self.base) % self.prime;
        }

        // Calculate hash of first window
        let mut text_hash = Self::hash(text, m);

        for i in 0..=(n - m) {
            // Check if hashes match
            if text_hash == self.pattern_hash {
                // Verify actual match (hash collision check)
                if text[i..i + m] == *pattern {
                    matches.push(i);
                }
            }

            // Calculate hash for next window
            if i < n - m {
                text_hash = (self.base * (text_hash + self.prime - (text[i] as u64 * h) % self.prime)
                           + text[i + m] as u64) % self.prime;
            }
        }

        matches
    }

    /// Finds first occurrence
    pub fn find(&self, text: &[u8], pattern: &[u8]) -> Option<usize> {
        self.search(text, pattern).first().copied()
    }
}

/// Two-way string matching algorithm
/// Combines forward and backward search
pub struct TwoWay;

impl TwoWay {
    /// Finds all occurrences using two-way algorithm
    pub fn search(text: &[u8], pattern: &[u8]) -> Vec<usize> {
        let mut matches = Vec::new();
        let m = pattern.len();
        let n = text.len();

        if m == 0 || n < m {
            return matches;
        }

        let critical_pos = Self::maximal_suffix(pattern);
        let mut period = critical_pos + 1;

        // Check if pattern is periodic
        let mut i = 0;
        while i < critical_pos && pattern[i] == pattern[i + period] {
            i += 1;
        }

        if i < critical_pos {
            period = critical_pos;
        }

        let mut pos = 0;
        let mut memory = 0;

        while pos <= n - m {
            let mut i = if memory > critical_pos { memory } else { critical_pos };

            // Forward search
            while i < m && pattern[i] == text[pos + i] {
                i += 1;
            }

            if i < m {
                pos += i - critical_pos;
                memory = 0;
                continue;
            }

            // Backward search
            let mut j = critical_pos;
            while j > memory && pattern[j - 1] == text[pos + j - 1] {
                j -= 1;
            }

            if j <= memory {
                matches.push(pos);
                pos += period;
                memory = m - period;
            } else {
                pos += j - memory;
                memory = 0;
            }
        }

        matches
    }

    /// Computes maximal suffix
    fn maximal_suffix(pattern: &[u8]) -> usize {
        let mut ms = 0;
        let mut p = 1;
        let mut k = 0;

        while p + k < pattern.len() {
            if pattern[ms + k] < pattern[p + k] {
                p += k + 1;
                k = 0;
            } else if pattern[ms + k] == pattern[p + k] {
                k += 1;
            } else {
                ms = p;
                p = ms + 1;
                k = 0;
            }
        }

        ms
    }

    /// Finds first occurrence
    pub fn find(text: &[u8], pattern: &[u8]) -> Option<usize> {
        Self::search(text, pattern).first().copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive_search() {
        let text = b"ABABCABABA";
        let pattern = b"ABA";

        let matches = NaiveSearch::search(text, pattern);
        assert_eq!(matches, vec![0, 5, 7]);

        let first = NaiveSearch::find(text, pattern);
        assert_eq!(first, Some(0));
    }

    #[test]
    fn test_boyer_moore() {
        let pattern = b"EXAMPLE";
        let bm = BoyerMoore::new(pattern);

        let text = b"HERE IS A SIMPLE EXAMPLE TEXT WITH EXAMPLE WORD";
        let matches = bm.search(text);

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0], 17);
        assert_eq!(matches[1], 38);
    }

    #[test]
    fn test_kmp() {
        let pattern = b"ABABC";
        let kmp = Kmp::new(pattern);

        let text = b"ABABDABABCABABA";
        let matches = kmp.search(text);

        assert_eq!(matches, vec![5]);
    }

    #[test]
    fn test_rabin_karp() {
        let pattern = b"test";
        let rk = RabinKarp::new(pattern);

        let text = b"this is a test string with test word";
        let matches = rk.search(text, pattern);

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_two_way() {
        let text = b"GCATCGCAGAGAGTATACAGTACG";
        let pattern = b"GCAGAGAG";

        let matches = TwoWay::search(text, pattern);
        assert_eq!(matches, vec![5]);
    }

    #[test]
    fn test_empty_pattern() {
        let text = b"test";
        let pattern = b"";

        assert!(NaiveSearch::search(text, pattern).is_empty());
        assert!(TwoWay::search(text, pattern).is_empty());
    }

    #[test]
    fn test_pattern_longer_than_text() {
        let text = b"short";
        let pattern = b"very long pattern";

        assert!(NaiveSearch::search(text, pattern).is_empty());
    }

    #[test]
    fn test_no_match() {
        let text = b"ABCDEFG";
        let pattern = b"XYZ";

        assert!(NaiveSearch::find(text, pattern).is_none());
        assert!(TwoWay::find(text, pattern).is_none());
    }
}
