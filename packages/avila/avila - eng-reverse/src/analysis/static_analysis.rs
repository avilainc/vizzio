use crate::core::Binary;
use crate::core::types::Result;
use crate::analysis::static::cfg_builder::CfgBuilder;

pub struct StaticAnalyzer;

#[derive(Debug, Clone)]
pub struct ControlFlowAnalysis {
    pub num_basic_blocks: usize,
    pub num_functions: usize,
    pub cyclomatic_complexity: f64,
    pub has_loops: bool,
    pub has_dead_code: bool,
    pub suspicious_patterns: Vec<String>,
}

impl StaticAnalyzer {
    pub fn analyze_control_flow(binary: &Binary) -> Result<ControlFlowAnalysis> {
        let mut cfg = CfgBuilder::new();

        // Construir CFG a partir do entry point
        if let Err(e) = cfg.build(&binary.data, binary.entry_point) {
            return Err(crate::core::types::DeriaxError::AnalysisError(
                format!("Erro ao construir CFG: {}", e)
            ));
        }

        // Análise de loops
        let loops = cfg.find_loops();
        let has_loops = !loops.is_empty();

        // Análise de código morto
        let dead_code = cfg.find_dead_code();
        let has_dead_code = !dead_code.is_empty();

        // Análise de ofuscação
        let obfuscation = cfg.detect_obfuscation();
        let mut suspicious_patterns = Vec::new();

        if obfuscation.has_control_flow_flattening {
            suspicious_patterns.push("Control Flow Flattening detectado".to_string());
        }
        if obfuscation.has_junk_code {
            suspicious_patterns.push("Junk Code detectado".to_string());
        }
        if obfuscation.complexity_score > 50.0 {
            suspicious_patterns.push(format!("Complexidade ciclomática alta: {:.2}", obfuscation.complexity_score));
        }

        Ok(ControlFlowAnalysis {
            num_basic_blocks: cfg.node_count(),
            num_functions: 1, // Simplificado - seria necessário identificar funções
            cyclomatic_complexity: obfuscation.complexity_score,
            has_loops,
            has_dead_code,
            suspicious_patterns,
        })
    }

    pub fn detect_obfuscation(binary: &Binary) -> Result<bool> {
        // Detectar ofuscação através de entropia e padrões
        let high_entropy_sections = binary.sections.iter()
            .filter(|s| s.entropy > 7.5)
            .count();

        Ok(high_entropy_sections > 0)
    }

    pub fn find_interesting_constants(data: &[u8]) -> Vec<u64> {
        // Procurar constantes interessantes (magic numbers, etc)
        let mut constants = Vec::new();

        // Implementação simplificada
        for i in 0..data.len().saturating_sub(8) {
            let value = u64::from_le_bytes([
                data[i], data[i+1], data[i+2], data[i+3],
                data[i+4], data[i+5], data[i+6], data[i+7],
            ]);

            // Verificar se é um valor interessante
            if value > 0x10000 && value < 0x7fffffff {
                constants.push(value);
            }
        }

        constants
    }
}
