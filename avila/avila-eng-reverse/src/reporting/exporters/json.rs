// JSON exporter - implementação manual sem dependências
use super::super::generator::Report;
use super::ReportExporter;
use std::error::Error;

pub struct JsonExporter;

impl JsonExporter {
    fn escape_json_string(s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '"' => "\\\"".to_string(),
                '\\' => "\\\\".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }
}

impl ReportExporter for JsonExporter {
    fn export(&self, report: &Report) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut json = String::from("{\n");
        
        // Title
        json.push_str(&format!("  \"title\": \"{}\",\n", 
            Self::escape_json_string(&report.title)));
        
        // Summary
        json.push_str(&format!("  \"summary\": \"{}\",\n", 
            Self::escape_json_string(&report.summary)));
        
        // Sections
        json.push_str("  \"sections\": [\n");
        for (i, section) in report.sections.iter().enumerate() {
            json.push_str("    {\n");
            json.push_str(&format!("      \"title\": \"{}\",\n", 
                Self::escape_json_string(&section.title)));
            json.push_str(&format!("      \"content\": \"{}\"\n", 
                Self::escape_json_string(&section.content)));
            json.push_str("    }");
            if i < report.sections.len() - 1 {
                json.push_str(",");
            }
            json.push_str("\n");
        }
        json.push_str("  ],\n");
        
        // Metadata
        json.push_str("  \"metadata\": {\n");
        json.push_str(&format!("    \"generated_at\": \"{}\",\n", 
            Self::escape_json_string(&report.metadata.generated_at)));
        json.push_str(&format!("    \"tool_version\": \"{}\"\n", 
            Self::escape_json_string(&report.metadata.tool_version)));
        json.push_str("  }\n");
        
        json.push_str("}\n");

        Ok(json.into_bytes())
    }

    fn file_extension(&self) -> &str {
        "json"
    }
}
