//! Bit manipulation utilities

/// Bit manipulation operations
pub struct BitOps;

impl BitOps {
    /// Counts the number of set bits (population count)
    #[inline]
    pub const fn popcount(mut value: usize) -> u32 {
        let mut count = 0;
        while value != 0 {
            count += 1;
            value &= value - 1; // Clear lowest set bit
        }
        count
    }

    /// Returns the index of the lowest set bit (0-indexed)
    #[inline]
    pub const fn trailing_zeros(value: usize) -> u32 {
        value.trailing_zeros()
    }

    /// Returns the index of the highest set bit (0-indexed)
    #[inline]
    pub const fn leading_zeros(value: usize) -> u32 {
        value.leading_zeros()
    }

    /// Sets a bit at position
    #[inline]
    pub const fn set_bit(value: usize, pos: u32) -> usize {
        value | (1 << pos)
    }

    /// Clears a bit at position
    #[inline]
    pub const fn clear_bit(value: usize, pos: u32) -> usize {
        value & !(1 << pos)
    }

    /// Toggles a bit at position
    #[inline]
    pub const fn toggle_bit(value: usize, pos: u32) -> usize {
        value ^ (1 << pos)
    }

    /// Tests if a bit is set at position
    #[inline]
    pub const fn test_bit(value: usize, pos: u32) -> bool {
        (value & (1 << pos)) != 0
    }

    /// Extracts bits from start to end (inclusive)
    #[inline]
    pub const fn extract_bits(value: usize, start: u32, end: u32) -> usize {
        let mask = ((1 << (end - start + 1)) - 1) << start;
        (value & mask) >> start
    }
}

/// Bitmap for tracking allocated slots
#[derive(Clone, Copy)]
pub struct Bitmap<const N: usize> {
    bits: [usize; N],
}

impl<const N: usize> Bitmap<N> {
    const BITS_PER_WORD: usize = core::mem::size_of::<usize>() * 8;

    /// Creates a new empty bitmap
    pub const fn new() -> Self {
        Self { bits: [0; N] }
    }

    /// Creates a bitmap with all bits set
    pub const fn new_full() -> Self {
        Self { bits: [!0; N] }
    }

    /// Sets a bit
    pub fn set(&mut self, index: usize) {
        let word = index / Self::BITS_PER_WORD;
        let bit = index % Self::BITS_PER_WORD;
        if word < N {
            self.bits[word] = BitOps::set_bit(self.bits[word], bit as u32);
        }
    }

    /// Clears a bit
    pub fn clear(&mut self, index: usize) {
        let word = index / Self::BITS_PER_WORD;
        let bit = index % Self::BITS_PER_WORD;
        if word < N {
            self.bits[word] = BitOps::clear_bit(self.bits[word], bit as u32);
        }
    }

    /// Tests if a bit is set
    pub fn test(&self, index: usize) -> bool {
        let word = index / Self::BITS_PER_WORD;
        let bit = index % Self::BITS_PER_WORD;
        if word < N {
            BitOps::test_bit(self.bits[word], bit as u32)
        } else {
            false
        }
    }

    /// Finds the first unset bit
    pub fn find_first_unset(&self) -> Option<usize> {
        for (word_idx, &word) in self.bits.iter().enumerate() {
            if word != !0 {
                let bit_idx = BitOps::trailing_zeros(!word);
                return Some(word_idx * Self::BITS_PER_WORD + bit_idx as usize);
            }
        }
        None
    }

    /// Counts the number of set bits
    pub fn count_set(&self) -> usize {
        self.bits.iter().map(|&w| BitOps::popcount(w) as usize).sum()
    }

    /// Clears all bits
    pub fn clear_all(&mut self) {
        self.bits.fill(0);
    }

    /// Sets all bits
    pub fn set_all(&mut self) {
        self.bits.fill(!0);
    }
}

impl<const N: usize> Default for Bitmap<N> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bit_ops() {
        assert_eq!(BitOps::popcount(0b1010), 2);
        assert_eq!(BitOps::popcount(0b1111), 4);

        let value = 0b0000;
        let value = BitOps::set_bit(value, 2);
        assert_eq!(value, 0b0100);

        let value = BitOps::clear_bit(value, 2);
        assert_eq!(value, 0b0000);

        assert!(BitOps::test_bit(0b1000, 3));
        assert!(!BitOps::test_bit(0b1000, 2));
    }

    #[test]
    fn test_bitmap() {
        let mut bitmap = Bitmap::<1>::new();

        assert!(!bitmap.test(0));
        bitmap.set(5);
        assert!(bitmap.test(5));

        bitmap.clear(5);
        assert!(!bitmap.test(5));

        bitmap.set(3);
        bitmap.set(7);
        assert_eq!(bitmap.count_set(), 2);
    }

    #[test]
    fn test_bitmap_find() {
        let mut bitmap = Bitmap::<1>::new();
        bitmap.set_all();
        bitmap.clear(10);

        assert_eq!(bitmap.find_first_unset(), Some(10));
    }
}
