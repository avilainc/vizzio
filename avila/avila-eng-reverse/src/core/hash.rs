// Implementação simples de hashing sem dependências externas
// Apenas para checksums básicos

pub struct SimpleHash;

impl SimpleHash {
    /// Calcula um hash simples de 32 bytes (256 bits) usando operações bit a bit
    pub fn sha256_simple(data: &[u8]) -> String {
        let mut hash = [0u32; 8];

        // Valores iniciais (primeiros 32 bits das raízes quadradas dos primeiros 8 primos)
        hash[0] = 0x6a09e667;
        hash[1] = 0xbb67ae85;
        hash[2] = 0x3c6ef372;
        hash[3] = 0xa54ff53a;
        hash[4] = 0x510e527f;
        hash[5] = 0x9b05688c;
        hash[6] = 0x1f83d9ab;
        hash[7] = 0x5be0cd19;

        // Processar dados em chunks
        for chunk in data.chunks(64) {
            let mut w = [0u32; 64];

            // Copiar chunk para w[0..16]
            for (i, bytes) in chunk.chunks(4).enumerate() {
                if i >= 16 { break; }
                let mut val = 0u32;
                for (j, &byte) in bytes.iter().enumerate() {
                    val |= (byte as u32) << (24 - j * 8);
                }
                w[i] = val;
            }

            // Estender para 64 palavras
            for i in 16..64 {
                let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
                let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
                w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
            }

            // Processar
            let mut a = hash[0];
            let mut b = hash[1];
            let mut c = hash[2];
            let mut d = hash[3];
            let mut e = hash[4];
            let mut f = hash[5];
            let mut g = hash[6];
            let mut h = hash[7];

            for i in 0..64 {
                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ ((!e) & g);
                let temp1 = h.wrapping_add(s1).wrapping_add(ch).wrapping_add(Self::k(i)).wrapping_add(w[i]);
                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let maj = (a & b) ^ (a & c) ^ (b & c);
                let temp2 = s0.wrapping_add(maj);

                h = g;
                g = f;
                f = e;
                e = d.wrapping_add(temp1);
                d = c;
                c = b;
                b = a;
                a = temp1.wrapping_add(temp2);
            }

            hash[0] = hash[0].wrapping_add(a);
            hash[1] = hash[1].wrapping_add(b);
            hash[2] = hash[2].wrapping_add(c);
            hash[3] = hash[3].wrapping_add(d);
            hash[4] = hash[4].wrapping_add(e);
            hash[5] = hash[5].wrapping_add(f);
            hash[6] = hash[6].wrapping_add(g);
            hash[7] = hash[7].wrapping_add(h);
        }

        // Converter para string hexadecimal
        format!("{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
            hash[0], hash[1], hash[2], hash[3], hash[4], hash[5], hash[6], hash[7])
    }

    /// Calcula SHA-1 simplificado (20 bytes)
    pub fn sha1_simple(data: &[u8]) -> String {
        let mut h0 = 0x67452301u32;
        let mut h1 = 0xEFCDAB89u32;
        let mut h2 = 0x98BADCFEu32;
        let mut h3 = 0x10325476u32;
        let mut h4 = 0xC3D2E1F0u32;

        for chunk in data.chunks(64) {
            let mut w = [0u32; 80];

            // Processar chunk
            for (i, bytes) in chunk.chunks(4).enumerate() {
                if i >= 16 { break; }
                let mut val = 0u32;
                for (j, &byte) in bytes.iter().enumerate() {
                    val |= (byte as u32) << (24 - j * 8);
                }
                w[i] = val;
            }

            // Estender
            for i in 16..80 {
                w[i] = (w[i-3] ^ w[i-8] ^ w[i-14] ^ w[i-16]).rotate_left(1);
            }

            let mut a = h0;
            let mut b = h1;
            let mut c = h2;
            let mut d = h3;
            let mut e = h4;

            for i in 0..80 {
                let (f, k) = if i < 20 {
                    ((b & c) | ((!b) & d), 0x5A827999)
                } else if i < 40 {
                    (b ^ c ^ d, 0x6ED9EBA1)
                } else if i < 60 {
                    ((b & c) | (b & d) | (c & d), 0x8F1BBCDC)
                } else {
                    (b ^ c ^ d, 0xCA62C1D6)
                };

                let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(w[i]);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }

            h0 = h0.wrapping_add(a);
            h1 = h1.wrapping_add(b);
            h2 = h2.wrapping_add(c);
            h3 = h3.wrapping_add(d);
            h4 = h4.wrapping_add(e);
        }

        format!("{:08x}{:08x}{:08x}{:08x}{:08x}", h0, h1, h2, h3, h4)
    }

    /// Calcula MD5 simplificado (16 bytes)
    pub fn md5_simple(data: &[u8]) -> String {
        let mut a0 = 0x67452301u32;
        let mut b0 = 0xefcdab89u32;
        let mut c0 = 0x98badcfeu32;
        let mut d0 = 0x10325476u32;

        for chunk in data.chunks(64) {
            let mut m = [0u32; 16];

            for (i, bytes) in chunk.chunks(4).enumerate() {
                if i >= 16 { break; }
                let mut val = 0u32;
                for (j, &byte) in bytes.iter().enumerate() {
                    val |= (byte as u32) << (j * 8);
                }
                m[i] = val;
            }

            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;

            for i in 0..64 {
                let (f, g) = if i < 16 {
                    ((b & c) | ((!b) & d), i)
                } else if i < 32 {
                    ((d & b) | ((!d) & c), (5 * i + 1) % 16)
                } else if i < 48 {
                    (b ^ c ^ d, (3 * i + 5) % 16)
                } else {
                    (c ^ (b | (!d)), (7 * i) % 16)
                };

                let temp = d;
                d = c;
                c = b;
                b = b.wrapping_add(
                    (a.wrapping_add(f).wrapping_add(Self::md5_k(i)).wrapping_add(m[g]))
                    .rotate_left(Self::md5_s(i))
                );
                a = temp;
            }

            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }

        format!("{:08x}{:08x}{:08x}{:08x}", a0, b0, c0, d0)
    }

    // Constantes SHA-256
    fn k(i: usize) -> u32 {
        const K: [u32; 64] = [
            0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
            0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
            0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
            0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
            0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
            0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
            0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
            0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
        ];
        K[i]
    }

    // Constantes MD5
    fn md5_k(i: usize) -> u32 {
        const K: [u32; 64] = [
            0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
            0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
            0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
            0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
            0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
            0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
            0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
            0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
        ];
        K[i]
    }

    fn md5_s(i: usize) -> u32 {
        const S: [u32; 64] = [
            7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
            5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
            4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
            6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,
        ];
        S[i]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_empty() {
        let hash = SimpleHash::sha256_simple(b"");
        assert_eq!(hash.len(), 64); // 32 bytes = 64 caracteres hex
    }

    #[test]
    fn test_sha1_empty() {
        let hash = SimpleHash::sha1_simple(b"");
        assert_eq!(hash.len(), 40); // 20 bytes = 40 caracteres hex
    }

    #[test]
    fn test_md5_empty() {
        let hash = SimpleHash::md5_simple(b"");
        assert_eq!(hash.len(), 32); // 16 bytes = 32 caracteres hex
    }

    #[test]
    fn test_sha256_hello() {
        let hash = SimpleHash::sha256_simple(b"hello");
        println!("SHA256('hello'): {}", hash);
        assert_eq!(hash.len(), 64);
    }
}
