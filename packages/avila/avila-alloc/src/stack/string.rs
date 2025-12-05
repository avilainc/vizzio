//! StackString - UTF-8 validated string on stack

use core::{fmt, str};

/// Stack-allocated UTF-8 string with fixed capacity
#[derive(Debug)]
pub struct StackString<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> StackString<N> {
    /// Creates a new empty `StackString`
    pub const fn new() -> Self {
        Self {
            data: [0; N],
            len: 0,
        }
    }

    /// Returns the length in bytes
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the string is empty
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the total capacity in bytes
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Pushes a string slice onto the end
    pub fn push_str(&mut self, s: &str) -> Result<(), ()> {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > N {
            return Err(());
        }
        self.data[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }

    /// Pushes a character onto the end
    pub fn push(&mut self, c: char) -> Result<(), ()> {
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        self.push_str(s)
    }

    /// Returns the string as a `&str`
    pub fn as_str(&self) -> &str {
        unsafe { str::from_utf8_unchecked(&self.data[..self.len]) }
    }

    /// Clears the string
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Removes the last character and returns it
    pub fn pop(&mut self) -> Option<char> {
        if self.len == 0 {
            return None;
        }
        let s = self.as_str();
        let ch = s.chars().last()?;
        self.len -= ch.len_utf8();
        Some(ch)
    }

    /// Truncates the string to `new_len` bytes
    pub fn truncate(&mut self, new_len: usize) {
        if new_len < self.len {
            // Ensure we're on a char boundary
            if core::str::from_utf8(&self.data[..new_len]).is_ok() {
                self.len = new_len;
            }
        }
    }

    /// Returns the remaining capacity in bytes
    pub const fn remaining_capacity(&self) -> usize {
        N - self.len
    }

    /// Returns the string as bytes
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.len]
    }

    /// Checks if the string starts with the given prefix
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.as_str().starts_with(prefix)
    }

    /// Checks if the string ends with the given suffix
    pub fn ends_with(&self, suffix: &str) -> bool {
        self.as_str().ends_with(suffix)
    }

    /// Returns true if the string contains the given pattern
    pub fn contains(&self, pattern: &str) -> bool {
        self.as_str().contains(pattern)
    }

    /// Splits the string at the given index, returning both parts
    pub fn split_at(&self, mid: usize) -> Option<(&str, &str)> {
        if mid <= self.len && self.as_str().is_char_boundary(mid) {
            Some(self.as_str().split_at(mid))
        } else {
            None
        }
    }

    /// Creates a StackString from a string slice
    pub fn from_str(s: &str) -> Result<Self, ()> {
        if s.len() > N {
            return Err(());
        }
        let mut string = Self::new();
        string.push_str(s)?;
        Ok(string)
    }

    /// Returns an iterator over the chars
    pub fn chars(&self) -> core::str::Chars<'_> {
        self.as_str().chars()
    }

    /// Returns an iterator over the lines
    pub fn lines(&self) -> core::str::Lines<'_> {
        self.as_str().lines()
    }

    /// Converts to lowercase
    pub fn to_lowercase(&self) -> Self
    where
        [(); N]: ,
    {
        let mut result = Self::new();
        for ch in self.as_str().chars() {
            for lower in ch.to_lowercase() {
                let _ = result.push(lower);
            }
        }
        result
    }

    /// Converts to uppercase
    pub fn to_uppercase(&self) -> Self
    where
        [(); N]: ,
    {
        let mut result = Self::new();
        for ch in self.as_str().chars() {
            for upper in ch.to_uppercase() {
                let _ = result.push(upper);
            }
        }
        result
    }
}

impl<const N: usize> Default for StackString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> fmt::Display for StackString<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl<const N: usize> AsRef<str> for StackString<N> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> AsRef<[u8]> for StackString<N> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<const N: usize> PartialEq for StackString<N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl<const N: usize> PartialEq<str> for StackString<N> {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl<const N: usize> PartialEq<&str> for StackString<N> {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl<const N: usize> Eq for StackString<N> {}

impl<const N: usize> Clone for StackString<N> {
    fn clone(&self) -> Self {
        let mut new_string = Self::new();
        new_string.data[..self.len].copy_from_slice(&self.data[..self.len]);
        new_string.len = self.len;
        new_string
    }
}

impl<const N: usize> core::ops::Deref for StackString<N> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<const N: usize> core::hash::Hash for StackString<N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl<const N: usize> core::cmp::PartialOrd for StackString<N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl<const N: usize> core::cmp::Ord for StackString<N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}
