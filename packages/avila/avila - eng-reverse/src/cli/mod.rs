mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "deriax")]
#[command(author = "Deriax Team")]
#[command(version = "0.1.0")]
#[command(about = "🔬 Deriax - Engenharia Reversa Avançada", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Análise completa de binário
    Analyze {
        /// Caminho do arquivo para analisar
        #[arg(short, long)]
        file: String,

        /// Exportar relatório em JSON
        #[arg(short, long)]
        json: bool,

        /// Análise profunda (mais lenta)
        #[arg(short, long)]
        deep: bool,
    },

    /// Detecção de malware
    Malware {
        /// Caminho do arquivo
        #[arg(short, long)]
        file: String,

        /// Atualizar assinaturas antes de escanear
        #[arg(short, long)]
        update: bool,
    },

    /// Scanner de vulnerabilidades
    Vuln {
        /// Caminho do arquivo
        #[arg(short, long)]
        file: String,

        /// Procurar ROP gadgets
        #[arg(short, long)]
        rop: bool,

        /// Máximo de instruções por gadget
        #[arg(long, default_value = "5")]
        max_gadgets: usize,
    },

    /// Desassembler
    Disasm {
        /// Caminho do arquivo
        #[arg(short, long)]
        file: String,

        /// Endereço inicial (hex)
        #[arg(short, long)]
        address: Option<String>,

        /// Número de instruções
        #[arg(short, long, default_value = "50")]
        count: usize,
    },

    /// Ferramentas de CTF
    Ctf {
        #[command(subcommand)]
        tool: CtfTools,
    },

    /// Extração de strings
    Strings {
        /// Caminho do arquivo
        #[arg(short, long)]
        file: String,

        /// Tamanho mínimo
        #[arg(short, long, default_value = "4")]
        min: usize,
    },

    /// Calcular hashes
    Hash {
        /// Caminho do arquivo
        #[arg(short, long)]
        file: String,
    },
}

#[derive(Subcommand)]
pub enum CtfTools {
    /// Ferramentas de criptografia
    Crypto {
        #[command(subcommand)]
        action: CryptoActions,
    },

    /// Ferramentas de pwn
    Pwn {
        #[command(subcommand)]
        action: PwnActions,
    },

    /// Ferramentas de encoding
    Encode {
        #[command(subcommand)]
        action: EncodingActions,
    },

    /// Ferramentas de forense
    Forensics {
        #[command(subcommand)]
        action: ForensicsActions,
    },
}

#[derive(Subcommand)]
pub enum CryptoActions {
    /// Calcular hash
    Hash {
        #[arg(short, long)]
        data: String,

        #[arg(short, long, default_value = "sha256")]
        algorithm: String,
    },

    /// XOR com chave
    Xor {
        #[arg(short, long)]
        data: String,

        #[arg(short, long)]
        key: String,
    },

    /// ROT13
    Rot13 {
        #[arg(short, long)]
        text: String,
    },

    /// Brute force XOR
    XorBrute {
        #[arg(short, long)]
        file: String,
    },

    /// Análise de frequência
    Frequency {
        #[arg(short, long)]
        file: String,
    },
}

#[derive(Subcommand)]
pub enum PwnActions {
    /// Gerar padrão cíclico
    Cyclic {
        #[arg(short, long)]
        length: usize,
    },

    /// Encontrar offset
    CyclicFind {
        #[arg(short, long)]
        pattern_file: String,

        #[arg(short, long)]
        target: String,
    },

    /// Converter para little-endian
    Pack {
        #[arg(short, long)]
        value: String,

        #[arg(short = 'b', long, default_value = "64")]
        bits: u8,
    },

    /// Shellcode
    Shellcode {
        #[arg(short, long, default_value = "linux_x64")]
        platform: String,
    },
}

#[derive(Subcommand)]
pub enum EncodingActions {
    /// Base64 encode
    B64Encode {
        #[arg(short, long)]
        data: String,
    },

    /// Base64 decode
    B64Decode {
        #[arg(short, long)]
        data: String,
    },

    /// Hex encode
    HexEncode {
        #[arg(short, long)]
        data: String,
    },

    /// Hex decode
    HexDecode {
        #[arg(short, long)]
        data: String,
    },

    /// URL encode
    UrlEncode {
        #[arg(short, long)]
        data: String,
    },

    /// URL decode
    UrlDecode {
        #[arg(short, long)]
        data: String,
    },

    /// Detectar encoding
    Detect {
        #[arg(short, long)]
        data: String,
    },
}

#[derive(Subcommand)]
pub enum ForensicsActions {
    /// Detectar tipo de arquivo
    FileType {
        #[arg(short, long)]
        file: String,
    },

    /// Hex dump
    HexDump {
        #[arg(short, long)]
        file: String,

        #[arg(short, long, default_value = "0")]
        offset: usize,

        #[arg(short, long, default_value = "256")]
        length: usize,
    },

    /// Extrair strings
    Strings {
        #[arg(short, long)]
        file: String,

        #[arg(short, long, default_value = "4")]
        min_length: usize,
    },

    /// Calcular entropia
    Entropy {
        #[arg(short, long)]
        file: String,
    },
}

pub async fn execute(cli: Cli) -> Result<()> {
    commands::handle_command(cli).await
}
