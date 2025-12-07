//! ECDSA - Elliptic Curve Digital Signature Algorithm
//!
//! Usado em Bitcoin e Ethereum (com secp256k1)

use super::SignatureVerification;
use crate::curves::{secp256k1::Secp256k1, Point, EllipticCurve};
use avila_primitives::U256;
use avila_math::modular::{add_mod, mul_mod};
use avila_math::inverse::mod_inverse;

/// Assinatura ECDSA: (r, s)
#[derive(Debug, Clone, Copy)]
pub struct EcdsaSignature {
    /// Componente r da assinatura
    pub r: U256,
    /// Componente s da assinatura
    pub s: U256,
}

/// Chave pública ECDSA
#[derive(Debug, Clone, Copy)]
pub struct EcdsaPublicKey {
    /// Ponto na curva elíptica
    pub point: Point,
}

/// Chave privada ECDSA
#[derive(Debug, Clone, Copy)]
pub struct EcdsaPrivateKey {
    /// Escalar privado
    pub scalar: U256,
}

impl EcdsaPrivateKey {
    /// Deriva chave pública: Q = d × G
    pub fn public_key(&self) -> EcdsaPublicKey {
        let point = Secp256k1::scalar_mul(&self.scalar, &Secp256k1::G);
        EcdsaPublicKey { point }
    }

    /// Assina mensagem
    ///
    /// Algoritmo:
    /// 1. e = hash(message)
    /// 2. k = random nonce (CRÍTICO: deve ser único!)
    /// 3. R = k × G
    /// 4. r = R.x mod n
    /// 5. s = k⁻¹ × (e + r × d) mod n
    /// 6. Retorna (r, s)
    pub fn sign(&self, message_hash: &U256, nonce: &U256) -> EcdsaSignature {
        let n = &Secp256k1::N;

        // R = k × G
        let r_point = Secp256k1::scalar_mul(nonce, &Secp256k1::G);

        // r = R.x mod n
        let r = Self::mod_n(&r_point.x, n);

        // Calcula k⁻¹ mod n
        let k_inv = mod_inverse(nonce, n).expect("Invalid nonce");

        // e + r × d mod n
        let r_times_d = mul_mod(&r, &self.scalar, n);
        let e_plus_rd = add_mod(message_hash, &r_times_d, n);

        // s = k⁻¹ × (e + r × d) mod n
        let s = mul_mod(&k_inv, &e_plus_rd, n);

        EcdsaSignature { r, s }
    }

    /// Reduz U256 módulo n
    fn mod_n(value: &U256, n: &U256) -> U256 {
        // Implementação simples: subtração repetida se value >= n
        let mut result = *value;
        while !result.is_zero() && result >= *n {
            result = result.wrapping_sub(n);
        }
        result
    }
}

impl EcdsaPublicKey {
    /// Verifica assinatura
    ///
    /// Algoritmo:
    /// 1. Verifica r, s ∈ [1, n-1]
    /// 2. e = hash(message)
    /// 3. w = s⁻¹ mod n
    /// 4. u₁ = e × w mod n
    /// 5. u₂ = r × w mod n
    /// 6. R = u₁×G + u₂×Q
    /// 7. Verifica r == R.x mod n
    pub fn verify(&self, message_hash: &U256, sig: &EcdsaSignature) -> SignatureVerification {
        // Validações básicas
        if sig.r.is_zero() || sig.s.is_zero() {
            return SignatureVerification::Invalid;
        }

        let n = &Secp256k1::N;

        // Verifica r, s < n
        if sig.r >= *n || sig.s >= *n {
            return SignatureVerification::Invalid;
        }

        // w = s⁻¹ mod n
        let w = match mod_inverse(&sig.s, n) {
            Some(inv) => inv,
            None => return SignatureVerification::Invalid,
        };

        // u₁ = e × w mod n
        let u1 = mul_mod(message_hash, &w, n);

        // u₂ = r × w mod n
        let u2 = mul_mod(&sig.r, &w, n);

        // R = u₁×G + u₂×Q (soma de dois pontos escalares)
        let p1 = Secp256k1::scalar_mul(&u1, &Secp256k1::G);
        let p2 = Secp256k1::scalar_mul(&u2, &self.point);
        let r_point = Secp256k1::add(&p1, &p2);

        // Verifica se ponto no infinito
        if r_point.is_infinity() {
            return SignatureVerification::Invalid;
        }

        // Verifica R.x mod n == r
        let r_x = EcdsaPrivateKey::mod_n(&r_point.x, n);

        if r_x == sig.r {
            SignatureVerification::Valid
        } else {
            SignatureVerification::Invalid
        }
    }

