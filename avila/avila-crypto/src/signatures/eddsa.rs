//! EdDSA (Edwards-curve Digital Signature Algorithm)
//!
//! Especificamente Ed25519
//! Vantagens:
//! - Determinística (não precisa de nonce aleatório)
//! - Constant-time
//! - Rápida

use super::SignatureVerification;
use crate::curves::curve25519::Curve25519;
use crate::curves::Point;
use crate::hash::sha512::Sha512;
use alloc::vec::Vec;
use avila_math::inverse::mod_inverse;
use avila_math::modular::{add_mod, mul_mod, pow_mod, sub_mod};
use avila_primitives::U256;
use core::cmp::Ordering;
use core::convert::TryInto;

const FIELD_MODULUS: U256 = U256 {
    limbs: Curve25519::P_LIMBS,
};

const ORDER_L: U256 = U256 {
    limbs: Curve25519::L_LIMBS,
};

const EDWARDS_D: U256 = U256 {
    limbs: Curve25519::D_LIMBS,
};

const BASE_256: U256 = U256 {
    limbs: [256, 0, 0, 0],
};

const SQRT_EXPONENT: U256 = U256 {
    limbs: [
        0xFFFFFFFFFFFFFFFE,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x0FFFFFFFFFFFFFFF,
    ],
};

const SQRT_M1_EXPONENT: U256 = U256 {
    limbs: [
        0xFFFFFFFFFFFFFFFB,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x1FFFFFFFFFFFFFFF,
    ],
};

/// Assinatura Ed25519: (R, S)
#[derive(Debug, Clone, Copy)]
pub struct Ed25519Signature {
    /// Ponto R (32 bytes compressed)
    pub r: [u8; 32],
    /// Scalar S (32 bytes)
    pub s: [u8; 32],
}

/// Chave pública Ed25519
#[derive(Debug, Clone, Copy)]
pub struct Ed25519PublicKey {
    /// Ponto A compressed (32 bytes)
    pub point: [u8; 32],
}

/// Chave privada Ed25519
#[derive(Debug, Clone, Copy)]
pub struct Ed25519PrivateKey {
    /// Seed (32 bytes)
    pub seed: [u8; 32],
}

impl Ed25519PrivateKey {
    /// Deriva chave pública
    ///
    /// 1. h = SHA-512(seed)
    /// 2. a = clamp(h[0..32])
    /// 3. A = a × G
    pub fn public_key(&self) -> Ed25519PublicKey {
        let hash = Sha512::hash(&self.seed);

        let mut a_bytes = [0u8; 32];
        a_bytes.copy_from_slice(&hash[..32]);
        clamp_scalar(&mut a_bytes);

        let a_scalar = u256_from_le_bytes(&a_bytes);
        let point = Curve25519::scalar_mul(&a_scalar, &Curve25519::G);
        let encoded = encode_point(&point);

        Ed25519PublicKey { point: encoded }
    }

    /// Assina mensagem (determinístico)
    ///
    /// Ed25519 signing:
    /// 1. h = SHA-512(seed)
    /// 2. a = clamp(h[0..32])
    /// 3. prefix = h[32..64]
    /// 4. r = SHA-512(prefix || message)
    /// 5. R = r × G
    /// 6. k = SHA-512(R || A || message)
    /// 7. S = (r + k × a) mod L
    pub fn sign(&self, message: &[u8]) -> Ed25519Signature {
        let hash = Sha512::hash(&self.seed);

        let mut a_bytes = [0u8; 32];
        a_bytes.copy_from_slice(&hash[..32]);
        clamp_scalar(&mut a_bytes);
        let a_scalar = u256_from_le_bytes(&a_bytes);

        let public_point = Curve25519::scalar_mul(&a_scalar, &Curve25519::G);
        let public_bytes = encode_point(&public_point);

        let prefix = &hash[32..];
        let mut r_input = Vec::with_capacity(prefix.len() + message.len());
        r_input.extend_from_slice(prefix);
        r_input.extend_from_slice(message);
        let r_hash = Sha512::hash(&r_input);
        let r_scalar = reduce_scalar(&r_hash);

        let r_point = Curve25519::scalar_mul(&r_scalar, &Curve25519::G);
        let r_encoded = encode_point(&r_point);

        let mut k_input = Vec::with_capacity(r_encoded.len() + public_bytes.len() + message.len());
        k_input.extend_from_slice(&r_encoded);
        k_input.extend_from_slice(&public_bytes);
        k_input.extend_from_slice(message);
        let k_hash = Sha512::hash(&k_input);
        let k_scalar = reduce_scalar(&k_hash);

        let k_times_a = mul_mod(&k_scalar, &a_scalar, &ORDER_L);
        let s_scalar = add_mod(&r_scalar, &k_times_a, &ORDER_L);

        Ed25519Signature {
            r: r_encoded,
            s: u256_to_le_bytes(&s_scalar),
        }
    }
}

