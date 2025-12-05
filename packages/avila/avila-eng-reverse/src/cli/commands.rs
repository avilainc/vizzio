use super::{Cli, Commands, CtfTools, CryptoActions, PwnActions, EncodingActions, ForensicsActions};
use crate::core::Binary;
use crate::analysis::BinaryAnalyzer;
use crate::malware::MalwareDetector;
use crate::vuln::{VulnerabilityScanner, RopGadgetFinder};
use crate::ctf::{CryptoTools, PwnTools, EncodingTools, ForensicsTools};
use anyhow::Result;
use colored::Colorize;
use std::fs;

pub async fn handle_command(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Analyze { file, json, deep } => {
            analyze_binary(&file, json, deep).await?;
        }
        Commands::Malware { file, update } => {
            detect_malware(&file, update).await?;
        }
        Commands::Vuln { file, rop, max_gadgets } => {
            scan_vulnerabilities(&file, rop, max_gadgets).await?;
        }
        Commands::Disasm { file, address, count } => {
            disassemble_binary(&file, address, count).await?;
        }
        Commands::Ctf { tool } => {
            handle_ctf_tool(tool).await?;
        }
        Commands::Strings { file, min } => {
            extract_strings(&file, min).await?;
        }
        Commands::Hash { file } => {
            calculate_hashes(&file).await?;
        }
    }
    Ok(())
}

async fn analyze_binary(path: &str, json_output: bool, deep: bool) -> Result<()> {
    println!("{}", "🔍 Analisando binário...".cyan());

    let binary = Binary::from_file(path)?;

    println!("\n{}", "📊 Informações Básicas:".green().bold());
    println!("  Formato: {}", binary.format);
    println!("  Arquitetura: {}", binary.arch.name());
    println!("  Entry Point: 0x{:x}", binary.entry_point);
    println!("  Image Base: 0x{:x}", binary.image_base);
    println!("  Tamanho: {} bytes", binary.data.len());

    println!("\n{}", "🔐 Hashes:".green().bold());
    println!("  MD5:    {}", binary.hashes.md5);
    println!("  SHA-1:  {}", binary.hashes.sha1);
    println!("  SHA-256: {}", binary.hashes.sha256);

    println!("\n{}", "📦 Seções:".green().bold());
    for section in &binary.sections {
        let entropy_color = if section.entropy > 7.0 {
            "red"
        } else if section.entropy > 6.0 {
            "yellow"
        } else {
            "green"
        };

        println!("  {} - VA: 0x{:08x}, Size: 0x{:08x}, Entropy: {}",
            section.name.cyan(),
            section.virtual_address,
            section.virtual_size,
            format!("{:.2}", section.entropy).color(entropy_color)
        );
    }

    if !binary.imports.is_empty() {
        println!("\n{}", format!("📚 Imports ({} funções):", binary.imports.len()).green().bold());
        let preview = binary.imports.iter().take(10);
        for import in preview {
            println!("  {} -> {}", import.library.yellow(), import.function);
        }
        if binary.imports.len() > 10 {
            println!("  ... e mais {} imports", binary.imports.len() - 10);
        }
    }

    if deep {
        println!("\n{}", "🔬 Análise Profunda...".cyan());
        let report = BinaryAnalyzer::analyze(&binary)?;

        if !report.suspicious_indicators.is_empty() {
            println!("\n{}", "⚠️  Indicadores Suspeitos:".red().bold());
            for indicator in &report.suspicious_indicators {
                println!("  [{}] {} - {}",
                    format!("{:?}", indicator.severity).red(),
                    indicator.category.yellow(),
                    indicator.description
                );
            }
        }

        println!("\n{}", format!("📈 Score de Risco: {:.1}/100", report.risk_score).yellow().bold());

        if json_output {
            let json = serde_json::to_string_pretty(&report)?;
            println!("\n{}", json);
        }
    }

    Ok(())
}

async fn detect_malware(path: &str, _update: bool) -> Result<()> {
    println!("{}", "🦠 Escaneando malware...".cyan());

    let binary = Binary::from_file(path)?;
    let detector = MalwareDetector::new();
    let result = detector.detect(&binary);

    println!("\n{}", "📋 Resultado da Detecção:".green().bold());

    if result.is_malicious {
        println!("  Status: {} (Confiança: {:.1}%)",
            "MALICIOSO".red().bold(),
            result.confidence * 100.0
        );
    } else {
        println!("  Status: {}", "LIMPO".green().bold());
    }

    if let Some(family) = result.family {
        println!("  Família: {}", family.yellow());
    }

    if !result.detections.is_empty() {
        println!("\n{}", "🎯 Detecções:".red().bold());
        for detection in &result.detections {
            println!("  [{:?}] {} - {}",
                detection.detection_type,
                detection.name.red(),
                detection.description
            );
        }
    }

    if !result.behavior_tags.is_empty() {
        println!("\n{}", "🏷️  Tags de Comportamento:".yellow().bold());
        for tag in &result.behavior_tags {
            println!("  • {}", tag);
        }
    }

    Ok(())
}

