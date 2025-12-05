// Report templates
pub struct Templates;

impl Templates {
    /// Executive summary template
    pub fn executive_summary() -> String {
        r#"
# Executive Summary

## Overview
{{overview}}

## Key Findings
{{findings}}

## Recommendations
{{recommendations}}
"#.to_string()
    }

    /// Technical deep-dive template
    pub fn technical_deepdive() -> String {
        r#"
# Technical Analysis Report

## File Information
- Filename: {{filename}}
- Hash: {{hash}}
- Size: {{size}}

## Static Analysis
{{static_analysis}}

## Dynamic Analysis
{{dynamic_analysis}}

## Vulnerabilities
{{vulnerabilities}}

## IOCs
{{iocs}}
"#.to_string()
    }

    /// Malware analysis template
    pub fn malware_analysis() -> String {
        r#"
# Malware Analysis Report

## Sample Information
{{sample_info}}

## Behavioral Analysis
{{behavior}}

## Network Activity
{{network}}

## File Operations
{{file_ops}}

## MITRE ATT&CK Mapping
{{mitre_attack}}
"#.to_string()
    }
}
