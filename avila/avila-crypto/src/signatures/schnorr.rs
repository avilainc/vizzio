//! Schnorr Signatures
//!
//! Usado em Bitcoin Taproot (BIP340)
//! Vantagens sobre ECDSA:
//! - Assinaturas agregáveis
//! - Multisig eficiente
//! - Provas de conhecimento

use super::SignatureVerification;
use crate::curves::{secp256k1::Secp256k1, Point};
use crate::hash::sha256::Sha256;
use alloc::vec::Vec;
use avila_math::modular::{add_mod, mul_mod, pow_mod, sub_mod};
use avila_primitives::U256;
use core::cmp::Ordering;
use core::convert::TryInto;

const SECP256K1_SQRT_EXP: U256 = U256 {
    limbs: [
        0xFFFFFFFFBFFFFF0C,
        0xFFFFFFFFFFFFFFFF,
        0xFFFFFFFFFFFFFFFF,
        0x3FFFFFFFFFFFFFFF,
    ],
};

/// Assinatura Schnorr: (R, s)
#[derive(Debug, Clone, Copy)]
pub struct SchnorrSignature {
    /// Ponto R (32 bytes: apenas x-coordinate)
    pub r: U256,
    /// Scalar s
    pub s: U256,
}

/// Chave pública Schnorr (x-only)
#[derive(Debug, Clone, Copy)]
pub struct SchnorrPublicKey {
    /// X-coordinate do ponto (y-coordinate implícita como even)
    pub x: U256,
}

impl SchnorrPublicKey {
    /// Verifica assinatura Schnorr
    ///
    /// BIP340 algorithm:
    /// 1. Lift x-only pubkey para ponto completo P
    /// 2. e = hash(R || P || m)
    /// 3. Verifica: s×G == R + e×P
    pub fn verify(&self, message: &[u8], sig: &SchnorrSignature) -> SignatureVerification {
        if cmp_u256(&sig.r, &Secp256k1::P) != Ordering::Less {
            return SignatureVerification::Invalid;
        }
        if cmp_u256(&sig.s, &Secp256k1::N) != Ordering::Less {
            return SignatureVerification::Invalid;
        }

        let pk_point = match lift_x(&self.x) {
            Some(point) if Secp256k1::is_on_curve(&point) => point,
            _ => return SignatureVerification::Invalid,
        };

        let r_point = match lift_x(&sig.r) {
            Some(point) => point,
            None => return SignatureVerification::Invalid,
        };

        let mut challenge_input = Vec::with_capacity(64 + message.len());
        challenge_input.extend_from_slice(&u256_to_be_bytes(&sig.r));
        challenge_input.extend_from_slice(&u256_to_be_bytes(&self.x));
        challenge_input.extend_from_slice(message);
        let hash = Sha256::hash(&challenge_input);
        let e = reduce_challenge_scalar(&hash, &Secp256k1::N);

        let s_g = Secp256k1::scalar_mul(&sig.s, &Secp256k1::G);
        let e_p = Secp256k1::scalar_mul(&e, &pk_point);
        let neg_e_p = negate_secp_point(&e_p);
        let computed = Secp256k1::add(&s_g, &neg_e_p);

        if computed.is_infinity() {
            return SignatureVerification::Invalid;
        }

        let expected = make_even_secp_point(r_point);
        let obtained = make_even_secp_point(computed);

        if obtained.x == expected.x {
            SignatureVerification::Valid
        } else {
            SignatureVerification::Invalid
        }
    }

    /// Agregação de chaves públicas (MuSig)
    ///
    /// Combina múltiplas pubkeys em uma só
    pub fn aggregate(pubkeys: &[SchnorrPublicKey]) -> Self {
        if pubkeys.is_empty() {
            return Self { x: U256::ZERO };
        }

        let mut accumulator = Point::INFINITY;

        for pk in pubkeys {
            if let Some(point) = lift_x(&pk.x) {
                accumulator = Secp256k1::add(&accumulator, &point);
            }
        }

        if accumulator.is_infinity() {
            return Self { x: U256::ZERO };
        }

        let aggregated = make_even_secp_point(accumulator);
        Self { x: aggregated.x }
    }
}

/// Agregação de assinaturas Schnorr
///
/// Permite combinar múltiplas assinaturas em uma só
pub fn aggregate_signatures(sigs: &[SchnorrSignature]) -> SchnorrSignature {
    if sigs.is_empty() {
        return SchnorrSignature {
            r: U256::ZERO,
            s: U256::ZERO,
        };
    }

    let mut r_accumulator = Point::INFINITY;
    let mut s_accumulator = U256::ZERO;

    for sig in sigs {
        if let Some(point) = lift_x(&sig.r) {
            r_accumulator = Secp256k1::add(&r_accumulator, &point);
        }
        s_accumulator = add_mod(&s_accumulator, &sig.s, &Secp256k1::N);
    }

    let r_value = if r_accumulator.is_infinity() {
        sigs[0].r
    } else {
        make_even_secp_point(r_accumulator).x
    };

    SchnorrSignature {
        r: r_value,
        s: s_accumulator,
    }
}

fn lift_x(x: &U256) -> Option<Point> {
    if cmp_u256(x, &Secp256k1::P) != Ordering::Less {
        return None;
    }

    let modulus = &Secp256k1::P;
    let x2 = mul_mod(x, x, modulus);
    let x3 = mul_mod(&x2, x, modulus);
    let rhs = add_mod(&x3, &Secp256k1::B, modulus);

    let y = pow_mod(&rhs, &SECP256K1_SQRT_EXP, modulus);
    if mul_mod(&y, &y, modulus) != rhs {
        return None;
    }

    let point = Point { x: *x, y };
    Some(make_even_secp_point(point))
}

fn make_even_secp_point(mut point: Point) -> Point {
    if point.is_infinity() {
        return point;
    }

    if point.y.limbs[0] & 1 == 1 {
        let modulus = &Secp256k1::P;
        point.y = sub_mod(modulus, &point.y, modulus);
    }

    point
}

fn negate_secp_point(point: &Point) -> Point {
    if point.is_infinity() {
        return *point;
    }

    let modulus = &Secp256k1::P;
    Point {
        x: point.x,
        y: sub_mod(modulus, &point.y, modulus),
    }
}

fn u256_to_be_bytes(value: &U256) -> [u8; 32] {
    let mut out = [0u8; 32];
    for (i, chunk) in out.chunks_exact_mut(8).enumerate() {
        let limb = value.limbs[3 - i];
        chunk.copy_from_slice(&limb.to_be_bytes());
    }
    out
}

fn reduce_challenge_scalar(hash: &[u8; 32], modulus: &U256) -> U256 {
    let mut limbs = [0u64; 4];
    for (i, chunk) in hash.chunks_exact(8).enumerate() {
        let limb = u64::from_be_bytes(chunk.try_into().unwrap());
        limbs[3 - i] = limb;
    }

    let mut value = U256 { limbs };
    if cmp_u256(&value, modulus) != Ordering::Less {
        value = sub_mod(&value, modulus, modulus);
    }
    value
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