async fn scan_vulnerabilities(path: &str, find_rop: bool, max_gadgets: usize) -> Result<()> {
    println!("{}", "🔓 Escaneando vulnerabilidades...".cyan());

    let binary = Binary::from_file(path)?;
    let result = VulnerabilityScanner::scan(&binary);

    println!("\n{}", "🛡️  Recursos de Segurança:".green().bold());
    println!("  DEP/NX: {}", if result.security_features.dep_enabled { "✓".green() } else { "✗".red() });
    println!("  ASLR: {}", if result.security_features.aslr_enabled { "✓".green() } else { "✗".red() });
    println!("  Stack Canary: {}", if result.security_features.stack_canary { "✓".green() } else { "✗".red() });
    println!("  PIE: {}", if result.security_features.pie_enabled { "✓".green() } else { "✗".red() });

    if !result.vulnerabilities.is_empty() {
        println!("\n{}", format!("⚠️  Vulnerabilidades Encontradas ({}):", result.vulnerabilities.len()).red().bold());
        for vuln in &result.vulnerabilities {
            println!("  [{:?}] {:?}",
                vuln.severity.to_string().red(),
                vuln.vuln_type
            );
            println!("      {}", vuln.description);
        }
    } else {
        println!("\n{}", "✅ Nenhuma vulnerabilidade óbvia encontrada".green());
    }

    if !result.recommendations.is_empty() {
        println!("\n{}", "💡 Recomendações:".cyan().bold());
        for rec in &result.recommendations {
            println!("  • {}", rec);
        }
    }

    if find_rop {
        println!("\n{}", "🔍 Procurando ROP gadgets...".cyan());
        let finder = RopGadgetFinder::new(binary.arch);
        let gadgets = finder.find_gadgets(&binary, max_gadgets);

        println!("  Encontrados {} gadgets úteis", gadgets.len());

        for (idx, gadget) in gadgets.iter().take(20).enumerate() {
            println!("  [{}] 0x{:016x}: {}",
                idx,
                gadget.address,
                gadget.instructions.join("; ").yellow()
            );
        }

        if gadgets.len() > 20 {
            println!("  ... e mais {} gadgets", gadgets.len() - 20);
        }
    }

    Ok(())
}

async fn disassemble_binary(path: &str, address: Option<String>, count: usize) -> Result<()> {
    println!("{}", "🔧 Desassemblando...".cyan());

    let binary = Binary::from_file(path)?;
    let disasm = crate::core::Disassembler::new(binary.arch);

    let start_addr = if let Some(addr_str) = address {
        u64::from_str_radix(addr_str.trim_start_matches("0x"), 16)?
    } else {
        binary.entry_point
    };

    let instructions = disasm.disassemble(&binary.data, start_addr, count)?;

    println!("\n{}", "📝 Instruções:".green().bold());
    for instr in instructions {
        println!("{}", instr.to_string());
    }

    Ok(())
}

async fn handle_ctf_tool(tool: CtfTools) -> Result<()> {
    match tool {
        CtfTools::Crypto { action } => handle_crypto_action(action).await?,
        CtfTools::Pwn { action } => handle_pwn_action(action).await?,
        CtfTools::Encode { action } => handle_encoding_action(action).await?,
        CtfTools::Forensics { action } => handle_forensics_action(action).await?,
    }
    Ok(())
}

async fn handle_crypto_action(action: CryptoActions) -> Result<()> {
    match action {
        CryptoActions::Hash { data, algorithm } => {
            let bytes = data.as_bytes();
            let hash = match algorithm.as_str() {
                "md5" => CryptoTools::md5(bytes),
                "sha256" => CryptoTools::sha256(bytes),
                "sha512" => CryptoTools::sha512(bytes),
                _ => return Err(anyhow::anyhow!("Algoritmo desconhecido")),
            };
            println!("{}", hash);
        }
        CryptoActions::Xor { data, key } => {
            let result = CryptoTools::xor(data.as_bytes(), key.as_bytes());
            println!("{}", String::from_utf8_lossy(&result));
        }
        CryptoActions::Rot13 { text } => {
            println!("{}", CryptoTools::rot13(&text));
        }
        CryptoActions::XorBrute { file } => {
            let data = fs::read(&file)?;
            let results = CryptoTools::xor_bruteforce(&data);
            println!("Top 5 resultados:");
            for (key, text, score) in results.iter().take(5) {
                println!("\nChave: 0x{:02x} (score: {:.2})", key, score);
                println!("{}", text);
            }
        }
        CryptoActions::Frequency { file } => {
            let data = fs::read(&file)?;
            let freq = CryptoTools::frequency_analysis(&data);
            println!("Top 10 bytes mais frequentes:");
            for (byte, count) in freq.iter().take(10) {
                println!("  0x{:02x} ('{}') : {} ocorrências",
                    byte,
                    if *byte >= 32 && *byte <= 126 { *byte as char } else { '.' },
                    count
                );
            }
        }
    }
    Ok(())
}

