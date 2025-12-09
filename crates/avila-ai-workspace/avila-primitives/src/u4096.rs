//! Inteiro de 4096 bits unsigned (RSA-4096)

/// Inteiro de 4096 bits (64 limbs de 64 bits)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(align(64))]
pub struct U4096 {
    /// Limbs em ordem little-endian (64 bits cada)
    pub limbs: [u64; 64],
}

impl U4096 {
    /// Quantidade de limbs de 64 bits
    pub const LIMBS: usize = 64;
    /// Quantidade total de bits representados
    pub const BITS: usize = 4096;
    /// Tamanho em bytes
    pub const BYTES: usize = 512;

    /// Valor zero (todos os limbs iguais a zero)
    pub const ZERO: Self = Self { limbs: [0; 64] };
    /// Valor um (limb menos significativo igual a 1)
    pub const ONE: Self = {
        let mut limbs = [0; 64];
        limbs[0] = 1;
        Self { limbs }
    };

    /// Constrói a partir de um valor `u64`
    pub const fn from_u64(value: u64) -> Self {
        let mut limbs = [0; 64];
        limbs[0] = value;
        Self { limbs }
    }

    /// Retorna verdadeiro se todos os limbs forem zero
    pub const fn is_zero(&self) -> bool {
        let mut i = 0;
        while i < 64 {
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

    /// Constroi um `U4096` a partir de bytes big-endian (até 512 bytes)
    pub fn from_bytes_be(bytes: &[u8]) -> Self {
        assert!(bytes.len() <= 512);
        let mut limbs = [0u64; 64];
        let mut padded = [0u8; 512];
        padded[512 - bytes.len()..].copy_from_slice(bytes);

        for (i, chunk) in padded.chunks_exact(8).enumerate() {
            limbs[63 - i] = u64::from_be_bytes(chunk.try_into().unwrap());
        }

        Self { limbs }
    }

    /// Converte o inteiro para bytes big-endian (512 bytes)
    pub fn to_bytes_be(&self) -> [u8; 512] {
        let mut bytes = [0u8; 512];
        for (i, &limb) in self.limbs.iter().rev().enumerate() {
            bytes[i * 8..(i + 1) * 8].copy_from_slice(&limb.to_be_bytes());
        }
        bytes
    }
}
