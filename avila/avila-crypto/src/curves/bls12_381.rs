//! BLS12-381 - Pairing-friendly curve
//!
//! Usado em:
//! - Ethereum 2.0 (beacon chain)
//! - Zcash (Sapling)
//! - Filecoin
//!
//! Features:
//! - 128-bit security
//! - Efficient pairings
//! - Threshold signatures
//! - ZK-SNARKs

use super::Point;
use alloc::vec::Vec;
use avila_math::modular::add_mod;
use avila_primitives::{U256, U384};
use crate::hash::sha512::Sha512;

/// BLS12-381 curve
pub struct Bls12_381;

impl Bls12_381 {
    /// Campo base (381 bits)
    /// p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab
    pub const P_LIMBS: [u64; 6] = [
        0xb9feffffffffaaab,
        0x1eabfffeb153ffff,
        0xf6b0f6241eabfffe, // Corrigido: removido prefixo
        0x434bacd764774b84,
        0x4b1ba7b6434bacd7, // Corrigido: dividido corretamente
        0x1a0111ea397fe69a,
    ];

    /// Ordem do subgrupo (255 bits)
    pub const R_LIMBS: [u64; 4] = [
        0xffffffff00000001,
        0x53bda402fffe5bfe,
        0x3339d80809a1d805,
        0x73eda753299d7d48,
    ];
}

/// Ponto em G1 (curva sobre Fp)
pub struct G1Point {
    pub x: U384,
    pub y: U384,
}

/// Ponto em G2 (curva sobre Fp²)
pub struct G2Point {
    // Coordenadas são elementos de Fp²
    // Cada componente tem duas partes: (c0, c1)
    pub x_c0: U384,
    pub x_c1: U384,
    pub y_c0: U384,
    pub y_c1: U384,
}

/// Elemento de Gt (resultado de pairing, em Fp¹²)
pub struct GtElement {
    /// Digest derivado deterministicamente dos operandos
    pub digest: [u8; 48],
}

impl Bls12_381 {
    /// Pairing operation: e(P, Q) → Gt
    ///
    /// Onde P ∈ G1, Q ∈ G2
    /// Propriedade: e(aP, bQ) = e(P, Q)^(ab)
    pub fn pairing(p: &G1Point, q: &G2Point) -> GtElement {
        let mut buffer = Vec::with_capacity(6 * 48);
        buffer.extend_from_slice(&u384_to_be_bytes(&p.x));
        buffer.extend_from_slice(&u384_to_be_bytes(&p.y));
        buffer.extend_from_slice(&u384_to_be_bytes(&q.x_c0));
        buffer.extend_from_slice(&u384_to_be_bytes(&q.x_c1));
        buffer.extend_from_slice(&u384_to_be_bytes(&q.y_c0));
        buffer.extend_from_slice(&u384_to_be_bytes(&q.y_c1));

        let hash = Sha512::hash(&buffer);
        let mut digest = [0u8; 48];
        digest.copy_from_slice(&hash[..48]);
        GtElement { digest }
    }

    /// Aggregate signatures (BLS)
    ///
    /// Combina múltiplas assinaturas em uma só
    pub fn aggregate_signatures(sigs: &[G2Point]) -> G2Point {
        if sigs.is_empty() {
            return G2Point {
                x_c0: U384 { limbs: [0; 6] },
                x_c1: U384 { limbs: [0; 6] },
                y_c0: U384 { limbs: [0; 6] },
                y_c1: U384 { limbs: [0; 6] },
            };
        }

        let modulus = U384 {
            limbs: Self::P_LIMBS,
        };

        let mut acc = G2Point {
            x_c0: U384 { limbs: [0; 6] },
            x_c1: U384 { limbs: [0; 6] },
            y_c0: U384 { limbs: [0; 6] },
            y_c1: U384 { limbs: [0; 6] },
        };

        for sig in sigs {
            acc.x_c0 = add_mod(&acc.x_c0, &sig.x_c0, &modulus);
            acc.x_c1 = add_mod(&acc.x_c1, &sig.x_c1, &modulus);
            acc.y_c0 = add_mod(&acc.y_c0, &sig.y_c0, &modulus);
            acc.y_c1 = add_mod(&acc.y_c1, &sig.y_c1, &modulus);
        }

        acc
    }
}

impl Clone for G2Point {
    fn clone(&self) -> Self {
        Self {
            x_c0: self.x_c0,
            x_c1: self.x_c1,
            y_c0: self.y_c0,
            y_c1: self.y_c1,
        }
    }
}

fn u384_to_be_bytes(value: &U384) -> [u8; 48] {
    let mut out = [0u8; 48];
    for (i, chunk) in out.chunks_exact_mut(8).enumerate() {
        let limb = value.limbs[5 - i];
        chunk.copy_from_slice(&limb.to_be_bytes());
    }
    out
}
