// Report exporters
use super::generator::Report;
use std::error::Error;

pub mod json;
pub mod html;
pub mod pdf;
pub mod markdown;

pub use json::JsonExporter;
pub use html::HtmlExporter;
pub use pdf::PdfExporter;
pub use markdown::MarkdownExporter;

/// Trait for report exporters
pub trait ReportExporter {
    fn export(&self, report: &Report) -> Result<Vec<u8>, Box<dyn Error>>;
    fn file_extension(&self) -> &str;
}
