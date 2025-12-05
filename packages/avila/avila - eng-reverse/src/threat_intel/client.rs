// Unified threat intelligence client
use super::virustotal::VirusTotalClient;
use super::otx::OtxClient;
use super::misp::MispClient;
use std::error::Error;

/// Unified threat intelligence client
pub struct ThreatIntelClient {
    vt_client: Option<VirusTotalClient>,
    otx_client: Option<OtxClient>,
    misp_client: Option<MispClient>,
}

impl ThreatIntelClient {
    pub fn new() -> Self {
        Self {
            vt_client: None,
            otx_client: None,
            misp_client: None,
        }
    }

    /// Configure VirusTotal
    pub fn with_virustotal(mut self, api_key: String) -> Self {
        self.vt_client = Some(VirusTotalClient::new(api_key));
        self
    }

    /// Configure OTX
    pub fn with_otx(mut self, api_key: String) -> Self {
        self.otx_client = Some(OtxClient::new(api_key));
        self
    }

    /// Configure MISP
    pub fn with_misp(mut self, url: String, api_key: String) -> Self {
        self.misp_client = Some(MispClient::new(url, api_key));
        self
    }

    /// Query all sources for hash
    pub async fn query_hash(&self, hash: &str) -> Result<ThreatIntelReport, Box<dyn Error>> {
        let mut report = ThreatIntelReport::new(hash.to_string());

        // Query VirusTotal
        if let Some(vt) = &self.vt_client {
            if let Ok(vt_report) = vt.query_hash(hash).await {
                report.vt_detections = Some(vt_report.detections);
                report.vt_total = Some(vt_report.total_engines);
            }
        }

        // Query OTX
        if let Some(otx) = &self.otx_client {
            if let Ok(otx_report) = otx.query_indicator(hash, "file").await {
                report.otx_pulses = otx_report.pulses.len();
            }
        }

        Ok(report)
    }

    /// Enrich IOC with threat intelligence
    pub async fn enrich_ioc(&self, ioc: &str, ioc_type: &str) -> Result<EnrichedIoc, Box<dyn Error>> {
        Ok(EnrichedIoc {
            value: ioc.to_string(),
            ioc_type: ioc_type.to_string(),
            tags: Vec::new(),
            sources: Vec::new(),
            risk_score: 0,
        })
    }
}

#[derive(Debug, Clone)]
pub struct ThreatIntelReport {
    pub hash: String,
    pub vt_detections: Option<usize>,
    pub vt_total: Option<usize>,
    pub otx_pulses: usize,
    pub misp_events: usize,
    pub overall_verdict: Verdict,
}

impl ThreatIntelReport {
    pub fn new(hash: String) -> Self {
        Self {
            hash,
            vt_detections: None,
            vt_total: None,
            otx_pulses: 0,
            misp_events: 0,
            overall_verdict: Verdict::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Verdict {
    Malicious,
    Suspicious,
    Clean,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct EnrichedIoc {
    pub value: String,
    pub ioc_type: String,
    pub tags: Vec<String>,
    pub sources: Vec<String>,
    pub risk_score: u32,
}
