//! Addition operations with carry handling

/// Add two u64 values and return the sum and carry
#[inline]
pub const fn adc(a: u64, b: u64, carry: u64) -> (u64, u64) {
    let sum = a as u128 + b as u128 + carry as u128;
    (sum as u64, (sum >> 64) as u64)
}

/// Add two limb arrays with carry propagation
pub fn add_assign(result: &mut [u64], rhs: &[u64]) -> u64 {
    let mut carry = 0u64;
    for (a, &b) in result.iter_mut().zip(rhs.iter()) {
        let (sum, c) = adc(*a, b, carry);
        *a = sum;
        carry = c;
    }
    carry
}

/// Add two limb arrays and return result
pub fn add(lhs: &[u64], rhs: &[u64], result: &mut [u64]) -> u64 {
    result.copy_from_slice(lhs);
    add_assign(result, rhs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adc_no_carry() {
        let (sum, carry) = adc(10, 20, 0);
        assert_eq!(sum, 30);
        assert_eq!(carry, 0);
    }

    #[test]
    fn test_adc_with_carry() {
        let (sum, carry) = adc(u64::MAX, 1, 0);
        assert_eq!(sum, 0);
        assert_eq!(carry, 1);
    }

    #[test]
    fn test_add_assign() {
        let mut a = [10u64, 20u64];
        let b = [5u64, 15u64];
        let carry = add_assign(&mut a, &b);
        assert_eq!(a, [15, 35]);
        assert_eq!(carry, 0);
    }
}
