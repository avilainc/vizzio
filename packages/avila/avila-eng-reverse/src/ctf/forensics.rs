use std::collections::HashMap;

pub struct ForensicsTools;

impl ForensicsTools {
    /// Extrair strings de um arquivo
    pub fn extract_strings(data: &[u8], min_length: usize) -> Vec<(usize, String)> {
        let mut strings = Vec::new();
        let mut current = String::new();
        let mut start_offset = 0;

        for (offset, &byte) in data.iter().enumerate() {
            if byte >= 32 && byte <= 126 {
                if current.is_empty() {
                    start_offset = offset;
                }
                current.push(byte as char);
            } else {
                if current.len() >= min_length {
                    strings.push((start_offset, current.clone()));
                }
                current.clear();
            }
        }

        if current.len() >= min_length {
            strings.push((start_offset, current));
        }

        strings
    }

    /// Detectar magic bytes (assinatura de arquivo)
    pub fn detect_file_type(data: &[u8]) -> Option<&'static str> {
        if data.len() < 4 {
            return None;
        }

        match &data[..4] {
            [0x4D, 0x5A, _, _] => Some("PE/EXE"),
            [0x7F, 0x45, 0x4C, 0x46] => Some("ELF"),
            [0x89, 0x50, 0x4E, 0x47] => Some("PNG"),
            [0xFF, 0xD8, 0xFF, _] => Some("JPEG"),
            [0x50, 0x4B, 0x03, 0x04] => Some("ZIP"),
            [0x52, 0x61, 0x72, 0x21] => Some("RAR"),
            [0x25, 0x50, 0x44, 0x46] => Some("PDF"),
            _ => {
                // Verificar outros formatos
                if data.len() >= 2 {
                    match &data[..2] {
                        [0x1F, 0x8B] => return Some("GZIP"),
                        [0x42, 0x5A] => return Some("BZIP2"),
                        _ => {}
                    }
                }
                None
            }
        }
    }

    /// Calcular entropia de dados
    pub fn calculate_entropy(data: &[u8]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }

        let mut counts = [0u32; 256];
        for &byte in data {
            counts[byte as usize] += 1;
        }

        let len = data.len() as f64;
        counts.iter()
            .filter(|&&count| count > 0)
            .map(|&count| {
                let p = count as f64 / len;
                -p * p.log2()
            })
            .sum()
    }

    /// Procurar por padrões específicos (URLs, IPs, emails)
    pub fn find_patterns(data: &[u8]) -> HashMap<String, Vec<String>> {
        let mut patterns = HashMap::new();

        let strings = Self::extract_strings(data, 4);
        let all_text = strings.iter()
            .map(|(_, s)| s.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        // URLs
        let urls = Self::extract_urls(&all_text);
        if !urls.is_empty() {
            patterns.insert("URLs".to_string(), urls);
        }

        // IPs
        let ips = Self::extract_ips(&all_text);
        if !ips.is_empty() {
            patterns.insert("IP Addresses".to_string(), ips);
        }

        // Emails
        let emails = Self::extract_emails(&all_text);
        if !emails.is_empty() {
            patterns.insert("Emails".to_string(), emails);
        }

        patterns
    }

    fn extract_urls(text: &str) -> Vec<String> {
        let mut urls = Vec::new();
        for word in text.split_whitespace() {
            if word.starts_with("http://") || word.starts_with("https://") {
                urls.push(word.to_string());
            }
        }
        urls
    }

    fn extract_ips(text: &str) -> Vec<String> {
        let mut ips = Vec::new();
        let parts: Vec<&str> = text.split(|c: char| !c.is_numeric() && c != '.').collect();

        for part in parts {
            let octets: Vec<&str> = part.split('.').collect();
            if octets.len() == 4 && octets.iter().all(|o| o.parse::<u8>().is_ok()) {
                ips.push(part.to_string());
            }
        }
        ips
    }

    fn extract_emails(text: &str) -> Vec<String> {
        let mut emails = Vec::new();
        for word in text.split_whitespace() {
            if word.contains('@') && word.contains('.') {
                emails.push(word.to_string());
            }
        }
        emails
    }

    /// Hex dump formatado
    pub fn hex_dump(data: &[u8], offset: usize, length: usize) -> String {
        let mut result = String::new();
        let end = (offset + length).min(data.len());

        for i in (offset..end).step_by(16) {
            result.push_str(&format!("{:08x}: ", i));

            // Hex bytes
            for j in 0..16 {
                if i + j < end {
                    result.push_str(&format!("{:02x} ", data[i + j]));
                } else {
                    result.push_str("   ");
                }
            }

            result.push_str(" |");

            // ASCII representation
            for j in 0..16 {
                if i + j < end {
                    let byte = data[i + j];
                    if byte >= 32 && byte <= 126 {
                        result.push(byte as char);
                    } else {
                        result.push('.');
                    }
                }
            }

            result.push_str("|\n");
        }

        result
    }

    /// Carregar arquivo em memória
    pub fn carve_files(data: &[u8]) -> Vec<(usize, &'static str)> {
        let mut carved = Vec::new();

        for i in 0..data.len().saturating_sub(4) {
            if let Some(file_type) = Self::detect_file_type(&data[i..]) {
                carved.push((i, file_type));
            }
        }

        carved
    }
}