async fn handle_pwn_action(action: PwnActions) -> Result<()> {
    match action {
        PwnActions::Cyclic { length } => {
            let pattern = PwnTools::cyclic_pattern(length);
            println!("{}", String::from_utf8_lossy(&pattern));
        }
        PwnActions::CyclicFind { pattern_file, target } => {
            let pattern = fs::read(&pattern_file)?;
            let target_bytes = target.as_bytes();
            if let Some(offset) = PwnTools::cyclic_find(&pattern, target_bytes) {
                println!("Offset: {}", offset);
            } else {
                println!("Não encontrado");
            }
        }
        PwnActions::Pack { value, bits } => {
            let num = if value.starts_with("0x") {
                u64::from_str_radix(&value[2..], 16)?
            } else {
                value.parse()?
            };

            let bytes = if bits == 64 {
                PwnTools::p64(num)
            } else {
                PwnTools::p32(num as u32)
            };

            println!("{}", hex::encode(bytes));
        }
        PwnActions::Shellcode { platform } => {
            let shellcode = match platform.as_str() {
                "linux_x64" => PwnTools::linux_x64_shell(),
                _ => return Err(anyhow::anyhow!("Plataforma não suportada")),
            };
            println!("{}", hex::encode(shellcode));
        }
    }
    Ok(())
}

async fn handle_encoding_action(action: EncodingActions) -> Result<()> {
    match action {
        EncodingActions::B64Encode { data } => {
            println!("{}", EncodingTools::to_base64(data.as_bytes()));
        }
        EncodingActions::B64Decode { data } => {
            let decoded = EncodingTools::from_base64(&data)
                .map_err(|e| anyhow::anyhow!(e))?;
            println!("{}", String::from_utf8_lossy(&decoded));
        }
        EncodingActions::HexEncode { data } => {
            println!("{}", EncodingTools::to_hex(data.as_bytes()));
        }
        EncodingActions::HexDecode { data } => {
            let decoded = EncodingTools::from_hex(&data)
                .map_err(|e| anyhow::anyhow!(e))?;
            println!("{}", String::from_utf8_lossy(&decoded));
        }
        EncodingActions::UrlEncode { data } => {
            println!("{}", EncodingTools::url_encode(&data));
        }
        EncodingActions::UrlDecode { data } => {
            println!("{}", EncodingTools::url_decode(&data)
                .map_err(|e| anyhow::anyhow!(e))?);
        }
        EncodingActions::Detect { data } => {
            let encodings = EncodingTools::detect_encoding(&data);
            println!("Possíveis encodings: {:?}", encodings);
        }
    }
    Ok(())
}

async fn handle_forensics_action(action: ForensicsActions) -> Result<()> {
    match action {
        ForensicsActions::FileType { file } => {
            let data = fs::read(&file)?;
            if let Some(file_type) = ForensicsTools::detect_file_type(&data) {
                println!("Tipo: {}", file_type);
            } else {
                println!("Tipo desconhecido");
            }
        }
        ForensicsActions::HexDump { file, offset, length } => {
            let data = fs::read(&file)?;
            println!("{}", ForensicsTools::hex_dump(&data, offset, length));
        }
        ForensicsActions::Strings { file, min_length } => {
            let data = fs::read(&file)?;
            let strings = ForensicsTools::extract_strings(&data, min_length);
            for (offset, string) in strings {
                println!("0x{:08x}: {}", offset, string);
            }
        }
        ForensicsActions::Entropy { file } => {
            let data = fs::read(&file)?;
            let entropy = ForensicsTools::calculate_entropy(&data);
            println!("Entropia: {:.2} bits", entropy);
        }
    }
    Ok(())
}

async fn extract_strings(path: &str, min: usize) -> Result<()> {
    println!("{}", "🔍 Extraindo strings...".cyan());

    let binary = Binary::from_file(path)?;
    let strings = binary.find_strings(min);

    println!("Encontradas {} strings:", strings.len());
    for string in strings.iter().take(100) {
        println!("  {}", string);
    }

    if strings.len() > 100 {
        println!("... e mais {} strings", strings.len() - 100);
    }

    Ok(())
}

async fn calculate_hashes(path: &str) -> Result<()> {
    let data = fs::read(path)?;

    println!("{}", "🔐 Hashes:".green().bold());
    println!("  MD5:    {}", CryptoTools::md5(&data));
    println!("  SHA-256: {}", CryptoTools::sha256(&data));
    println!("  SHA-512: {}", CryptoTools::sha512(&data));

    Ok(())
}
