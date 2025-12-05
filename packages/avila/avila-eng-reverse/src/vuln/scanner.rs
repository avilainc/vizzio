use crate::core::Binary;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VulnerabilityScanResult {
    pub vulnerabilities: Vec<Vulnerability>,
    pub security_features: SecurityFeatures,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vulnerability {
    pub vuln_type: VulnerabilityType,
    pub severity: VulnSeverity,
    pub description: String,
    pub location: Option<u64>,
    pub proof_of_concept: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnerabilityType {
    BufferOverflow,
    FormatString,
    IntegerOverflow,
    UseAfterFree,
    DoubleFree,
    NullPointerDereference,
    RaceCondition,
    CommandInjection,
    SqlInjection,
    PathTraversal,
    InsecureRandom,
    WeakCrypto,
    HardcodedCredentials,
    StackCanaryMissing,
    AslrDisabled,
    NxDisabled,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VulnSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

impl std::fmt::Display for VulnSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VulnSeverity::Critical => write!(f, "CRITICAL"),
            VulnSeverity::High => write!(f, "HIGH"),
            VulnSeverity::Medium => write!(f, "MEDIUM"),
            VulnSeverity::Low => write!(f, "LOW"),
            VulnSeverity::Info => write!(f, "INFO"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityFeatures {
    pub dep_enabled: bool,  // Data Execution Prevention
    pub aslr_enabled: bool, // Address Space Layout Randomization
    pub stack_canary: bool,
    pub pie_enabled: bool,  // Position Independent Executable
    pub relro: RelroLevel,
    pub fortify: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelroLevel {
    None,
    Partial,
    Full,
}

pub struct VulnerabilityScanner;

impl VulnerabilityScanner {
    pub fn scan(binary: &Binary) -> VulnerabilityScanResult {
        let mut vulnerabilities = Vec::new();

        // Verificar recursos de segurança
        let security_features = Self::check_security_features(binary);

        // Verificar vulnerabilidades comuns
        vulnerabilities.extend(Self::check_buffer_overflows(binary));
        vulnerabilities.extend(Self::check_format_strings(binary));
        vulnerabilities.extend(Self::check_dangerous_functions(binary));
        vulnerabilities.extend(Self::check_crypto_issues(binary));

        // Gerar recomendações
        let recommendations = Self::generate_recommendations(&security_features, &vulnerabilities);

        VulnerabilityScanResult {
            vulnerabilities,
            security_features,
            recommendations,
        }
    }

    fn check_security_features(binary: &Binary) -> SecurityFeatures {
        // Verificações simplificadas - em produção seria mais complexo
        let dep_enabled = match binary.format {
            crate::core::types::BinaryFormat::PE32 | crate::core::types::BinaryFormat::PE64 => {
                // Verificar DLL characteristics no PE header
                true // Placeholder
            },
            _ => false,
        };

        let aslr_enabled = match binary.format {
            crate::core::types::BinaryFormat::PE32 | crate::core::types::BinaryFormat::PE64 => {
                // Verificar dynamic base flag
                true // Placeholder
            },
            crate::core::types::BinaryFormat::ELF32 | crate::core::types::BinaryFormat::ELF64 => {
                // Verificar se é PIE
                false // Placeholder
            },
            _ => false,
        };

        SecurityFeatures {
            dep_enabled,
            aslr_enabled,
            stack_canary: Self::has_stack_canary(binary),
            pie_enabled: false, // Placeholder
            relro: RelroLevel::None, // Placeholder
            fortify: false, // Placeholder
        }
    }

    fn has_stack_canary(binary: &Binary) -> bool {
        // Procurar por __stack_chk_fail ou __stack_chk_guard
        binary.imports.iter().any(|imp|
            imp.function.contains("stack_chk") ||
            imp.function.contains("security_cookie")
        ) || binary.symbols.iter().any(|sym|
            sym.name.contains("stack_chk") ||
            sym.name.contains("security_cookie")
        )
    }

    fn check_buffer_overflows(binary: &Binary) -> Vec<Vulnerability> {
        let mut vulns = Vec::new();

        let dangerous_funcs = [
            "strcpy", "strcat", "sprintf", "vsprintf",
            "gets", "scanf", "sscanf",
        ];

        for import in &binary.imports {
            for &func in &dangerous_funcs {
                if import.function.contains(func) {
                    vulns.push(Vulnerability {
                        vuln_type: VulnerabilityType::BufferOverflow,
                        severity: VulnSeverity::High,
                        description: format!(
                            "Uso de função insegura '{}' que pode levar a buffer overflow",
                            func
                        ),
                        location: import.address,
                        proof_of_concept: Some(format!(
                            "A função {} não verifica limites de buffer",
                            func
                        )),
                    });
                }
            }
        }

        vulns
    }

    fn check_format_strings(binary: &Binary) -> Vec<Vulnerability> {
        let mut vulns = Vec::new();

        // Procurar por padrões de format string vulnerabilities
        let format_funcs = ["printf", "fprintf", "sprintf", "snprintf", "syslog"];

        for import in &binary.imports {
            for &func in &format_funcs {
                if import.function.contains(func) {
                    // Em análise real, verificaríamos se o format string é controlado pelo usuário
                    vulns.push(Vulnerability {
                        vuln_type: VulnerabilityType::FormatString,
                        severity: VulnSeverity::High,
                        description: format!(
                            "Potencial vulnerabilidade de format string em '{}'",
                            func
                        ),
                        location: import.address,
                        proof_of_concept: Some(
                            "Verificar se o format string é controlado por entrada do usuário".to_string()
                        ),
                    });
                }
            }
        }

        vulns
    }

    fn check_dangerous_functions(binary: &Binary) -> Vec<Vulnerability> {
        let mut vulns = Vec::new();

        let dangerous = [
            ("system", VulnSeverity::Critical, "Execução de comando"),
            ("exec", VulnSeverity::Critical, "Execução de processo"),
            ("popen", VulnSeverity::High, "Execução de shell"),
            ("strcpy", VulnSeverity::High, "Cópia sem verificação de tamanho"),
            ("gets", VulnSeverity::Critical, "Leitura sem limite"),
        ];

        for import in &binary.imports {
            for (func, severity, desc) in &dangerous {
                if import.function.to_lowercase().contains(&func.to_lowercase()) {
                    vulns.push(Vulnerability {
                        vuln_type: VulnerabilityType::Other(func.to_string()),
                        severity: severity.clone(),
                        description: format!("Uso de função perigosa: {} - {}", func, desc),
                        location: import.address,
                        proof_of_concept: None,
                    });
                }
            }
        }

        vulns
    }

    fn check_crypto_issues(binary: &Binary) -> Vec<Vulnerability> {
        let mut vulns = Vec::new();

        let weak_crypto = ["md5", "sha1", "des", "rc4"];

        for import in &binary.imports {
            let func_lower = import.function.to_lowercase();
            for &weak in &weak_crypto {
                if func_lower.contains(weak) {
                    vulns.push(Vulnerability {
                        vuln_type: VulnerabilityType::WeakCrypto,
                        severity: VulnSeverity::Medium,
                        description: format!(
                            "Uso de algoritmo criptográfico fraco: {}",
                            weak.to_uppercase()
                        ),
                        location: import.address,
                        proof_of_concept: Some(
                            "Considere usar algoritmos mais seguros como SHA-256, AES".to_string()
                        ),
                    });
                }
            }
        }

        // Verificar senhas hardcoded nas strings
        let strings = binary.find_strings(8);
        for (idx, string) in strings.iter().enumerate() {
            if string.to_lowercase().contains("password") ||
               string.to_lowercase().contains("secret") ||
               string.to_lowercase().contains("apikey") {
                vulns.push(Vulnerability {
                    vuln_type: VulnerabilityType::HardcodedCredentials,
                    severity: VulnSeverity::High,
                    description: "Possível credencial hardcoded encontrada".to_string(),
                    location: Some(idx as u64),
                    proof_of_concept: Some(format!("String suspeita: {}", string)),
                });
            }
        }

        vulns
    }

    fn generate_recommendations(
        features: &SecurityFeatures,
        vulnerabilities: &[Vulnerability],
    ) -> Vec<String> {
        let mut recommendations = Vec::new();

        if !features.dep_enabled {
            recommendations.push(
                "Habilitar DEP (Data Execution Prevention) para prevenir execução de código em áreas de dados".to_string()
            );
        }

        if !features.aslr_enabled {
            recommendations.push(
                "Habilitar ASLR (Address Space Layout Randomization) para dificultar exploits".to_string()
            );
        }

        if !features.stack_canary {
            recommendations.push(
                "Compilar com stack canaries (-fstack-protector) para detectar buffer overflows".to_string()
            );
        }

        if !features.pie_enabled {
            recommendations.push(
                "Compilar como PIE (Position Independent Executable) para melhorar ASLR".to_string()
            );
        }

        let critical_vulns = vulnerabilities.iter()
            .filter(|v| matches!(v.severity, VulnSeverity::Critical))
            .count();

        if critical_vulns > 0 {
            recommendations.push(format!(
                "URGENTE: Corrigir {} vulnerabilidade(s) crítica(s) encontrada(s)",
                critical_vulns
            ));
        }

        if recommendations.is_empty() {
            recommendations.push("Boas práticas de segurança estão sendo seguidas".to_string());
        }

        recommendations
    }
}
