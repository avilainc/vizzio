//! # Ávila Crypto
//!
//! Criptografia soberana - apenas algoritmos aprovados pela Ávila.
//! Zero compromissos com agências governamentais.
//!
//! ## Filosofia
//! - **secp256k1**: Battle-tested pelo Bitcoin
//! - **Curve25519**: Moderna e constant-time
//! - **BLS12-381**: Para threshold signatures e ZK
//! - **BLAKE3**: Mais rápido que SHA, mais seguro
//! - **Schnorr**: Agregação elegante
//! - **Ed25519**: Determinística e rápida
//!
//! ## NÃO USAMOS
//! - P-256 (NIST): constantes suspeitas
//! - RSA: lento e legado
//! - SHA-2: aprovado demais pelos governos

#![no_std]
extern crate alloc;
#![deny(unsafe_op_in_unsafe_fn)]
#![forbid(unsafe_code)]
#![deny(unreachable_pub)]
#![deny(rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(clippy, deny(clippy::all))]
#![cfg_attr(clippy, deny(clippy::pedantic))]
#![warn(missing_docs)]

pub mod curves;
pub mod signatures;
pub mod hash;
pub mod cipher;
pub mod mac;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
