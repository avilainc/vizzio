use crate::core::{Binary, Disassembler};
use crate::core::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub binary_info: BinaryInfo,
    pub suspicious_indicators: Vec<SuspiciousIndicator>,
    pub strings: Vec<StringMatch>,
    pub api_calls: Vec<ApiCall>,
    pub code_sections: Vec<CodeSectionAnalysis>,
    pub risk_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryInfo {
    pub path: String,
    pub format: BinaryFormat,
    pub architecture: String,
    pub entry_point: u64,
    pub image_base: u64,
    pub hashes: FileHash,
    pub file_size: usize,
    pub compilation_timestamp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuspiciousIndicator {
    pub category: String,
    pub description: String,
    pub severity: Severity,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringMatch {
    pub value: String,
    pub offset: usize,
    pub category: StringCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StringCategory {
    Url,
    IpAddress,
    FilePath,
    Registry,
    Crypto,
    Suspicious,
    Normal,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiCall {
    pub function: String,
    pub library: String,
    pub category: ApiCategory,
    pub risk_level: Severity,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ApiCategory {
    FileSystem,
    Network,
    Process,
    Registry,
    Crypto,
    Memory,
    System,
    Other,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeSectionAnalysis {
    pub section_name: String,
    pub entropy: f64,
    pub is_executable: bool,
    pub is_writable: bool,
    pub suspicious: bool,
    pub reason: Option<String>,
}

pub struct BinaryAnalyzer;

impl BinaryAnalyzer {
    pub fn analyze(binary: &Binary) -> Result<AnalysisReport> {
        let mut report = AnalysisReport {
            binary_info: Self::extract_binary_info(binary),
            suspicious_indicators: Vec::new(),
            strings: Vec::new(),
            api_calls: Vec::new(),
            code_sections: Vec::new(),
            risk_score: 0.0,
        };

        // Análise de seções
        report.code_sections = Self::analyze_sections(&binary.sections);

        // Análise de strings
        let strings = binary.find_strings(4);
        report.strings = Self::categorize_strings(&strings);

        // Análise de APIs
        report.api_calls = Self::analyze_imports(&binary.imports);

        // Detectar indicadores suspeitos
        report.suspicious_indicators = Self::detect_suspicious_patterns(binary, &report);

        // Calcular score de risco
        report.risk_score = Self::calculate_risk_score(&report);

        Ok(report)
    }

    fn extract_binary_info(binary: &Binary) -> BinaryInfo {
        BinaryInfo {
            path: binary.path.clone(),
            format: binary.format,
            architecture: binary.arch.name().to_string(),
            entry_point: binary.entry_point,
            image_base: binary.image_base,
            hashes: binary.hashes.clone(),
            file_size: binary.data.len(),
            compilation_timestamp: None,
        }
    }

    fn analyze_sections(sections: &[Section]) -> Vec<CodeSectionAnalysis> {
        sections.iter().map(|section| {
            let is_executable = (section.characteristics & 0x20000000) != 0;
            let is_writable = (section.characteristics & 0x80000000) != 0;
            let high_entropy = section.entropy > 7.0;

            let suspicious = (is_executable && is_writable) ||
                           (is_executable && high_entropy) ||
                           (section.name.contains("text") && high_entropy);

            let reason = if suspicious {
                if is_executable && is_writable {
                    Some("Seção executável e gravável (possível código injetado)".to_string())
                } else if high_entropy {
                    Some(format!("Alta entropia ({:.2}) - possível código ofuscado/criptografado", section.entropy))
                } else {
                    None
                }
            } else {
                None
            };

            CodeSectionAnalysis {
                section_name: section.name.clone(),
                entropy: section.entropy,
                is_executable,
                is_writable,
                suspicious,
                reason,
            }
        }).collect()
    }

    fn categorize_strings(strings: &[String]) -> Vec<StringMatch> {
        use regex::Regex;

        let url_regex = Regex::new(r"https?://[^\s]+").unwrap();
        let ip_regex = Regex::new(r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b").unwrap();
        let registry_regex = Regex::new(r"HKEY_|\\SOFTWARE\\|\\SYSTEM\\").unwrap();

        let suspicious_keywords = [
            "cmd.exe", "powershell", "rundll32", "regsvr32",
            "CreateRemoteThread", "VirtualAlloc", "WriteProcessMemory",
            "keylog", "password", "credential", "token",
        ];

        strings.iter().enumerate().map(|(offset, s)| {
            let category = if url_regex.is_match(s) {
                StringCategory::Url
            } else if ip_regex.is_match(s) {
                StringCategory::IpAddress
            } else if registry_regex.is_match(s) {
                StringCategory::Registry
            } else if s.contains(":\\") || s.contains("/etc/") {
                StringCategory::FilePath
            } else if suspicious_keywords.iter().any(|k| s.to_lowercase().contains(k)) {
                StringCategory::Suspicious
            } else {
                StringCategory::Normal
            };

            StringMatch {
                value: s.clone(),
                offset,
                category,
            }
        }).collect()
    }

    fn analyze_imports(imports: &[Import]) -> Vec<ApiCall> {
        let mut api_calls = Vec::new();

        for import in imports {
            let (category, risk) = Self::categorize_api(&import.function);

            api_calls.push(ApiCall {
                function: import.function.clone(),
                library: import.library.clone(),
                category,
                risk_level: risk,
            });
        }

        api_calls
    }

    fn categorize_api(function: &str) -> (ApiCategory, Severity) {
        let func_lower = function.to_lowercase();

        match func_lower.as_str() {
            s if s.contains("file") || s.contains("read") || s.contains("write") =>
                (ApiCategory::FileSystem, Severity::Low),
            s if s.contains("socket") || s.contains("connect") || s.contains("send") || s.contains("recv") =>
                (ApiCategory::Network, Severity::Medium),
            s if s.contains("process") || s.contains("thread") || s.contains("inject") =>
                (ApiCategory::Process, Severity::High),
            s if s.contains("registry") || s.contains("reg") =>
                (ApiCategory::Registry, Severity::Medium),
            s if s.contains("crypt") || s.contains("hash") =>
                (ApiCategory::Crypto, Severity::Low),
            s if s.contains("virtual") || s.contains("alloc") || s.contains("memory") =>
                (ApiCategory::Memory, Severity::Medium),
            _ => (ApiCategory::Other, Severity::Low),
        }
    }

    fn detect_suspicious_patterns(binary: &Binary, report: &AnalysisReport) -> Vec<SuspiciousIndicator> {
        let mut indicators = Vec::new();

        // Verificar seções suspeitas
        for section_analysis in &report.code_sections {
            if section_analysis.suspicious {
                indicators.push(SuspiciousIndicator {
                    category: "Seção Suspeita".to_string(),
                    description: format!("Seção '{}' com comportamento anômalo", section_analysis.section_name),
                    severity: Severity::High,
                    details: section_analysis.reason.clone().unwrap_or_default(),
                });
            }
        }

        // Verificar APIs perigosas
        let dangerous_apis = report.api_calls.iter()
            .filter(|api| matches!(api.risk_level, Severity::High | Severity::Critical))
            .count();

        if dangerous_apis > 5 {
            indicators.push(SuspiciousIndicator {
                category: "APIs Perigosas".to_string(),
                description: format!("Encontradas {} APIs de alto risco", dangerous_apis),
                severity: Severity::High,
                details: "Múltiplas APIs perigosas detectadas".to_string(),
            });
        }

        // Verificar strings suspeitas
        let suspicious_strings = report.strings.iter()
            .filter(|s| matches!(s.category, StringCategory::Suspicious))
            .count();

        if suspicious_strings > 3 {
            indicators.push(SuspiciousIndicator {
                category: "Strings Suspeitas".to_string(),
                description: format!("{} strings suspeitas encontradas", suspicious_strings),
                severity: Severity::Medium,
                details: "Palavras-chave relacionadas a atividades maliciosas".to_string(),
            });
        }

        indicators
    }

    fn calculate_risk_score(report: &AnalysisReport) -> f64 {
        let mut score = 0.0;

        // Seções suspeitas
        for section in &report.code_sections {
            if section.suspicious {
                score += 20.0;
            }
        }

        // APIs de alto risco
        for api in &report.api_calls {
            score += match api.risk_level {
                Severity::Critical => 15.0,
                Severity::High => 10.0,
                Severity::Medium => 5.0,
                Severity::Low => 1.0,
            };
        }

        // Strings suspeitas
        for string in &report.strings {
            if matches!(string.category, StringCategory::Suspicious) {
                score += 5.0;
            }
        }

        // Indicadores
        for indicator in &report.suspicious_indicators {
            score += match indicator.severity {
                Severity::Critical => 25.0,
                Severity::High => 15.0,
                Severity::Medium => 10.0,
                Severity::Low => 5.0,
            };
        }

        score.min(100.0)
    }
}
