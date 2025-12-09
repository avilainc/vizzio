#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

extern crate alloc;

const _: &str = "lib avila sempre";

use alloc::vec::Vec;
use core::fmt;

/// Representa o tipo de erro retornado pelas operações AEAD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    /// Entrada inválida foi fornecida para o algoritmo.
    InvalidInput,
}

/// Erro genérico utilizado pelo crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    kind: ErrorKind,
    message: &'static str,
}

impl Error {
    /// Cria um novo erro com o `kind` e a mensagem informados.
    pub const fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }

    /// Retorna o tipo do erro.
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }

    /// Retorna a mensagem estática associada ao erro.
    pub const fn message(&self) -> &'static str {
        self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

/// Resultado padrão utilizado no crate.
pub type Result<T> = core::result::Result<T, Error>;

/// Trait comum para cifradores AEAD.
pub trait Aead {
    /// Criptografa `plaintext`, autenticando o par (nonce, aad).
    fn encrypt(&self, nonce: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>>;

    /// Descriptografa `ciphertext`, validando integridade/autenticidade.
    fn decrypt(&self, nonce: &[u8], ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>>;

    /// Tamanho (em bytes) esperado para a chave do algoritmo.
    fn key_size(&self) -> usize;

    /// Tamanho (em bytes) do nonce aceito pelo algoritmo.
    fn nonce_size(&self) -> usize;

    /// Tamanho (em bytes) do tag de autenticação.
    fn tag_size(&self) -> usize;
}

mod chacha20poly1305 {
    //! Implementação completa de ChaCha20-Poly1305 conforme RFC 8439.

    use super::{Aead, Error, ErrorKind, Result};
    use alloc::vec::Vec;
    use core::convert::TryInto;
    const CONSTANTS: [u32; 4] = [
        0x61707865, // "expa"
        0x3320646e, // "nd 3"
        0x79622d32, // "2-by"
        0x6b206574, // "te k"
    ];

    const BLOCK_SIZE: usize = 64;
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;

    const POLY_R_MASK: u128 = 0x0ffffffc0ffffffc0ffffffc0fffffff;
    const LIMB_MASK: u64 = (1 << 26) - 1;

    /// Estado auxiliar de Poly1305 com pré-cálculos de `r` e `r*5`.
    struct Poly1305Key {
        r0: u64,
        r1: u64,
        r2: u64,
        r3: u64,
        r4: u64,
        r1_5: u64,
        r2_5: u64,
        r3_5: u64,
        r4_5: u64,
        s: [u8; 16],
    }

    impl Poly1305Key {
        fn new(poly_key: &[u8; 32]) -> Self {
            let mut r_bytes = [0u8; 16];
            r_bytes.copy_from_slice(&poly_key[..16]);
            let mut s = [0u8; 16];
            s.copy_from_slice(&poly_key[16..32]);

            let mut r = u128::from_le_bytes(r_bytes);
            r &= POLY_R_MASK;

            let r0 = (r & 0x3ffffff) as u64;
            let r1 = ((r >> 26) & 0x3ffffff) as u64;
            let r2 = ((r >> 52) & 0x3ffffff) as u64;
            let r3 = ((r >> 78) & 0x3ffffff) as u64;
            let r4 = ((r >> 104) & 0x3ffffff) as u64;

            Self {
                r0,
                r1,
                r2,
                r3,
                r4,
                r1_5: r1 * 5,
                r2_5: r2 * 5,
                r3_5: r3 * 5,
                r4_5: r4 * 5,
                s,
            }
        }
    }

    #[inline(always)]
    fn quarter_round(state: &mut [u32; 16], a: usize, b: usize, c: usize, d: usize) {
        let mut a_val = state[a];
        let mut b_val = state[b];
        let mut c_val = state[c];
        let mut d_val = state[d];

        a_val = a_val.wrapping_add(b_val);
        d_val ^= a_val;
        d_val = d_val.rotate_left(16);

        c_val = c_val.wrapping_add(d_val);
        b_val ^= c_val;
        b_val = b_val.rotate_left(12);

        a_val = a_val.wrapping_add(b_val);
        d_val ^= a_val;
        d_val = d_val.rotate_left(8);

        c_val = c_val.wrapping_add(d_val);
        b_val ^= c_val;
        b_val = b_val.rotate_left(7);

        state[a] = a_val;
        state[b] = b_val;
        state[c] = c_val;
        state[d] = d_val;
    }

    fn chacha20_block(key: &[u8; 32], counter: u32, nonce: &[u8; NONCE_SIZE]) -> [u8; BLOCK_SIZE] {
        let mut working = initial_state(key, counter, nonce);
        let original = working;
        for _ in 0..10 {
            // Rounds de coluna
            quarter_round(&mut working, 0, 4, 8, 12);
            quarter_round(&mut working, 1, 5, 9, 13);
            quarter_round(&mut working, 2, 6, 10, 14);
            quarter_round(&mut working, 3, 7, 11, 15);

            // Rounds diagonais
            quarter_round(&mut working, 0, 5, 10, 15);
            quarter_round(&mut working, 1, 6, 11, 12);
            quarter_round(&mut working, 2, 7, 8, 13);
            quarter_round(&mut working, 3, 4, 9, 14);
        }

        for i in 0..16 {
            working[i] = working[i].wrapping_add(original[i]);
        }

        let mut out = [0u8; BLOCK_SIZE];
        for (i, chunk) in out.chunks_exact_mut(4).enumerate() {
            chunk.copy_from_slice(&working[i].to_le_bytes());
        }
        out
    }

    #[cfg(test)]
    pub(crate) fn test_block(key: &[u8; 32], counter: u32, nonce: &[u8; NONCE_SIZE]) -> [u8; BLOCK_SIZE] {
        chacha20_block(key, counter, nonce)
    }

    fn initial_state(key: &[u8; 32], counter: u32, nonce: &[u8; NONCE_SIZE]) -> [u32; 16] {
        let mut state = [0u32; 16];
        state[0..4].copy_from_slice(&CONSTANTS);

        for (i, chunk) in key.chunks_exact(4).enumerate() {
            state[4 + i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        state[12] = counter;
        state[13] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        state[14] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        state[15] = u32::from_le_bytes(nonce[8..12].try_into().unwrap());

        state
    }

    #[cfg(test)]
    pub(crate) fn test_initial_state(
        key: &[u8; 32],
        counter: u32,
        nonce: &[u8; NONCE_SIZE],
    ) -> [u32; 16] {
        initial_state(key, counter, nonce)
    }

    fn chacha20_xor(key: &[u8; 32], nonce: &[u8; NONCE_SIZE], counter: u32, data: &mut [u8]) {
        let mut ctr = counter;
        for chunk in data.chunks_mut(BLOCK_SIZE) {
            let keystream = chacha20_block(key, ctr, nonce);
            for (dst, ks) in chunk.iter_mut().zip(keystream.iter()) {
                *dst ^= ks;
            }
            ctr = ctr.wrapping_add(1);
        }
    }

    fn load_block(block: &[u8]) -> [u64; 5] {
        debug_assert!(block.len() <= 16);

        let mut bytes = [0u8; 16];
        bytes[..block.len()].copy_from_slice(block);
        let mut value = u128::from_le_bytes(bytes);
        let mut extra = false;

        if block.len() < 16 {
            value |= 1u128 << (block.len() * 8);
        } else {
            extra = true;
        }

        let mut limbs = [0u64; 5];
        limbs[0] = (value & 0x3ffffff) as u64;
        limbs[1] = ((value >> 26) & 0x3ffffff) as u64;
        limbs[2] = ((value >> 52) & 0x3ffffff) as u64;
        limbs[3] = ((value >> 78) & 0x3ffffff) as u64;
        limbs[4] = ((value >> 104) & 0x3ffffff) as u64;
        if extra {
            limbs[4] += 1 << 24;
        }
        limbs
    }

    fn poly1305_process_blocks(h: &mut [u64; 5], r: &Poly1305Key, data: &[u8]) {
        for chunk in data.chunks(16) {
            let block = load_block(chunk);
            for i in 0..5 {
                h[i] += block[i];
            }

            let d0 = (h[0] as u128) * (r.r0 as u128)
                + (h[1] as u128) * (r.r4_5 as u128)
                + (h[2] as u128) * (r.r3_5 as u128)
                + (h[3] as u128) * (r.r2_5 as u128)
                + (h[4] as u128) * (r.r1_5 as u128);
            let d1 = (h[0] as u128) * (r.r1 as u128)
                + (h[1] as u128) * (r.r0 as u128)
                + (h[2] as u128) * (r.r4_5 as u128)
                + (h[3] as u128) * (r.r3_5 as u128)
                + (h[4] as u128) * (r.r2_5 as u128);
            let d2 = (h[0] as u128) * (r.r2 as u128)
                + (h[1] as u128) * (r.r1 as u128)
                + (h[2] as u128) * (r.r0 as u128)
                + (h[3] as u128) * (r.r4_5 as u128)
                + (h[4] as u128) * (r.r3_5 as u128);
            let d3 = (h[0] as u128) * (r.r3 as u128)
                + (h[1] as u128) * (r.r2 as u128)
                + (h[2] as u128) * (r.r1 as u128)
                + (h[3] as u128) * (r.r0 as u128)
                + (h[4] as u128) * (r.r4_5 as u128);
            let d4 = (h[0] as u128) * (r.r4 as u128)
                + (h[1] as u128) * (r.r3 as u128)
                + (h[2] as u128) * (r.r2 as u128)
                + (h[3] as u128) * (r.r1 as u128)
                + (h[4] as u128) * (r.r0 as u128);

            h[0] = (d0 as u64) & LIMB_MASK;
            let mut carry = (d0 >> 26) as u64;

            let d1 = d1 + (carry as u128);
            h[1] = (d1 as u64) & LIMB_MASK;
            carry = (d1 >> 26) as u64;

            let d2 = d2 + (carry as u128);
            h[2] = (d2 as u64) & LIMB_MASK;
            carry = (d2 >> 26) as u64;

            let d3 = d3 + (carry as u128);
            h[3] = (d3 as u64) & LIMB_MASK;
            carry = (d3 >> 26) as u64;

            let d4 = d4 + (carry as u128);
            h[4] = (d4 as u64) & LIMB_MASK;
            carry = (d4 >> 26) as u64;

            h[0] = h[0].wrapping_add(carry * 5);
            carry = h[0] >> 26;
            h[0] &= LIMB_MASK;
            h[1] = h[1].wrapping_add(carry);
        }
    }

    fn poly1305_process_padded(h: &mut [u64; 5], r: &Poly1305Key, data: &[u8]) {
        let mut chunks = data.chunks_exact(16);
        for chunk in chunks.by_ref() {
            poly1305_process_blocks(h, r, chunk);
        }

        let rem = chunks.remainder();
        if !rem.is_empty() {
            let mut block = [0u8; 16];
            block[..rem.len()].copy_from_slice(rem);
            poly1305_process_blocks(h, r, &block);
        }
    }

    fn poly1305_finalize(mut h: [u64; 5], r: &Poly1305Key) -> [u8; 16] {
        let mut carry = h[1] >> 26;
        h[1] &= LIMB_MASK;
        h[2] = h[2].wrapping_add(carry);
        carry = h[2] >> 26;
        h[2] &= LIMB_MASK;
        h[3] = h[3].wrapping_add(carry);
        carry = h[3] >> 26;
        h[3] &= LIMB_MASK;
        h[4] = h[4].wrapping_add(carry);
        carry = h[4] >> 26;
        h[4] &= LIMB_MASK;
        h[0] = h[0].wrapping_add(carry * 5);
        carry = h[0] >> 26;
        h[0] &= LIMB_MASK;
        h[1] = h[1].wrapping_add(carry);
        h[1] &= LIMB_MASK;

        let mut g0 = h[0] + 5;
        carry = g0 >> 26;
        g0 &= LIMB_MASK;
        let mut g1 = h[1] + carry;
        carry = g1 >> 26;
        g1 &= LIMB_MASK;
        let mut g2 = h[2] + carry;
        carry = g2 >> 26;
        g2 &= LIMB_MASK;
        let mut g3 = h[3] + carry;
        carry = g3 >> 26;
        g3 &= LIMB_MASK;
        let g4 = (h[4] + carry) as i64 - (1 << 26);

        let mask = !((g4 >> 63) as u64);
        let f0 = (h[0] & !mask) | (g0 & mask);
        let f1 = (h[1] & !mask) | (g1 & mask);
        let f2 = (h[2] & !mask) | (g2 & mask);
        let f3 = (h[3] & !mask) | (g3 & mask);
        let f4 = (h[4] & !mask) | (((g4 as u64) & LIMB_MASK) & mask);

        let mut acc = f0 as u128;
        acc |= (f1 as u128) << 26;
        acc |= (f2 as u128) << 52;
        acc |= (f3 as u128) << 78;
        acc |= (f4 as u128) << 104;

        let s = u128::from_le_bytes(r.s);
        acc = acc.wrapping_add(s);

        acc.to_le_bytes()
    }

    fn poly1305_mac(poly_key: &[u8; 32], aad: &[u8], ciphertext: &[u8]) -> [u8; 16] {
        let key = Poly1305Key::new(poly_key);
        let mut h = [0u64; 5];
        poly1305_process_padded(&mut h, &key, aad);
        poly1305_process_padded(&mut h, &key, ciphertext);

        let mut len_block = [0u8; 16];
        let aad_len = aad.len() as u64;
        let ct_len = ciphertext.len() as u64;
        len_block[..8].copy_from_slice(&aad_len.to_le_bytes());
        len_block[8..].copy_from_slice(&ct_len.to_le_bytes());
        poly1305_process_blocks(&mut h, &key, &len_block);

        poly1305_finalize(h, &key)
    }

    #[cfg(test)]
    pub(crate) fn test_poly1305_mac(poly_key: &[u8; 32], aad: &[u8], ciphertext: &[u8]) -> [u8; 16] {
        poly1305_mac(poly_key, aad, ciphertext)
    }

    fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0u8;
        for (&x, &y) in a.iter().zip(b.iter()) {
            diff |= x ^ y;
        }
        diff == 0
    }

    /// Implementação do AEAD ChaCha20-Poly1305.
    pub struct ChaCha20Poly1305 {
        key: [u8; 32],
    }

    impl ChaCha20Poly1305 {
        /// Cria nova instância com a chave de 256 bits.
        pub fn new(key: &[u8; 32]) -> Self {
            Self { key: *key }
        }

        fn poly_key(&self, nonce: &[u8; NONCE_SIZE]) -> [u8; 32] {
            let block = chacha20_block(&self.key, 0, nonce);
            let mut poly = [0u8; 32];
            poly.copy_from_slice(&block[..32]);
            poly
        }

        fn chacha20_process(&self, nonce: &[u8; NONCE_SIZE], counter: u32, data: &mut [u8]) {
            chacha20_xor(&self.key, nonce, counter, data);
        }
    }

    impl Aead for ChaCha20Poly1305 {
        fn encrypt(&self, nonce: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
            let nonce: [u8; NONCE_SIZE] = nonce
                .try_into()
                .map_err(|_| Error::new(ErrorKind::InvalidInput, "Nonce precisa ter 12 bytes"))?;

            let mut ciphertext = plaintext.to_vec();
            self.chacha20_process(&nonce, 1, &mut ciphertext);

            let poly_key = self.poly_key(&nonce);
            let tag = poly1305_mac(&poly_key, aad, &ciphertext);

            let mut out = Vec::with_capacity(ciphertext.len() + TAG_SIZE);
            out.extend_from_slice(&ciphertext);
            out.extend_from_slice(&tag);
            Ok(out)
        }

        fn decrypt(&self, nonce: &[u8], ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
            let nonce: [u8; NONCE_SIZE] = nonce
                .try_into()
                .map_err(|_| Error::new(ErrorKind::InvalidInput, "Nonce precisa ter 12 bytes"))?;

            if ciphertext.len() < TAG_SIZE {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Ciphertext precisa conter tag",
                ));
            }

            let (cipher_body, tag) = ciphertext.split_at(ciphertext.len() - TAG_SIZE);
            let poly_key = self.poly_key(&nonce);
            let expected_tag = poly1305_mac(&poly_key, aad, cipher_body);
            if !constant_time_eq(&expected_tag, tag) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Tag de autenticação inválida",
                ));
            }