impl Ed25519PublicKey {
    /// Verifica assinatura
    ///
    /// Ed25519 verification:
    /// 1. Decodifica R e S
    /// 2. k = SHA-512(R || A || message)
    /// 3. Verifica: S×G == R + k×A
    pub fn verify(&self, message: &[u8], sig: &Ed25519Signature) -> SignatureVerification {
        let a_point = match decode_point(&self.point) {
            Some(point) if Curve25519::is_on_curve(&point) => point,
            _ => return SignatureVerification::Invalid,
        };

        let r_point = match decode_point(&sig.r) {
            Some(point) => point,
            None => return SignatureVerification::Invalid,
        };

        let s_scalar = u256_from_le_bytes(&sig.s);
        if cmp_u256(&s_scalar, &ORDER_L) != Ordering::Less {
            return SignatureVerification::Invalid;
        }

        let mut k_input = Vec::with_capacity(sig.r.len() + self.point.len() + message.len());
        k_input.extend_from_slice(&sig.r);
        k_input.extend_from_slice(&self.point);
        k_input.extend_from_slice(message);
        let k_hash = Sha512::hash(&k_input);
        let k_scalar = reduce_scalar(&k_hash);

        let left = Curve25519::scalar_mul(&s_scalar, &Curve25519::G);
        let k_a = Curve25519::scalar_mul(&k_scalar, &a_point);
        let right = Curve25519::add(&r_point, &k_a);

        let cofactor_scalar = U256 {
            limbs: [Curve25519::H as u64, 0, 0, 0],
        };

        let left_projected = Curve25519::scalar_mul(&cofactor_scalar, &left);
        let right_projected = Curve25519::scalar_mul(&cofactor_scalar, &right);

        if left_projected == right_projected {
            SignatureVerification::Valid
        } else {
            SignatureVerification::Invalid
        }
    }
}

fn clamp_scalar(bytes: &mut [u8; 32]) {
    bytes[0] &= 248;
    bytes[31] &= 63;
    bytes[31] |= 64;
}

fn u256_from_le_bytes(bytes: &[u8; 32]) -> U256 {
    let mut limbs = [0u64; 4];
    for (i, chunk) in bytes.chunks_exact(8).enumerate() {
        let limb = u64::from_le_bytes(chunk.try_into().unwrap());
        limbs[i] = limb;
    }
    U256 { limbs }
}

fn u256_to_le_bytes(value: &U256) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, chunk) in out.chunks_exact_mut(8).enumerate() {
        chunk.copy_from_slice(&value.limbs[i].to_le_bytes());
    }
    out
}

fn u256_from_u64(value: u64) -> U256 {
    U256 {
        limbs: [value, 0, 0, 0],
    }
}

fn reduce_scalar(hash: &[u8; 64]) -> U256 {
    let mut acc = U256::ZERO;
    let mut factor = U256::ONE;

    for &byte in hash.iter() {
        if byte != 0 {
            let term = mul_mod(&factor, &u256_from_u64(byte as u64), &ORDER_L);
            acc = add_mod(&acc, &term, &ORDER_L);
        }
        factor = mul_mod(&factor, &BASE_256, &ORDER_L);
    }

    acc
}

fn decode_point(bytes: &[u8; 32]) -> Option<Point> {
    let mut y_bytes = *bytes;
    let sign = (y_bytes[31] >> 7) != 0;
    y_bytes[31] &= 0x7F;

    let y = u256_from_le_bytes(&y_bytes);
    if cmp_u256(&y, &FIELD_MODULUS) != Ordering::Less {
        return None;
    }

    let y2 = mul_mod(&y, &y, &FIELD_MODULUS);
    let u = sub_mod(&y2, &U256::ONE, &FIELD_MODULUS);
    let v = add_mod(&mul_mod(&EDWARDS_D, &y2, &FIELD_MODULUS), &U256::ONE, &FIELD_MODULUS);

    let v_inv = mod_inverse(&v, &FIELD_MODULUS)?;
    let x_sq = mul_mod(&u, &v_inv, &FIELD_MODULUS);

    let mut x = pow_mod(&x_sq, &SQRT_EXPONENT, &FIELD_MODULUS);
    if mul_mod(&x, &x, &FIELD_MODULUS) != x_sq {
        let sqrt_m1 = pow_mod(&sub_mod(&FIELD_MODULUS, &U256::ONE, &FIELD_MODULUS), &SQRT_M1_EXPONENT, &FIELD_MODULUS);
        x = mul_mod(&x, &sqrt_m1, &FIELD_MODULUS);
    }

    if mul_mod(&x, &x, &FIELD_MODULUS) != x_sq {
        return None;
    }

    if (x.limbs[0] & 1 != 0) != sign {
        x = sub_mod(&FIELD_MODULUS, &x, &FIELD_MODULUS);
    }

    let point = Point { x, y };
    if Curve25519::is_on_curve(&point) {
        Some(point)
    } else {
        None
    }
}

fn encode_point(point: &Point) -> [u8; 32] {
    let mut y_bytes = u256_to_le_bytes(&point.y);
    let x_bytes = u256_to_le_bytes(&point.x);
    y_bytes[31] |= (x_bytes[0] & 1) << 7;
    y_bytes
}

fn cmp_u256(a: &U256, b: &U256) -> Ordering {
    for i in (0..a.limbs.len()).rev() {
        match a.limbs[i].cmp(&b.limbs[i]) {
            Ordering::Equal => continue,
            ord => return ord,
        }
    }
    Ordering::Equal
}
