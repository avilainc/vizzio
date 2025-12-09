//! Sistema de renderização de mapas

use crate::error::GeoError;
use super::coordinates::Point2D;

/// Estilo de renderização
#[derive(Clone, Debug)]
pub struct Style {
    /// Cor de preenchimento (RGBA)
    pub fill_color: [u8; 4],

    /// Cor da borda (RGBA)
    pub stroke_color: [u8; 4],

    /// Largura da borda
    pub stroke_width: f32,

    /// Opacidade (0.0 a 1.0)
    pub opacity: f32,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fill_color: [220, 220, 220, 255],    // Cinza claro
            stroke_color: [100, 100, 100, 255],  // Cinza escuro
            stroke_width: 1.0,
            opacity: 1.0,
        }
    }
}

impl Style {
    pub fn land() -> Self {
        Self {
            fill_color: [240, 240, 220, 255],    // Bege
            stroke_color: [120, 120, 120, 255],  // Cinza
            stroke_width: 1.0,
            opacity: 1.0,
        }
    }

    pub fn water() -> Self {
        Self {
            fill_color: [173, 216, 230, 255],    // Azul claro
            stroke_color: [100, 150, 180, 255],  // Azul médio
            stroke_width: 0.5,
            opacity: 1.0,
        }
    }

    pub fn graticule() -> Self {
        Self {
            fill_color: [0, 0, 0, 0],            // Transparente
            stroke_color: [200, 200, 200, 128],  // Cinza transparente
            stroke_width: 0.5,
            opacity: 0.5,
        }
    }
}

/// Opções de renderização
#[derive(Clone, Debug)]
pub struct RenderOptions {
    /// Largura do canvas em pixels
    pub width: u32,

    /// Altura do canvas em pixels
    pub height: u32,

    /// Cor de fundo
    pub background_color: [u8; 4],

    /// Mostrar grade de coordenadas
    pub show_graticule: bool,

    /// Estilo da grade
    pub graticule_style: Style,

    /// Mostrar nomes de países
    pub show_labels: bool,

    /// DPI para renderização
    pub dpi: u32,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            background_color: [240, 248, 255, 255], // Alice blue
            show_graticule: true,
            graticule_style: Style::graticule(),
            show_labels: true,
            dpi: 96,
        }
    }
}

/// Formato de saída
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutputFormat {
    /// PNG (bitmap)
    Png,
    /// SVG (vetorial)
    Svg,
    /// PDF (vetorial)
    Pdf,
    /// JSON (dados brutos)
    Json,
}

/// Trait para renderizadores de mapas
pub trait MapRenderer: Send {
    /// Inicializa o renderer com dimensões especificadas
    fn begin(&mut self, width: u32, height: u32) -> Result<(), GeoError>;

    /// Desenha o fundo
    fn draw_background(&mut self, options: &RenderOptions) -> Result<(), GeoError>;

    /// Desenha um polígono
    fn draw_polygon(&mut self, points: &[Point2D], style: &Style) -> Result<(), GeoError>;

    /// Desenha uma linha
    fn draw_line(&mut self, points: &[Point2D], style: &Style) -> Result<(), GeoError>;

    /// Desenha um texto/label
    fn draw_label(&mut self, text: &str, position: Point2D) -> Result<(), GeoError>;

    /// Finaliza e retorna os dados renderizados
    fn end(&mut self) -> Result<Vec<u8>, GeoError>;

    /// Formato de saída
    fn format(&self) -> OutputFormat;
}

/// Renderer SVG (vetorial)
pub struct SvgRenderer {
    width: u32,
    height: u32,
    elements: Vec<String>,
}