            let mut plaintext = cipher_body.to_vec();
            self.chacha20_process(&nonce, 1, &mut plaintext);
            Ok(plaintext)
        }

        fn key_size(&self) -> usize {
            32
        }

        fn nonce_size(&self) -> usize {
            NONCE_SIZE
        }

        fn tag_size(&self) -> usize {
            TAG_SIZE
        }
    }

    pub use ChaCha20Poly1305 as Cipher;
}

mod aesgcm {
    //! Implementação de AES-GCM (128/256). Segue SP 800-38D.

    use super::{Aead, Error, ErrorKind, Result};
    use alloc::vec::Vec;
    use core::convert::TryInto;

    const BLOCK_SIZE: usize = 16;
    const TAG_SIZE: usize = 16;
    const NONCE_SIZE: usize = 12;
    const MAX_ROUND_KEYS: usize = 60; // suficiente para AES-256 (4*(14+1)).

    const RCON: [u8; 14] = [
        0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1B, 0x36, 0x6C, 0xD8, 0xAB, 0x4D,
    ];

    const SBOX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];

    #[inline(always)]
    fn sub_word(word: u32) -> u32 {
        let mut bytes = word.to_be_bytes();
        for b in bytes.iter_mut() {
            *b = SBOX[*b as usize];
        }
        u32::from_be_bytes(bytes)
    }

    #[inline(always)]
    fn rot_word(word: u32) -> u32 {
        word.rotate_left(8)
    }

    fn expand_key(key: &[u8]) -> (usize, [u32; MAX_ROUND_KEYS]) {
        let nk = key.len() / 4;
        let nr = match key.len() {
            16 => 10,
            24 => 12,
            32 => 14,
            _ => panic!("Chave AES inválida"),
        };

        let mut round_keys = [0u32; MAX_ROUND_KEYS];
        for (i, chunk) in key.chunks_exact(4).enumerate() {
            round_keys[i] = u32::from_be_bytes(chunk.try_into().unwrap());
        }

        let total_words = 4 * (nr + 1);
        for i in nk..total_words {
            let mut temp = round_keys[i - 1];
            if i % nk == 0 {
                temp = sub_word(rot_word(temp))
                    ^ ((RCON[(i / nk) - 1] as u32) << 24);
            } else if nk > 6 && i % nk == 4 {
                temp = sub_word(temp);
            }
            round_keys[i] = round_keys[i - nk] ^ temp;
        }

        (nr, round_keys)
    }

    fn add_round_key(state: &mut [u8; BLOCK_SIZE], round_keys: &[u32], round: usize) {
        for i in 0..4 {
            let rk = round_keys[round * 4 + i].to_be_bytes();
            state[4 * i] ^= rk[0];
            state[4 * i + 1] ^= rk[1];
            state[4 * i + 2] ^= rk[2];
            state[4 * i + 3] ^= rk[3];
        }
    }

    fn sub_bytes(state: &mut [u8; BLOCK_SIZE]) {
        for byte in state.iter_mut() {
            *byte = SBOX[*byte as usize];
        }
    }

    fn shift_rows(state: &mut [u8; BLOCK_SIZE]) {
        let mut tmp = [0u8; BLOCK_SIZE];
        tmp.copy_from_slice(state);

        state[0] = tmp[0];
        state[1] = tmp[5];
        state[2] = tmp[10];
        state[3] = tmp[15];

        state[4] = tmp[4];
        state[5] = tmp[9];
        state[6] = tmp[14];
        state[7] = tmp[3];

        state[8] = tmp[8];
        state[9] = tmp[13];
        state[10] = tmp[2];
        state[11] = tmp[7];

        state[12] = tmp[12];
        state[13] = tmp[1];
        state[14] = tmp[6];
        state[15] = tmp[11];
    }

    #[inline(always)]
    fn xtime(x: u8) -> u8 {
        if x & 0x80 != 0 {
            (x << 1) ^ 0x1b
        } else {
            x << 1
        }
    }

    fn mix_columns(state: &mut [u8; BLOCK_SIZE]) {
        for c in 0..4 {
            let col = &mut state[c * 4..(c + 1) * 4];
            let t = col[0] ^ col[1] ^ col[2] ^ col[3];
            let tmp = col[0];
            col[0] ^= t ^ xtime(col[0] ^ col[1]);
            col[1] ^= t ^ xtime(col[1] ^ col[2]);
            col[2] ^= t ^ xtime(col[2] ^ col[3]);
            col[3] ^= t ^ xtime(col[3] ^ tmp);
        }
    }

    fn encrypt_block(round_keys: &[u32], nr: usize, block: &mut [u8; BLOCK_SIZE]) {
        add_round_key(block, round_keys, 0);
        for round in 1..nr {
            sub_bytes(block);
            shift_rows(block);
            mix_columns(block);
            add_round_key(block, round_keys, round);
        }
        sub_bytes(block);
        shift_rows(block);
        add_round_key(block, round_keys, nr);
    }

    fn inc32(counter: &mut [u8; BLOCK_SIZE]) {
        let mut value = u32::from_be_bytes(counter[12..16].try_into().unwrap());
        value = value.wrapping_add(1);
        counter[12..16].copy_from_slice(&value.to_be_bytes());
    }

    const GF_POLY: u128 = 0xe100_0000_0000_0000_0000_0000_0000_0000;

    fn gf_mul(mut x: u128, mut y: u128) -> u128 {
        let mut z = 0u128;
        for _ in 0..128 {
            if (x & 0x8000_0000_0000_0000_0000_0000_0000_0000) != 0 {
                z ^= y;
            }
            if (y & 1) != 0 {
                y = (y >> 1) ^ GF_POLY;
            } else {
                y >>= 1;
            }
            x <<= 1;
        }
        z
    }

    fn ghash(h: &[u8; 16], aad: &[u8], data: &[u8]) -> [u8; 16] {
        let mut y = 0u128;
        let h128 = u128::from_be_bytes(*h);

        for chunk in aad.chunks(16) {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            let x = u128::from_be_bytes(block);
            y ^= x;
            y = gf_mul(y, h128);
        }

        for chunk in data.chunks(16) {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            let x = u128::from_be_bytes(block);
            y ^= x;
            y = gf_mul(y, h128);
        }

        let mut len_block = [0u8; 16];
        let aad_bits = (aad.len() as u64).wrapping_mul(8);
        let data_bits = (data.len() as u64).wrapping_mul(8);
        len_block[..8].copy_from_slice(&aad_bits.to_be_bytes());
        len_block[8..].copy_from_slice(&data_bits.to_be_bytes());
        let x = u128::from_be_bytes(len_block);
        y ^= x;
        y = gf_mul(y, h128);

        y.to_be_bytes()
    }

    fn gctr(round_keys: &[u32], nr: usize, mut counter: [u8; 16], input: &[u8]) -> Vec<u8> {
        let mut out = Vec::with_capacity(input.len());
        for chunk in input.chunks(16) {
            let mut keystream = counter;
            encrypt_block(round_keys, nr, &mut keystream);

            for i in 0..chunk.len() {
                out.push(chunk[i] ^ keystream[i]);
            }

            inc32(&mut counter);
        }
        out
    }

    fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut diff = 0u8;
        for (&x, &y) in a.iter().zip(b.iter()) {
            diff |= x ^ y;
        }
        diff == 0
    }

    /// Estrutura que mantém chave expandida e subchave de GHASH.
    pub struct AesGcm {
        round_keys: [u32; MAX_ROUND_KEYS],
        rounds: usize,
        hash_subkey: [u8; 16],
        key_len: usize,
    }

    impl AesGcm {
        /// Instancia AES-128-GCM.
        pub fn new_128(key: &[u8; 16]) -> Self {
            Self::with_key(key)
        }

        /// Instancia AES-256-GCM.
        pub fn new_256(key: &[u8; 32]) -> Self {
            Self::with_key(key)
        }

        fn with_key(key: &[u8]) -> Self {
            let (rounds, round_keys) = expand_key(key);
            let mut hash = [0u8; 16];
            encrypt_block(&round_keys, rounds, &mut hash);
            Self {
                round_keys,
                rounds,
                hash_subkey: hash,
                key_len: key.len(),
            }
        }
    }

    impl Aead for AesGcm {
        fn encrypt(&self, nonce: &[u8], plaintext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
            let nonce: [u8; NONCE_SIZE] = nonce
                .try_into()
                .map_err(|_| Error::new(ErrorKind::InvalidInput, "Nonce precisa ter 12 bytes"))?;

            let mut j0 = [0u8; 16];
            j0[..12].copy_from_slice(&nonce);
            j0[15] = 1;

            let mut icb = j0;
            inc32(&mut icb);

            let ciphertext = gctr(&self.round_keys, self.rounds, icb, plaintext);

            let s = ghash(&self.hash_subkey, aad, &ciphertext);
            let mut tag_block = j0;
            encrypt_block(&self.round_keys, self.rounds, &mut tag_block);
            let mut tag = [0u8; TAG_SIZE];
            for i in 0..TAG_SIZE {
                tag[i] = s[i] ^ tag_block[i];
            }

            let mut out = Vec::with_capacity(ciphertext.len() + TAG_SIZE);
            out.extend_from_slice(&ciphertext);
            out.extend_from_slice(&tag);
            Ok(out)
        }

        fn decrypt(&self, nonce: &[u8], ciphertext: &[u8], aad: &[u8]) -> Result<Vec<u8>> {
            let nonce: [u8; NONCE_SIZE] = nonce
                .try_into()
                .map_err(|_| Error::new(ErrorKind::InvalidInput, "Nonce precisa ter 12 bytes"))?;

            if ciphertext.len() < TAG_SIZE {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Ciphertext precisa conter tag",
                ));
            }

            let (cipher_body, tag) = ciphertext.split_at(ciphertext.len() - TAG_SIZE);

            let mut j0 = [0u8; 16];
            j0[..12].copy_from_slice(&nonce);
            j0[15] = 1;

            let s = ghash(&self.hash_subkey, aad, cipher_body);
            let mut tag_block = j0;
            encrypt_block(&self.round_keys, self.rounds, &mut tag_block);
            let mut expected_tag = [0u8; TAG_SIZE];
            for i in 0..TAG_SIZE {
                expected_tag[i] = s[i] ^ tag_block[i];
            }

            if !constant_time_eq(&expected_tag, tag) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Tag de autenticação inválida",
                ));
            }

            let mut icb = j0;
            inc32(&mut icb);
            let plaintext = gctr(&self.round_keys, self.rounds, icb, cipher_body);
            Ok(plaintext)
        }

        fn key_size(&self) -> usize {
            self.key_len
        }

        fn nonce_size(&self) -> usize {
            NONCE_SIZE
        }

        fn tag_size(&self) -> usize {
            TAG_SIZE
        }
    }

    pub use AesGcm as Cipher;
}

