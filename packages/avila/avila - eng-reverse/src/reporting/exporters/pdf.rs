// PDF exporter
use super::super::generator::Report;
use super::ReportExporter;
use std::error::Error;

pub struct PdfExporter;

impl ReportExporter for PdfExporter {
    fn export(&self, report: &Report) -> Result<Vec<u8>, Box<dyn Error>> {
        // TODO: Implement PDF generation (using printpdf or similar)
        Err("PDF export not yet implemented".into())
    }

    fn file_extension(&self) -> &str {
        "pdf"
    }
}
