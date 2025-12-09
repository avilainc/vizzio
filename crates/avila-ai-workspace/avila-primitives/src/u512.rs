//! Inteiro de 512 bits unsigned

/// Inteiro de 512 bits (8 limbs de 64 bits)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(align(64))]
pub struct U512 {
    /// Limbs em ordem little-endian (64 bits cada)
    pub limbs: [u64; 8],
}

impl U512 {
    /// Número de limbs de 64 bits
    pub const LIMBS: usize = 8;
    /// Quantidade de bits representados
    pub const BITS: usize = 512;
    /// Tamanho em bytes
    pub const BYTES: usize = 64;

    /// Valor zero
    pub const ZERO: Self = Self { limbs: [0; 8] };
    /// Valor um
    pub const ONE: Self = Self { limbs: [1, 0, 0, 0, 0, 0, 0, 0] };
    /// Valor máximo (todos os bits em 1)
    pub const MAX: Self = Self { limbs: [u64::MAX; 8] };

    /// Constrói a partir de um `u64`
    pub const fn from_u64(value: u64) -> Self {
        Self { limbs: [value, 0, 0, 0, 0, 0, 0, 0] }
    }

    /// Retorna verdadeiro se todos os limbs forem zero
    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < 8 {
            if self.limbs[i] != 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    /// Retorna verdadeiro se o bit menos significativo estiver setado
    pub const fn is_odd(&self) -> bool {
        (self.limbs[0] & 1) == 1
    }

    /// Constrói um `U512` a partir de bytes big-endian (até 64 bytes)
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= 64);
        let mut limbs = [0u64; 8];
        let mut padded = [0u8; 64];
        padded[64 - bytes.len()..].copy_from_slice(bytes);

        for (i, chunk) in padded.chunks_exact(8).enumerate() {
            limbs[7 - i] = u64::from_be_bytes(chunk.try_into().unwrap());
        }

        Self { limbs }
    }

    /// Converte o inteiro para bytes big-endian (64 bytes)
    pub fn to_bytes_be(&self) -> [u8; 64] {
        let mut bytes = [0u8; 64];
        for (i, &limb) in self.limbs.iter().rev().enumerate() {
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_be_bytes());
        }
        bytes
    }
}
