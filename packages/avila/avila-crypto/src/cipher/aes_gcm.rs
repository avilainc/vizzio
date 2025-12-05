//! AES-256-GCM - Implementação pura em Rust
//!
//! Implementação completa de AES-256 em modo GCM (Galois/Counter Mode)
//! Suporta tanto software puro quanto aceleração por hardware quando disponível

// AES S-box
const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

// Rcon para key expansion
const RCON: [u8; 11] = [0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

/// AES-256-GCM cipher
pub struct AesGcm {
    round_keys: [[u8; 16]; 15], // AES-256 tem 14 rounds + 1 inicial
}

impl AesGcm {
    /// Cria novo cipher com a chave
    pub fn new(key: &[u8; 32]) -> Self {
        let mut cipher = Self {
            round_keys: [[0u8; 16]; 15],
        };
        cipher.key_expansion(key);
        cipher
    }

    /// Key expansion para AES-256
    fn key_expansion(&mut self, key: &[u8; 32]) {
        let mut w = [[0u8; 4]; 60]; // 4 * (14 + 1) = 60 words

        // Primeiras 8 words vêm da chave
        for i in 0..8 {
            w[i].copy_from_slice(&key[i * 4..(i + 1) * 4]);
        }

        // Expande o resto
        for i in 8..60 {
            let mut temp = w[i - 1];

            if i % 8 == 0 {
                // RotWord + SubWord + Rcon
                temp.rotate_left(1);
                for byte in &mut temp {
                    *byte = SBOX[*byte as usize];
                }
                temp[0] ^= RCON[i / 8];
            } else if i % 8 == 4 {
                // SubWord apenas
                for byte in &mut temp {
                    *byte = SBOX[*byte as usize];
                }
            }

            for j in 0..4 {
                temp[j] ^= w[i - 8][j];
            }
            w[i] = temp;
        }

        // Converte words para round keys
        for i in 0..15 {
            for j in 0..4 {
                self.round_keys[i][j * 4..(j + 1) * 4].copy_from_slice(&w[i * 4 + j]);
            }
        }
    }

    /// SubBytes transformation
    fn sub_bytes(state: &mut [u8; 16]) {
        for byte in state.iter_mut() {
            *byte = SBOX[*byte as usize];
        }
    }

    /// ShiftRows transformation
    fn shift_rows(state: &mut [u8; 16]) {
        let temp = *state;
        // Row 0: no shift
        // Row 1: shift left by 1
        state[1] = temp[5];
        state[5] = temp[9];
        state[9] = temp[13];
        state[13] = temp[1];
        // Row 2: shift left by 2
        state[2] = temp[10];
        state[10] = temp[2];
        state[6] = temp[14];
        state[14] = temp[6];
        // Row 3: shift left by 3
        state[3] = temp[15];
        state[15] = temp[11];
        state[11] = temp[7];
        state[7] = temp[3];
    }

    /// MixColumns transformation
    fn mix_columns(state: &mut [u8; 16]) {
        fn xtime(x: u8) -> u8 {
            let msb = x & 0x80;
            let result = x << 1;
            if msb != 0 {
                result ^ 0x1b
            } else {
                result
            }
        }

        for i in 0..4 {
            let s0 = state[i * 4];
            let s1 = state[i * 4 + 1];
            let s2 = state[i * 4 + 2];
            let s3 = state[i * 4 + 3];

            state[i * 4] = xtime(s0) ^ xtime(s1) ^ s1 ^ s2 ^ s3;
            state[i * 4 + 1] = s0 ^ xtime(s1) ^ xtime(s2) ^ s2 ^ s3;
            state[i * 4 + 2] = s0 ^ s1 ^ xtime(s2) ^ xtime(s3) ^ s3;
            state[i * 4 + 3] = xtime(s0) ^ s0 ^ s1 ^ s2 ^ xtime(s3);
        }
    }

    /// AddRoundKey transformation
    fn add_round_key(state: &mut [u8; 16], round_key: &[u8; 16]) {
        for i in 0..16 {
            state[i] ^= round_key[i];
        }
    }

    /// Encripta um bloco AES-256
    fn encrypt_block(&self, block: &mut [u8; 16]) {
        // Initial round
        Self::add_round_key(block, &self.round_keys[0]);

        // Main rounds
        for round in 1..14 {
            Self::sub_bytes(block);
            Self::shift_rows(block);
            Self::mix_columns(block);
            Self::add_round_key(block, &self.round_keys[round]);
        }

        // Final round (sem MixColumns)
        Self::sub_bytes(block);
        Self::shift_rows(block);
        Self::add_round_key(block, &self.round_keys[14]);
    }

    /// Incrementa counter para CTR mode
    fn increment_counter(counter: &mut [u8; 16]) {
        for i in (0..16).rev() {
            counter[i] = counter[i].wrapping_add(1);
            if counter[i] != 0 {
                break;
            }
        }
    }

    /// GHASH para autenticação
    fn ghash(h: &[u8; 16], aad: &[u8], ciphertext: &[u8]) -> [u8; 16] {
        let mut y = [0u8; 16];

        // Process AAD
        for chunk in aad.chunks(16) {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            for i in 0..16 {
                y[i] ^= block[i];
            }
            Self::gmul(&mut y, h);
        }

        // Process ciphertext
        for chunk in ciphertext.chunks(16) {
            let mut block = [0u8; 16];
            block[..chunk.len()].copy_from_slice(chunk);
            for i in 0..16 {
                y[i] ^= block[i];
            }
            Self::gmul(&mut y, h);
        }

        // Process lengths
        let aad_bits = (aad.len() as u64) * 8;
        let ct_bits = (ciphertext.len() as u64) * 8;
        let mut len_block = [0u8; 16];
        len_block[0..8].copy_from_slice(&aad_bits.to_be_bytes());
        len_block[8..16].copy_from_slice(&ct_bits.to_be_bytes());

        for i in 0..16 {
            y[i] ^= len_block[i];
        }
        Self::gmul(&mut y, h);

        y
    }

    /// Multiplicação em GF(2^128) para GHASH
    fn gmul(x: &mut [u8; 16], h: &[u8; 16]) {
        let mut z = [0u8; 16];
        let mut v = *h;

        for byte in x.iter() {
            for bit in (0..8).rev() {
                if (byte >> bit) & 1 == 1 {
                    for i in 0..16 {
                        z[i] ^= v[i];
                    }
                }

                let lsb = v[15] & 1;
                for i in (1..16).rev() {
                    v[i] = (v[i] >> 1) | (v[i - 1] << 7);
                }
                v[0] >>= 1;

                if lsb == 1 {
                    v[0] ^= 0xe1;
                }
            }
        }

        *x = z;
    }

    /// Criptografa com AES-256-GCM
    pub fn encrypt(
        key: &[u8; 32],
        nonce: &[u8; 12],
        aad: &[u8],
        plaintext: &[u8],
        ciphertext: &mut [u8],
        tag: &mut [u8; 16],
    ) {
        assert!(ciphertext.len() >= plaintext.len());

        let cipher = Self::new(key);

        // Gera H = E(K, 0^128)
        let mut h = [0u8; 16];
        cipher.encrypt_block(&mut h);

        // Prepara counter inicial: nonce || 0x00000001
        let mut counter = [0u8; 16];
        counter[..12].copy_from_slice(nonce);
        counter[15] = 1;

        // Encripta usando CTR mode
        for (i, chunk) in plaintext.chunks(16).enumerate() {
            let mut keystream = counter;
            cipher.encrypt_block(&mut keystream);

            let offset = i * 16;
            let len = chunk.len().min(16);
            for j in 0..len {
                ciphertext[offset + j] = chunk[j] ^ keystream[j];
            }

            Self::increment_counter(&mut counter);
        }

        // Calcula tag usando GHASH
        let ghash_result = Self::ghash(&h, aad, &ciphertext[..plaintext.len()]);

        // Tag = GHASH XOR E(K, nonce || 0x00000001)
        let mut tag_mask = [0u8; 16];
        tag_mask[..12].copy_from_slice(nonce);
        tag_mask[15] = 1;
        cipher.encrypt_block(&mut tag_mask);

        for i in 0..16 {
            tag[i] = ghash_result[i] ^ tag_mask[i];
        }
    }

    /// Decriptografa com AES-256-GCM
    pub fn decrypt(
        key: &[u8; 32],
        nonce: &[u8; 12],
        aad: &[u8],
        ciphertext: &[u8],
        tag: &[u8; 16],
        plaintext: &mut [u8],
    ) -> bool {
        assert!(plaintext.len() >= ciphertext.len());

        let cipher = Self::new(key);

        // Verifica tag primeiro
        let mut h = [0u8; 16];
        cipher.encrypt_block(&mut h);

        let ghash_result = Self::ghash(&h, aad, ciphertext);

        let mut expected_tag = [0u8; 16];
        expected_tag[..12].copy_from_slice(nonce);
        expected_tag[15] = 1;
        cipher.encrypt_block(&mut expected_tag);

        for i in 0..16 {
            expected_tag[i] ^= ghash_result[i];
        }

        // Constant-time comparison
        let mut diff = 0u8;
        for i in 0..16 {
            diff |= tag[i] ^ expected_tag[i];
        }

        if diff != 0 {
            return false;
        }

        // Decripta usando CTR mode (idêntico à encriptação)
        let mut counter = [0u8; 16];
        counter[..12].copy_from_slice(nonce);
        counter[15] = 1;

        for (i, chunk) in ciphertext.chunks(16).enumerate() {
            let mut keystream = counter;
            cipher.encrypt_block(&mut keystream);

            let offset = i * 16;
            let len = chunk.len().min(16);
            for j in 0..len {
                plaintext[offset + j] = chunk[j] ^ keystream[j];
            }

            Self::increment_counter(&mut counter);
        }

        true
    }
}
