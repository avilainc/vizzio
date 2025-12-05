// IOC enrichment
use std::error::Error;

/// IOC enricher
pub struct IocEnricher {
    mitre_mappings: std::collections::HashMap<String, Vec<String>>,
}

impl IocEnricher {
    pub fn new() -> Self {
        let mut enricher = Self {
            mitre_mappings: std::collections::HashMap::new(),
        };
        enricher.load_mitre_mappings();
        enricher
    }

    /// Enrich IOC with context
    pub fn enrich(&self, ioc: &str, ioc_type: &str) -> EnrichedIoc {
        EnrichedIoc {
            value: ioc.to_string(),
            ioc_type: ioc_type.to_string(),
            context: self.get_context(ioc, ioc_type),
            mitre_techniques: self.get_mitre_techniques(ioc_type),
            threat_actors: Vec::new(),
            campaigns: Vec::new(),
        }
    }

    /// Get context for IOC
    fn get_context(&self, ioc: &str, ioc_type: &str) -> String {
        match ioc_type {
            "ip" => "IP address associated with C2 infrastructure".to_string(),
            "domain" => "Domain used for malware distribution".to_string(),
            "hash" => "File hash of known malware".to_string(),
            _ => "Unknown IOC type".to_string(),
        }
    }

    /// Map IOC to MITRE ATT&CK techniques
    fn get_mitre_techniques(&self, ioc_type: &str) -> Vec<String> {
        self.mitre_mappings
            .get(ioc_type)
            .cloned()
            .unwrap_or_default()
    }

    /// Load MITRE ATT&CK mappings
    fn load_mitre_mappings(&mut self) {
        self.mitre_mappings.insert(
            "ip".to_string(),
            vec!["T1071".to_string(), "T1090".to_string()], // C2, Proxy
        );
        self.mitre_mappings.insert(
            "domain".to_string(),
            vec!["T1071".to_string(), "T1566".to_string()], // C2, Phishing
        );
    }
}

#[derive(Debug, Clone)]
pub struct EnrichedIoc {
    pub value: String,
    pub ioc_type: String,
    pub context: String,
    pub mitre_techniques: Vec<String>,
    pub threat_actors: Vec<String>,
    pub campaigns: Vec<String>,
}
