// Markdown exporter
use super::super::generator::Report;
use super::ReportExporter;
use std::error::Error;

pub struct MarkdownExporter;

impl ReportExporter for MarkdownExporter {
    fn export(&self, report: &Report) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", report.title));
        md.push_str(&format!("{}\n\n", report.summary));

        for section in &report.sections {
            md.push_str(&format!("## {}\n\n", section.title));
            md.push_str(&format!("{}\n\n", section.content));
        }

        md.push_str("---\n\n");
        md.push_str(&format!("*Generated: {} | Version: {}*\n",
            report.metadata.generated_at,
            report.metadata.tool_version));

        Ok(md.into_bytes())
    }

    fn file_extension(&self) -> &str {
        "md"
    }
}