impl SvgRenderer {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            elements: Vec::new(),
        }
    }

    fn color_to_hex(color: [u8; 4]) -> String {
        format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2])
    }

    fn opacity_from_color(color: [u8; 4]) -> f32 {
        color[3] as f32 / 255.0
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MapRenderer for SvgRenderer {
    fn begin(&mut self, width: u32, height: u32) -> Result<(), GeoError> {
        self.width = width;
        self.height = height;
        self.elements.clear();
        Ok(())
    }

    fn draw_background(&mut self, options: &RenderOptions) -> Result<(), GeoError> {
        let color = Self::color_to_hex(options.background_color);
        let bg = format!(
            r#"<rect width="100%" height="100%" fill="{}"/>"#,
            color
        );
        self.elements.push(bg);
        Ok(())
    }

    fn draw_polygon(&mut self, points: &[Point2D], style: &Style) -> Result<(), GeoError> {
        if points.is_empty() {
            return Ok(());
        }

        let points_str: Vec<String> = points
            .iter()
            .map(|p| format!("{:.2},{:.2}", p.x, p.y))
            .collect();

        let fill = Self::color_to_hex(style.fill_color);
        let stroke = Self::color_to_hex(style.stroke_color);
        let fill_opacity = Self::opacity_from_color(style.fill_color);
        let stroke_opacity = Self::opacity_from_color(style.stroke_color);

        let polygon = format!(
            r#"<polygon points="{}" fill="{}" fill-opacity="{:.2}" stroke="{}" stroke-opacity="{:.2}" stroke-width="{:.2}"/>"#,
            points_str.join(" "),
            fill,
            fill_opacity,
            stroke,
            stroke_opacity,
            style.stroke_width
        );

        self.elements.push(polygon);
        Ok(())
    }

    fn draw_line(&mut self, points: &[Point2D], style: &Style) -> Result<(), GeoError> {
        if points.len() < 2 {
            return Ok(());
        }

        let points_str: Vec<String> = points
            .iter()
            .map(|p| format!("{:.2},{:.2}", p.x, p.y))
            .collect();

        let stroke = Self::color_to_hex(style.stroke_color);
        let opacity = Self::opacity_from_color(style.stroke_color);

        let line = format!(
            r#"<polyline points="{}" fill="none" stroke="{}" stroke-opacity="{:.2}" stroke-width="{:.2}"/>"#,
            points_str.join(" "),
            stroke,
            opacity,
            style.stroke_width
        );

        self.elements.push(line);
        Ok(())
    }

    fn draw_label(&mut self, text: &str, position: Point2D) -> Result<(), GeoError> {
        let label = format!(
            r#"<text x="{:.2}" y="{:.2}" font-family="Arial" font-size="12" fill="#333">{}</text>"#,
            position.x,
            position.y,
            text
        );
        self.elements.push(label);
        Ok(())
    }

    fn end(&mut self) -> Result<Vec<u8>, GeoError> {
        let mut svg = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">
"#,
            self.width, self.height, self.width, self.height
        );

        for element in &self.elements {
            svg.push_str("  ");
            svg.push_str(element);
            svg.push('\n');
        }

        svg.push_str("</svg>");

        Ok(svg.into_bytes())
    }

    fn format(&self) -> OutputFormat {
        OutputFormat::Svg
    }
}

/// Renderer JSON (dados estruturados)
pub struct JsonRenderer {
    width: u32,
    height: u32,
    features: Vec<serde_json::Value>,
}

impl JsonRenderer {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            features: Vec::new(),
        }
    }
}

impl Default for JsonRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl MapRenderer for JsonRenderer {
    fn begin(&mut self, width: u32, height: u32) -> Result<(), GeoError> {
        self.width = width;
        self.height = height;
        self.features.clear();
        Ok(())
    }

    fn draw_background(&mut self, options: &RenderOptions) -> Result<(), GeoError> {
        let bg = serde_json::json!({
            "type": "background",
            "color": options.background_color,
        });
        self.features.push(bg);
        Ok(())
    }

    fn draw_polygon(&mut self, points: &[Point2D], style: &Style) -> Result<(), GeoError> {
        let coords: Vec<[f64; 2]> = points
            .iter()
            .map(|p| [p.x, p.y])
            .collect();

        let polygon = serde_json::json!({
            "type": "polygon",
            "coordinates": coords,
            "style": {
                "fill": style.fill_color,
                "stroke": style.stroke_color,
                "stroke_width": style.stroke_width,
            }
        });

        self.features.push(polygon);
        Ok(())
    }

    fn draw_line(&mut self, points: &[Point2D], style: &Style) -> Result<(), GeoError> {
        let coords: Vec<[f64; 2]> = points
            .iter()
            .map(|p| [p.x, p.y])
            .collect();

        let line = serde_json::json!({
            "type": "line",
            "coordinates": coords,
            "style": {
                "stroke": style.stroke_color,
                "stroke_width": style.stroke_width,
            }
        });

        self.features.push(line);
        Ok(())
    }

    fn draw_label(&mut self, text: &str, position: Point2D) -> Result<(), GeoError> {
        let label = serde_json::json!({
            "type": "label",
            "text": text,
            "position": [position.x, position.y],
        });

        self.features.push(label);
        Ok(())
    }

    fn end(&mut self) -> Result<Vec<u8>, GeoError> {
        let output = serde_json::json!({
            "type": "FeatureCollection",
            "width": self.width,
            "height": self.height,
            "features": self.features,
        });

        let json = serde_json::to_string_pretty(&output)
            .map_err(|e| GeoError::RenderError(e.to_string()))?;

        Ok(json.into_bytes())
    }

    fn format(&self) -> OutputFormat {
        OutputFormat::Json
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svg_renderer() {
        let mut renderer = SvgRenderer::new();
        renderer.begin(800, 600).unwrap();

        let points = vec![
            Point2D::new(100.0, 100.0),
            Point2D::new(200.0, 100.0),
            Point2D::new(150.0, 200.0),
        ];

        renderer.draw_polygon(&points, &Style::default()).unwrap();
        let result = renderer.end().unwrap();

        let svg = String::from_utf8(result).unwrap();
        assert!(svg.contains("<svg"));
        assert!(svg.contains("<polygon"));
    }
}