pub use aesgcm::Cipher as AesGcm;
pub use chacha20poly1305::Cipher as ChaCha20Poly1305;

/// Módulo de conveniência com as estruturas principais.
pub mod prelude {
    pub use crate::{Aead, AesGcm, ChaCha20Poly1305};
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::chacha20poly1305;
    use core::convert::TryInto;

    fn hex_to_bytes(s: &str) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(s.len() / 2);
        let mut iter = s
            .bytes()
            .filter(|b| !b.is_ascii_whitespace())
            .peekable();
        while iter.peek().is_some() {
            let high = iter
                .next()
                .expect("hex string must have even length");
            let low = iter
                .next()
                .expect("hex string must have even length");
            let hi = (high as char).to_digit(16).expect("hex digit") as u8;
            let lo = (low as char).to_digit(16).expect("hex digit") as u8;
            bytes.push((hi << 4) | lo);
        }
        bytes
    }

    #[test]
    fn chacha20_block_zero_vector() {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let block = chacha20poly1305::test_block(&key, 0, &nonce);
        let expected = hex_to_bytes(
            "76b8e0ada0f13d90405d6ae55386bd28bdd219b8a08ded1aa836efcc8b770dc7\
             da41597c5157488d7724e03fb8d84a376a43b8f41518a11cc387b669b2ee6586",
        );
        assert_eq!(&block[..], expected.as_slice());
    }

    #[test]
    fn chacha20poly1305_rfc8439_vector() {
        let key_bytes = hex_to_bytes("808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f");
        let key: [u8; 32] = key_bytes.as_slice().try_into().unwrap();
        let nonce_bytes = hex_to_bytes("070000004041424344454647");
        let aad = hex_to_bytes("50515253c0c1c2c3c4c5c6c7");
        let plaintext = b"Ladies and Gentlemen of the class of '99: If I could offer you only one tip for the future, sunscreen would be it.".to_vec();
        assert_eq!(plaintext.len(), 114);

        let cipher = ChaCha20Poly1305::new(&key);

        let ciphertext = cipher
            .encrypt(&nonce_bytes, &plaintext, &aad)
            .expect("encryption ok");
        let expected = hex_to_bytes("d31a8d34648e60db7b86afbc53ef7ec2a4aded51296e08fea9e2b5a736ee62d63dbea45e8ca9671282fafb69da92728b1a71de0a9e060b2905d6a5b67ecd3b3692ddbd7f2d778b8c9803aee328091b58fab324e4fad675945585808b4831d7bc3ff4def08e4b7a9de576d26586cec64b61161ae10b594f09e26a7e902ecbd0600691");
        assert_eq!(expected.len(), plaintext.len() + 16);

        assert_eq!(ciphertext, expected);

        let decrypted = cipher
            .decrypt(&nonce_bytes, &ciphertext, &aad)
            .expect("decrypt ok");
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn chacha20poly1305_tag_failure() {
        let key = [0u8; 32];
        let nonce = [0u8; 12];
        let cipher = ChaCha20Poly1305::new(&key);
        let mut ct = cipher.encrypt(&nonce, b"data", b"aad").unwrap();
        ct[0] ^= 0x01;
        assert!(cipher.decrypt(&nonce, &ct, b"aad").is_err());
    }

    #[test]
    fn aes_gcm_zero_vector() {
        let key = [0u8; 16];
        let nonce = [0u8; 12];
        let plaintext = [0u8; 0];
        let cipher = AesGcm::new_128(&key);

        let ciphertext = cipher.encrypt(&nonce, &plaintext, &[]).unwrap();
        assert_eq!(ciphertext.len(), 16);
        let expected_tag = hex_to_bytes("58e2fccefa7e3061367f1d57a4e7455a");
        assert_eq!(&ciphertext, &expected_tag);

        let decrypted = cipher.decrypt(&nonce, &ciphertext, &[]).unwrap();
        assert!(decrypted.is_empty());
    }

    #[test]
    fn aes_gcm_known_vector() {
        let key_bytes = hex_to_bytes("00000000000000000000000000000000");
        let key: [u8; 16] = key_bytes.as_slice().try_into().unwrap();
        let nonce = hex_to_bytes("000000000000000000000000");
        let plaintext = hex_to_bytes("00000000000000000000000000000000");
        let expected_cipher = hex_to_bytes("0388dace60b6a392f328c2b971b2fe78ab6e47d42cec13bdf53a67b21257bddf");

        let cipher = AesGcm::new_128(&key);
        let ciphertext = cipher
            .encrypt(&nonce, &plaintext, &[])
            .expect("encrypt");
        assert_eq!(ciphertext, expected_cipher);

        let decrypted = cipher
            .decrypt(&nonce, &ciphertext, &[])
            .expect("decrypt");
        assert_eq!(decrypted, plaintext);
    }
}
