use sha2::{Sha256, Sha512, Digest};
// MD5 usando md5 crate v0.7
// AES temporariamente desabilitado - será implementado internamente

pub struct CryptoTools;

impl CryptoTools {
    /// Calcula hash MD5
    pub fn md5(data: &[u8]) -> String {
        use md5::Digest;
        let mut hasher = md5::Md5::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Calcula hash SHA-256
    pub fn sha256(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// Calcula hash SHA-512
    pub fn sha512(data: &[u8]) -> String {
        let mut hasher = Sha512::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    /// XOR com chave
    pub fn xor(data: &[u8], key: &[u8]) -> Vec<u8> {
        if key.is_empty() {
            return data.to_vec();
        }

        data.iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ key[i % key.len()])
            .collect()
    }

    /// Rotação de bytes (ROT-N)
    pub fn rot(data: &[u8], n: u8) -> Vec<u8> {
        data.iter()
            .map(|&byte| byte.wrapping_add(n))
            .collect()
    }

    /// ROT13 para texto
    pub fn rot13(text: &str) -> String {
        text.chars()
            .map(|c| {
                match c {
                    'a'..='m' | 'A'..='M' => ((c as u8) + 13) as char,
                    'n'..='z' | 'N'..='Z' => ((c as u8) - 13) as char,
                    _ => c,
                }
            })
            .collect()
    }

    /// Cifra de César
    pub fn caesar(text: &str, shift: i32) -> String {
        text.chars()
            .map(|c| {
                if c.is_ascii_alphabetic() {
                    let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                    let shifted = (((c as u8 - base) as i32 + shift).rem_euclid(26)) as u8;
                    (base + shifted) as char
                } else {
                    c
                }
            })
            .collect()
    }

    /// Análise de frequência de caracteres
    pub fn frequency_analysis(data: &[u8]) -> Vec<(u8, usize)> {
        let mut freq = [0usize; 256];

        for &byte in data {
            freq[byte as usize] += 1;
        }

        let mut result: Vec<(u8, usize)> = freq.iter()
            .enumerate()
            .filter(|(_, &count)| count > 0)
            .map(|(byte, &count)| (byte as u8, count))
            .collect();

        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }

    /// Brute force XOR single-byte key
    pub fn xor_bruteforce(ciphertext: &[u8]) -> Vec<(u8, String, f64)> {
        let mut results = Vec::new();

        for key in 0..=255 {
            let decrypted: Vec<u8> = ciphertext.iter()
                .map(|&byte| byte ^ key)
                .collect();

            // Verificar se é texto ASCII válido
            if decrypted.iter().all(|&b| b >= 32 && b <= 126 || b == 10 || b == 13) {
                if let Ok(text) = String::from_utf8(decrypted.clone()) {
                    let score = Self::english_score(&text);
                    if score > 0.5 {
                        results.push((key, text, score));
                    }
                }
            }
        }

        results.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        results
    }

    /// Score baseado em frequência de letras em inglês
    fn english_score(text: &str) -> f64 {
        let freq = "ETAOIN SHRDLU";
        let text_upper = text.to_uppercase();

        let mut score = 0.0;
        let total = text.len() as f64;

        for c in freq.chars() {
            let count = text_upper.matches(c).count() as f64;
            score += count / total;
        }

        score
    }

    /// Detectar tipo de hash
    pub fn identify_hash(hash: &str) -> Vec<&'static str> {
        let mut types = Vec::new();
        let len = hash.len();

        match len {
            32 => types.push("MD5"),
            40 => types.push("SHA-1"),
            56 => types.push("SHA-224"),
            64 => types.push("SHA-256"),
            96 => types.push("SHA-384"),
            128 => types.push("SHA-512"),
            _ => types.push("Unknown"),
        }

        types
    }
}
