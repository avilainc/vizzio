//! Sistema de simbologia para mapas temáticos

use crate::cartography::renderer::Style;

/// Tipo de símbolo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    /// Círculo
    Circle,
    /// Quadrado
    Square,
    /// Triângulo
    Triangle,
    /// Estrela
    Star,
    /// Diamante
    Diamond,
    /// Cruz
    Cross,
    /// Pin (marcador de mapa)
    Pin,
}

/// Estilo de símbolo
#[derive(Debug, Clone)]
pub struct SymbolStyle {
    /// Tipo de símbolo
    pub symbol_type: SymbolType,

    /// Tamanho (em pixels)
    pub size: f32,

    /// Cor de preenchimento (RGBA)
    pub fill_color: [u8; 4],

    /// Cor da borda (RGBA)
    pub stroke_color: [u8; 4],

    /// Largura da borda
    pub stroke_width: f32,

    /// Opacidade (0.0 a 1.0)
    pub opacity: f32,

    /// Rotação (em graus)
    pub rotation: f32,
}

impl Default for SymbolStyle {
    fn default() -> Self {
        Self {
            symbol_type: SymbolType::Circle,
            size: 8.0,
            fill_color: [255, 0, 0, 255],      // Vermelho
            stroke_color: [0, 0, 0, 255],      // Preto
            stroke_width: 1.0,
            opacity: 1.0,
            rotation: 0.0,
        }
    }
}

impl SymbolStyle {
    /// Cria símbolo simples com cor
    pub fn simple(color: [u8; 4], size: f32) -> Self {
        Self {
            fill_color: color,
            size,
            ..Default::default()
        }
    }

    /// Converte para Style do renderer
    pub fn to_render_style(&self) -> Style {
        Style {
            fill_color: self.fill_color,
            stroke_color: self.stroke_color,
            stroke_width: self.stroke_width,
            opacity: self.opacity,
        }
    }
}

/// Rampa de cores (gradient)
#[derive(Debug, Clone)]
pub struct ColorRamp {
    /// Cores do gradient (mínimo 2)
    pub colors: Vec<[u8; 4]>,

    /// Valores associados às cores
    pub values: Vec<f64>,
}

impl ColorRamp {
    /// Cria rampa de 2 cores
    pub fn two_color(min_color: [u8; 4], max_color: [u8; 4]) -> Self {
        Self {
            colors: vec![min_color, max_color],
            values: vec![0.0, 1.0],
        }
    }

    /// Cria rampa de 3 cores (min, mid, max)
    pub fn three_color(min: [u8; 4], mid: [u8; 4], max: [u8; 4]) -> Self {
        Self {
            colors: vec![min, mid, max],
            values: vec![0.0, 0.5, 1.0],
        }
    }

    /// Rampa vermelho → amarelo → verde (bom para visualizar dados)
    pub fn traffic_light() -> Self {
        Self::three_color(
            [255, 0, 0, 255],     // Vermelho
            [255, 255, 0, 255],   // Amarelo
            [0, 255, 0, 255],     // Verde
        )
    }

    /// Rampa azul → branco → vermelho (temperatura)
    pub fn temperature() -> Self {
        Self::three_color(
            [0, 0, 255, 255],     // Azul frio
            [255, 255, 255, 255], // Branco neutro
            [255, 0, 0, 255],     // Vermelho quente
        )
    }

    /// Rampa tons de azul (dados quantitativos)
    pub fn blues() -> Self {
        Self {
            colors: vec![
                [247, 251, 255, 255],  // Azul muito claro
                [198, 219, 239, 255],  // Azul claro
                [107, 174, 214, 255],  // Azul médio
                [33, 113, 181, 255],   // Azul escuro
                [8, 48, 107, 255],     // Azul muito escuro
            ],
            values: vec![0.0, 0.25, 0.5, 0.75, 1.0],
        }
    }

    /// Rampa tons de verde
    pub fn greens() -> Self {
        Self {
            colors: vec![
                [247, 252, 245, 255],
                [199, 233, 192, 255],
                [116, 196, 118, 255],
                [35, 139, 69, 255],
                [0, 68, 27, 255],
            ],
            values: vec![0.0, 0.25, 0.5, 0.75, 1.0],
        }
    }

    /// Interpola cor para um valor (0.0 a 1.0)
    pub fn interpolate(&self, value: f64) -> [u8; 4] {
        let value = value.clamp(0.0, 1.0);

        // Encontra os dois valores mais próximos
        let mut idx = 0;
        for i in 0..self.values.len() - 1 {
            if value >= self.values[i] && value <= self.values[i + 1] {
                idx = i;
                break;
            }
        }

        let v1 = self.values[idx];
        let v2 = self.values[idx + 1];
        let c1 = self.colors[idx];
        let c2 = self.colors[idx + 1];

        // Interpolação linear
        let ratio = (value - v1) / (v2 - v1);

        [
            (c1[0] as f64 + ratio * (c2[0] as f64 - c1[0] as f64)) as u8,
            (c1[1] as f64 + ratio * (c2[1] as f64 - c1[1] as f64)) as u8,
            (c1[2] as f64 + ratio * (c2[2] as f64 - c1[2] as f64)) as u8,
            (c1[3] as f64 + ratio * (c2[3] as f64 - c1[3] as f64)) as u8,
        ]
    }

