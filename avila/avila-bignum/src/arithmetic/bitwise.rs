//! Bitwise operations (AND, OR, XOR, NOT, shifts)

/// Bitwise AND
pub fn and(lhs: &[u64], rhs: &[u64], result: &mut [u64]) {
    for ((r, &a), &b) in result.iter_mut().zip(lhs.iter()).zip(rhs.iter()) {
        *r = a & b;
    }
}

/// Bitwise OR
pub fn or(lhs: &[u64], rhs: &[u64], result: &mut [u64]) {
    for ((r, &a), &b) in result.iter_mut().zip(lhs.iter()).zip(rhs.iter()) {
        *r = a | b;
    }
}

/// Bitwise XOR
pub fn xor(lhs: &[u64], rhs: &[u64], result: &mut [u64]) {
    for ((r, &a), &b) in result.iter_mut().zip(lhs.iter()).zip(rhs.iter()) {
        *r = a ^ b;
    }
}

/// Bitwise NOT
pub fn not(limbs: &[u64], result: &mut [u64]) {
    for (r, &a) in result.iter_mut().zip(limbs.iter()) {
        *r = !a;
    }
}

/// Left shift by bits (0 < bits < 64)
pub fn shl_small(limbs: &mut [u64], bits: u32) {
    if bits == 0 || bits >= 64 {
        return;
    }

    let mut carry = 0u64;
    for limb in limbs.iter_mut() {
        let new_carry = *limb >> (64 - bits);
        *limb = (*limb << bits) | carry;
        carry = new_carry;
    }
}

/// Right shift by bits (0 < bits < 64)
pub fn shr_small(limbs: &mut [u64], bits: u32) {
    if bits == 0 || bits >= 64 {
        return;
    }

    let mut carry = 0u64;
    for limb in limbs.iter_mut().rev() {
        let new_carry = *limb << (64 - bits);
        *limb = (*limb >> bits) | carry;
        carry = new_carry;
    }
}

/// Count leading zeros across all limbs
pub fn leading_zeros(limbs: &[u64]) -> u32 {
    for limb in limbs.iter().rev() {
        let zeros = limb.leading_zeros();
        if zeros < 64 {
            return zeros;
        }
    }
    (limbs.len() as u32) * 64
}

/// Count trailing zeros across all limbs
pub fn trailing_zeros(limbs: &[u64]) -> u32 {
    for limb in limbs.iter() {
        let zeros = limb.trailing_zeros();
        if zeros < 64 {
            return zeros;
        }
    }
    (limbs.len() as u32) * 64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and() {
        let a = [0b1010u64, 0b1100];
        let b = [0b1001u64, 0b1010];
        let mut result = [0u64; 2];
        and(&a, &b, &mut result);
        assert_eq!(result, [0b1000, 0b1000]);
    }

    #[test]
    fn test_or() {
        let a = [0b1010u64, 0b1100];
        let b = [0b1001u64, 0b1010];
        let mut result = [0u64; 2];
        or(&a, &b, &mut result);
        assert_eq!(result, [0b1011, 0b1110]);
    }

    #[test]
    fn test_shl_small() {
        let mut limbs = [0b1010u64, 0b0001];
        shl_small(&mut limbs, 2);
        assert_eq!(limbs, [0b101000, 0b0100]);
    }

    #[test]
    fn test_leading_zeros() {
        let limbs = [0u64, 0u64, 0b1000_0000_0000_0000u64];
        assert_eq!(leading_zeros(&limbs), 0);
    }
}
