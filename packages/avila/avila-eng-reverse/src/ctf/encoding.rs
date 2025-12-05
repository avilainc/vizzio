use base64::{Engine as _, engine::general_purpose};

pub struct EncodingTools;

impl EncodingTools {
    /// Codificar para Base64
    pub fn to_base64(data: &[u8]) -> String {
        general_purpose::STANDARD.encode(data)
    }

    /// Decodificar de Base64
    pub fn from_base64(encoded: &str) -> Result<Vec<u8>, String> {
        general_purpose::STANDARD.decode(encoded)
            .map_err(|e| format!("Erro ao decodificar Base64: {}", e))
    }

    /// Codificar para Hex
    pub fn to_hex(data: &[u8]) -> String {
        hex::encode(data)
    }

    /// Decodificar de Hex
    pub fn from_hex(encoded: &str) -> Result<Vec<u8>, String> {
        hex::decode(encoded)
            .map_err(|e| format!("Erro ao decodificar Hex: {}", e))
    }

    /// URL encode
    pub fn url_encode(text: &str) -> String {
        text.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' {
                    c.to_string()
                } else {
                    format!("%{:02X}", c as u8)
                }
            })
            .collect()
    }

    /// URL decode
    pub fn url_decode(encoded: &str) -> Result<String, String> {
        let mut decoded = String::new();
        let mut chars = encoded.chars();

        while let Some(c) = chars.next() {
            if c == '%' {
                let hex: String = chars.by_ref().take(2).collect();
                if hex.len() == 2 {
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        decoded.push(byte as char);
                    } else {
                        return Err("Hex inválido".to_string());
                    }
                } else {
                    return Err("Formato inválido".to_string());
                }
            } else if c == '+' {
                decoded.push(' ');
            } else {
                decoded.push(c);
            }
        }

        Ok(decoded)
    }

    /// Binário para texto
    pub fn bin_to_text(binary: &str) -> Result<String, String> {
        let binary = binary.replace(' ', "");
        let mut text = String::new();

        for chunk in binary.as_bytes().chunks(8) {
            let byte_str = std::str::from_utf8(chunk)
                .map_err(|_| "UTF-8 inválido")?;
            let byte = u8::from_str_radix(byte_str, 2)
                .map_err(|_| "Binário inválido")?;
            text.push(byte as char);
        }

        Ok(text)
    }

    /// Texto para binário
    pub fn text_to_bin(text: &str) -> String {
        text.bytes()
            .map(|b| format!("{:08b}", b))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Decimal para várias bases
    pub fn dec_to_base(num: u64, base: u32) -> String {
        if base < 2 || base > 36 {
            return "Base inválida".to_string();
        }

        if num == 0 {
            return "0".to_string();
        }

        let mut result = String::new();
        let mut n = num;
        let digits = "0123456789abcdefghijklmnopqrstuvwxyz";

        while n > 0 {
            let digit = (n % base as u64) as usize;
            result.insert(0, digits.chars().nth(digit).unwrap());
            n /= base as u64;
        }

        result
    }

    /// Várias bases para decimal
    pub fn base_to_dec(num: &str, base: u32) -> Result<u64, String> {
        u64::from_str_radix(num, base)
            .map_err(|e| format!("Erro ao converter: {}", e))
    }

    /// Detecção automática de encoding
    pub fn detect_encoding(data: &str) -> Vec<&'static str> {
        let mut encodings = Vec::new();

        // Base64
        if general_purpose::STANDARD.decode(data).is_ok() {
            encodings.push("Base64");
        }

        // Hex
        if hex::decode(data).is_ok() {
            encodings.push("Hexadecimal");
        }

        // Binary
        if data.chars().all(|c| c == '0' || c == '1' || c.is_whitespace()) {
            encodings.push("Binary");
        }

        // URL encoded
        if data.contains('%') {
            encodings.push("URL Encoded");
        }

        if encodings.is_empty() {
            encodings.push("Plain Text");
        }

        encodings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64() {
        let data = b"Hello, World!";
        let encoded = EncodingTools::to_base64(data);
        let decoded = EncodingTools::from_base64(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
    }

    #[test]
    fn test_hex() {
        let data = b"Test";
        let encoded = EncodingTools::to_hex(data);
        let decoded = EncodingTools::from_hex(&encoded).unwrap();
        assert_eq!(data, decoded.as_slice());
    }
}