    /// Obtém cor para valor em um range específico
    pub fn color_for_value(&self, value: f64, min: f64, max: f64) -> [u8; 4] {
        if max <= min {
            return self.colors[0];
        }

        let normalized = (value - min) / (max - min);
        self.interpolate(normalized)
    }
}

/// Método de classificação de dados
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassificationMethod {
    /// Intervalos iguais
    EqualInterval,

    /// Quantis (mesma quantidade de dados em cada classe)
    Quantile,

    /// Quebras naturais (Jenks)
    NaturalBreaks,

    /// Desvio padrão
    StandardDeviation,

    /// Manual (definido pelo usuário)
    Manual,
}

/// Classificador de dados
pub struct Classifier {
    pub method: ClassificationMethod,
    pub num_classes: usize,
    pub breaks: Vec<f64>,
}

impl Classifier {
    /// Cria classificador com intervalos iguais
    pub fn equal_interval(min: f64, max: f64, num_classes: usize) -> Self {
        let step = (max - min) / num_classes as f64;
        let mut breaks = vec![min];

        for i in 1..num_classes {
            breaks.push(min + i as f64 * step);
        }

        breaks.push(max);

        Self {
            method: ClassificationMethod::EqualInterval,
            num_classes,
            breaks,
        }
    }

    /// Cria classificador com quantis
    pub fn quantile(mut values: Vec<f64>, num_classes: usize) -> Self {
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut breaks = Vec::new();
        breaks.push(values[0]);

        let step = values.len() / num_classes;

        for i in 1..num_classes {
            let idx = (i * step).min(values.len() - 1);
            breaks.push(values[idx]);
        }

        breaks.push(values[values.len() - 1]);

        Self {
            method: ClassificationMethod::Quantile,
            num_classes,
            breaks,
        }
    }

    /// Cria classificador manual
    pub fn manual(breaks: Vec<f64>) -> Self {
        let num_classes = breaks.len().saturating_sub(1);

        Self {
            method: ClassificationMethod::Manual,
            num_classes,
            breaks,
        }
    }

    /// Determina a classe de um valor
    pub fn classify(&self, value: f64) -> usize {
        for i in 0..self.breaks.len() - 1 {
            if value >= self.breaks[i] && value < self.breaks[i + 1] {
                return i;
            }
        }

        // Último intervalo é fechado
        if value == self.breaks[self.breaks.len() - 1] {
            return self.num_classes - 1;
        }

        0
    }

    /// Obtém descrição da classe
    pub fn class_label(&self, class: usize) -> String {
        if class >= self.num_classes {
            return "Fora do range".to_string();
        }

        format!("{:.2} - {:.2}", self.breaks[class], self.breaks[class + 1])
    }
}

/// Símbolo para categoria específica
#[derive(Debug, Clone)]
pub struct CategorySymbol {
    pub category: String,
    pub symbol: SymbolStyle,
    pub label: String,
}

/// Simbologia categórica (para dados qualitativos)
pub struct CategoricalSymbology {
    pub categories: Vec<CategorySymbol>,
    pub default_symbol: SymbolStyle,
}

impl CategoricalSymbology {
    pub fn new() -> Self {
        Self {
            categories: Vec::new(),
            default_symbol: SymbolStyle::default(),
        }
    }

    pub fn add_category(&mut self, category: String, symbol: SymbolStyle, label: String) {
        self.categories.push(CategorySymbol {
            category,
            symbol,
            label,
        });
    }

    pub fn get_symbol(&self, category: &str) -> &SymbolStyle {
        self.categories
            .iter()
            .find(|c| c.category == category)
            .map(|c| &c.symbol)
            .unwrap_or(&self.default_symbol)
    }
}

impl Default for CategoricalSymbology {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_ramp_interpolation() {
        let ramp = ColorRamp::two_color([255, 0, 0, 255], [0, 0, 255, 255]);

        let mid = ramp.interpolate(0.5);
        assert_eq!(mid[0], 127); // Meio termo entre 255 e 0
    }

    #[test]
    fn test_classifier() {
        let classifier = Classifier::equal_interval(0.0, 100.0, 5);

        assert_eq!(classifier.classify(10.0), 0);
        assert_eq!(classifier.classify(50.0), 2);
        assert_eq!(classifier.classify(90.0), 4);
    }

    #[test]
    fn test_categorical_symbology() {
        let mut symbology = CategoricalSymbology::new();

        symbology.add_category(
            "Restaurante".to_string(),
            SymbolStyle::simple([255, 0, 0, 255], 10.0),
            "Restaurantes".to_string(),
        );

        let symbol = symbology.get_symbol("Restaurante");
        assert_eq!(symbol.fill_color, [255, 0, 0, 255]);
    }
}
