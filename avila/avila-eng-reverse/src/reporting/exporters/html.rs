// HTML exporter
use super::super::generator::Report;
use super::ReportExporter;
use std::error::Error;

pub struct HtmlExporter;

impl ReportExporter for HtmlExporter {
    fn export(&self, report: &Report) -> Result<Vec<u8>, Box<dyn Error>> {
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&format!("<title>{}</title>\n", report.title));
        html.push_str("<style>body { font-family: Arial, sans-serif; margin: 40px; }</style>\n");
        html.push_str("</head>\n<body>\n");
        html.push_str(&format!("<h1>{}</h1>\n", report.title));
        html.push_str(&format!("<p>{}</p>\n", report.summary));

        for section in &report.sections {
            html.push_str(&format!("<h2>{}</h2>\n", section.title));
            html.push_str(&format!("<div>{}</div>\n", section.content));
        }

        html.push_str("<hr>\n");
        html.push_str(&format!("<small>Generated: {}</small>\n", report.metadata.generated_at));
        html.push_str("</body>\n</html>");

        Ok(html.into_bytes())
    }

    fn file_extension(&self) -> &str {
        "html"
    }
}