    /// Recupera chave pública da assinatura (Ethereum style)
    ///
    /// recovery_id (v) determina qual dos 2-4 possíveis pontos:
    /// - Bit 0: paridade de y (0 = par, 1 = ímpar)
    /// - Bit 1: se x overflowed n (raramente usado)
    pub fn recover(
        message_hash: &U256,
        sig: &EcdsaSignature,
        recovery_id: u8,
    ) -> Option<Self> {
        let n = &Secp256k1::N;
        let p = &Secp256k1::P;

        // Reconstrói ponto R a partir de r
        let x = if recovery_id & 2 == 0 {
            sig.r
        } else {
            // Caso raro: x = r + n
            let result = sig.r.wrapping_add(n);
            if result >= *p {
                return None;
            }
            result
        };

        // Calcula y² = x³ + 7
        let x_cubed = mul_mod(&mul_mod(&x, &x, p), &x, p);
        let y_squared = add_mod(&x_cubed, &Secp256k1::B, p);

        // Calcula y = √(y²) mod p usando exponenciação
        // Para p ≡ 3 (mod 4), y = y²^((p+1)/4) mod p
        let y = Self::sqrt_mod_p(&y_squared, p)?;

        // Escolhe y baseado na paridade
        let y_is_odd = (y.limbs[0] & 1) != 0;
        let want_odd = (recovery_id & 1) != 0;
        let y = if y_is_odd == want_odd {
            y
        } else {
            p.wrapping_sub(&y)
        };

        let r_point = Point { x, y };

        // Verifica se ponto está na curva
        if !Secp256k1::is_on_curve(&r_point) {
            return None;
        }

        // r⁻¹ mod n
        let r_inv = mod_inverse(&sig.r, n)?;

        // Q = r⁻¹ × (s×R - e×G)
        // = r⁻¹×s×R - r⁻¹×e×G

        let s_r = Secp256k1::scalar_mul(&sig.s, &r_point);
        let e_g = Secp256k1::scalar_mul(message_hash, &Secp256k1::G);

        // Negação de ponto: (x, -y) = (x, p - y)
        let e_g_neg = Point {
            x: e_g.x,
            y: p.wrapping_sub(&e_g.y),
        };        let diff = Secp256k1::add(&s_r, &e_g_neg);
        let public_point = Secp256k1::scalar_mul(&r_inv, &diff);

        Some(EcdsaPublicKey {
            point: public_point,
        })
    }

    /// Calcula raiz quadrada módulo p (Tonelli-Shanks simplificado)
    /// Para secp256k1, p ≡ 3 (mod 4), então é simples
    fn sqrt_mod_p(a: &U256, p: &U256) -> Option<U256> {
        // Expoente = (p + 1) / 4
        let mut exp = *p;
        exp.limbs[0] = exp.limbs[0].wrapping_add(1);

        // Shift right by 2 (divide por 4)
        let carry = exp.limbs[3] & 3;
        exp.limbs[3] >>= 2;
        exp.limbs[2] = (exp.limbs[2] >> 2) | ((exp.limbs[3] & 3) << 62);
        exp.limbs[1] = (exp.limbs[1] >> 2) | ((exp.limbs[2] & 3) << 62);
        exp.limbs[0] = (exp.limbs[0] >> 2) | ((exp.limbs[1] & 3) << 62);

        // y = a^exp mod p
        let y = Self::pow_mod(a, &exp, p);

        // Verifica: y² mod p == a
        let y_squared = mul_mod(&y, &y, p);
        if y_squared == *a {
            Some(y)
        } else {
            None
        }
    }

    /// Exponenciação modular (square-and-multiply)
    fn pow_mod(base: &U256, exp: &U256, modulus: &U256) -> U256 {
        let mut result = U256::ONE;
        let mut base = *base;

        for limb in exp.limbs.iter() {
            for bit in 0..64 {
                if (limb >> bit) & 1 == 1 {
                    result = mul_mod(&result, &base, modulus);
                }
                base = mul_mod(&base, &base, modulus);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pubkey_derivation() {
        let privkey = EcdsaPrivateKey {
            scalar: U256::from_u64(12345),
        };
        let pubkey = privkey.public_key();
        assert!(Secp256k1::is_on_curve(&pubkey.point));
    }

    #[test]
    fn test_sign_verify() {
        // Chave privada de teste
        let privkey = EcdsaPrivateKey {
            scalar: U256::from_u64(0x1234567890abcdef),
        };
        let pubkey = privkey.public_key();

        // Hash da mensagem (simulado)
        let message_hash = U256::from_u64(0xdeadbeef);

        // Nonce (em produção deve ser criptograficamente seguro e único!)
        let nonce = U256::from_u64(0x9876543210fedcba);

        // Assina
        let signature = privkey.sign(&message_hash, &nonce);

        // Verifica
        let result = pubkey.verify(&message_hash, &signature);
        assert_eq!(result, SignatureVerification::Valid);
    }
}
